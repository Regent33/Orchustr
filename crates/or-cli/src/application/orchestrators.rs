use crate::domain::entities::{
    InitOptions, OrchustrConfig, ProjectLanguage, RunRequest, RunSummary,
};
use crate::domain::errors::CliError;
use crate::infra::templates::{
    render_common_files, render_dart_files, render_python_files, render_rust_files,
    render_typescript_files,
};
use async_trait::async_trait;
use or_lens::{LensHandle, start_dashboard_server};
use or_schema::GraphSpec;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;

/// Runtime hook used by `or-cli` to hand parsed project config to an executor.
#[async_trait]
pub trait ProjectRunner: Send + Sync {
    /// Executes a parsed Orchustr project request.
    async fn run(&self, request: RunRequest) -> Result<(), CliError>;
}

/// Default project runner used by the `orchustr` binary.
///
/// Detects the language declared in `orchustr.yaml` and shells out to the
/// canonical entrypoint for that language (e.g. `cargo run` for Rust,
/// `python main.py` for Python). Inherits stdio so the user sees the
/// child process's output. If no recognised entrypoint is present in
/// `project_dir`, returns `CliError::InvalidProject` with a hint.
pub struct DefaultProjectRunner;

#[async_trait]
impl ProjectRunner for DefaultProjectRunner {
    async fn run(&self, request: RunRequest) -> Result<(), CliError> {
        let plan = launch_plan(&request)?;
        let mut command = Command::new(&plan.program);
        command
            .args(&plan.args)
            .current_dir(&request.project_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .kill_on_drop(true);
        let status = command.status().await.map_err(|error| {
            CliError::InvalidProject(format!(
                "failed to launch `{}`: {error}. Is the toolchain installed and on PATH?",
                plan.program
            ))
        })?;
        if !status.success() {
            return Err(CliError::InvalidProject(format!(
                "`{}` exited with status {}",
                plan.program,
                status
                    .code()
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "<signal>".to_owned())
            )));
        }
        Ok(())
    }
}

struct LaunchPlan {
    program: String,
    args: Vec<String>,
}

fn launch_plan(request: &RunRequest) -> Result<LaunchPlan, CliError> {
    let project_dir = &request.project_dir;
    match request.config.project.language {
        ProjectLanguage::Rust => {
            if project_dir.join("Cargo.toml").is_file() {
                Ok(LaunchPlan {
                    program: "cargo".to_owned(),
                    args: vec!["run".to_owned()],
                })
            } else {
                Err(CliError::InvalidProject(format!(
                    "no Cargo.toml found in {}",
                    project_dir.display()
                )))
            }
        }
        ProjectLanguage::Python => {
            for candidate in ["main.py", "agent.py", "app.py"] {
                if project_dir.join(candidate).is_file() {
                    return Ok(LaunchPlan {
                        program: python_program(),
                        args: vec![candidate.to_owned()],
                    });
                }
            }
            Err(CliError::InvalidProject(format!(
                "no Python entrypoint (main.py / agent.py / app.py) in {}",
                project_dir.display()
            )))
        }
        ProjectLanguage::Typescript => {
            if project_dir.join("package.json").is_file() {
                Ok(LaunchPlan {
                    program: npm_program(),
                    args: vec!["start".to_owned()],
                })
            } else if project_dir.join("src/index.ts").is_file() {
                Ok(LaunchPlan {
                    program: "npx".to_owned(),
                    args: vec!["tsx".to_owned(), "src/index.ts".to_owned()],
                })
            } else {
                Err(CliError::InvalidProject(format!(
                    "no package.json or src/index.ts in {}",
                    project_dir.display()
                )))
            }
        }
        ProjectLanguage::Dart => {
            if project_dir.join("pubspec.yaml").is_file() {
                Ok(LaunchPlan {
                    program: "dart".to_owned(),
                    args: vec!["run".to_owned()],
                })
            } else {
                Err(CliError::InvalidProject(format!(
                    "no pubspec.yaml in {}",
                    project_dir.display()
                )))
            }
        }
    }
}

fn python_program() -> String {
    // Prefer `python` then fall back to `python3`. Both are commonly on
    // PATH; the actual selection is left to the OS resolver.
    if cfg!(target_os = "windows") {
        "python".to_owned()
    } else {
        "python3".to_owned()
    }
}

fn npm_program() -> String {
    if cfg!(target_os = "windows") {
        // On Windows, `npm` ships as a .cmd shim that requires invoking via
        // the shell wrapper to be resolved by `Command`.
        "npm.cmd".to_owned()
    } else {
        "npm".to_owned()
    }
}

/// Creates a new Orchustr project scaffold from the provided options.
pub fn init_project(options: &InitOptions) -> Result<PathBuf, CliError> {
    let root = options.project_root();
    if root.exists() {
        return Err(CliError::InvalidProject(format!(
            "project directory already exists: {}",
            root.display()
        )));
    }

    fs::create_dir_all(&root)?;
    for (relative, contents) in render_common_files(options)? {
        write_file(&root.join(relative), &contents)?;
    }
    for (relative, contents) in language_files(options)? {
        write_file(&root.join(relative), &contents)?;
    }
    Ok(root)
}

/// Validates an Orchustr project directory or a directory of graph examples.
pub fn lint_path(path: &Path) -> Result<Vec<PathBuf>, CliError> {
    if path.is_file() {
        return Ok(vec![validate_graph_file(path)?]);
    }
    let config_path = path.join("orchustr.yaml");
    if config_path.exists() {
        let config = load_config(path)?;
        let graph_path = config.graph_path(path);
        validate_graph_file(&graph_path)?;
        return Ok(vec![graph_path]);
    }

    let mut validated = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file = entry.path();
        let is_yaml = file
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| matches!(value, "yaml" | "yml"));
        if is_yaml {
            validated.push(validate_graph_file(&file)?);
        }
    }
    if validated.is_empty() {
        return Err(CliError::InvalidProject(format!(
            "no graph yaml files found in {}",
            path.display()
        )));
    }
    Ok(validated)
}

/// Loads an Orchustr project and hands it to the provided runner implementation.
pub async fn run_project<R: ProjectRunner>(
    project_dir: &Path,
    runner: &R,
) -> Result<RunSummary, CliError> {
    let config = load_config(project_dir)?;
    let graph = load_graph(&config, project_dir)?;
    let request = RunRequest {
        project_dir: project_dir.to_path_buf(),
        config: config.clone(),
        graph,
    };
    runner.run(request).await?;
    Ok(RunSummary {
        project_name: config.project.name,
        dashboard_port: config
            .observability
            .enabled
            .then_some(config.observability.dashboard_port),
    })
}

/// Starts the local `or-lens` dashboard configured by an Orchustr project.
///
/// Returns the live `LensHandle` so the caller controls the server lifetime.
/// The previous version of this function shut the server down before
/// returning, which made the dashboard unusable for `orchustr trace`.
pub async fn trace_project(project_dir: &Path) -> Result<LensHandle, CliError> {
    let config = load_config(project_dir)?;
    let port = config.observability.dashboard_port;
    start_dashboard_server(port).await.map_err(CliError::from)
}

/// Scaffolds a node file inside an existing Orchustr project.
pub fn scaffold_node(project_dir: &Path, name: &str) -> Result<PathBuf, CliError> {
    let config = load_config(project_dir)?;
    let file_name = format!("{name}.{}", config.project.language.file_extension());
    let path = project_dir.join("nodes").join(file_name);
    let contents = config.project.language.node_template(name);
    write_file(&path, &contents)?;
    Ok(path)
}

/// Scaffolds a Rust `LoopTopology` stub inside an existing Orchustr project.
pub fn scaffold_topology(project_dir: &Path, name: &str) -> Result<PathBuf, CliError> {
    let path = project_dir.join(format!("{name}_topology.rs"));
    let contents = format!(
        "use or_core::DynState;\nuse or_loom::GraphBuilder;\nuse or_sentinel::LoopTopology;\n\n\
         /// Custom topology scaffold generated by `or-cli`.\n\
         pub struct {name}Topology;\n\n\
         impl LoopTopology for {name}Topology {{\n\
             fn build(&self) -> GraphBuilder<DynState> {{\n\
                 GraphBuilder::new()\n\
             }}\n\n\
             fn name(&self) -> &'static str {{\n\
                 \"{name}\"\n\
             }}\n\
         }}\n"
    );
    write_file(&path, &contents)?;
    Ok(path)
}

fn language_files(options: &InitOptions) -> Result<Vec<(PathBuf, String)>, CliError> {
    match options.language {
        ProjectLanguage::Rust => render_rust_files(options),
        ProjectLanguage::Python => render_python_files(options),
        ProjectLanguage::Typescript => render_typescript_files(options),
        ProjectLanguage::Dart => render_dart_files(options),
    }
}

fn load_config(project_dir: &Path) -> Result<OrchustrConfig, CliError> {
    let raw = fs::read_to_string(project_dir.join("orchustr.yaml"))?;
    serde_yaml::from_str(&raw).map_err(|error| CliError::Config(error.to_string()))
}

fn load_graph(config: &OrchustrConfig, project_dir: &Path) -> Result<GraphSpec, CliError> {
    let graph_path = config.graph_path(project_dir);
    let raw = fs::read_to_string(&graph_path)?;
    let graph = if graph_path
        .extension()
        .and_then(|value| value.to_str())
        .is_some_and(|value| value.eq_ignore_ascii_case("json"))
    {
        GraphSpec::from_json(&raw)?
    } else {
        GraphSpec::from_yaml(&raw)?
    };
    validate_graph_spec(&graph)?;
    Ok(graph)
}

fn validate_graph_file(path: &Path) -> Result<PathBuf, CliError> {
    let raw = fs::read_to_string(path)?;
    let graph = if path
        .extension()
        .and_then(|value| value.to_str())
        .is_some_and(|value| value.eq_ignore_ascii_case("json"))
    {
        GraphSpec::from_json(&raw)?
    } else {
        GraphSpec::from_yaml(&raw)?
    };
    validate_graph_spec(&graph)?;
    Ok(path.to_path_buf())
}

fn validate_graph_spec(graph: &GraphSpec) -> Result<(), CliError> {
    let nodes = graph
        .nodes
        .iter()
        .map(|node| node.id.clone())
        .collect::<HashSet<_>>();
    if !nodes.contains(&graph.entry) {
        return Err(CliError::Validation(format!(
            "entry node '{}' is not declared",
            graph.entry
        )));
    }
    if graph.exits.is_empty() {
        return Err(CliError::Validation(
            "graph must declare at least one exit node".to_owned(),
        ));
    }
    for exit in &graph.exits {
        if !nodes.contains(exit) {
            return Err(CliError::Validation(format!(
                "exit node '{}' is not declared",
                exit
            )));
        }
    }
    for edge in &graph.edges {
        if !nodes.contains(&edge.from) || !nodes.contains(&edge.to) {
            return Err(CliError::Validation(format!(
                "edge '{}' -> '{}' references an unknown node",
                edge.from, edge.to
            )));
        }
    }
    Ok(())
}

fn write_file(path: &Path, contents: &str) -> Result<(), CliError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}
