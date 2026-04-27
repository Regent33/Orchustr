use clap::{Parser, Subcommand, ValueEnum};
use or_cli::{
    CliError, DefaultProjectRunner, InitOptions, ProjectLanguage, ProviderKind, TopologyKind,
    init_project, lint_path, run_project, scaffold_node, scaffold_topology, trace_project,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "orchustr")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init(InitArgs),
    Run {
        project_dir: PathBuf,
    },
    Lint {
        project_dir: PathBuf,
    },
    Trace {
        project_dir: PathBuf,
    },
    New {
        #[command(subcommand)]
        command: NewArgs,
    },
}

#[derive(clap::Args)]
struct InitArgs {
    project_name: String,
    #[arg(long, value_enum, default_value_t = LanguageArg::Rust)]
    lang: LanguageArg,
    #[arg(long, value_enum, default_value_t = TopologyArg::React)]
    topology: TopologyArg,
    #[arg(long, value_enum, default_value_t = ProviderArg::Anthropic)]
    provider: ProviderArg,
}

#[derive(Subcommand)]
enum NewArgs {
    Node { name: String, project_dir: PathBuf },
    Topology { name: String, project_dir: PathBuf },
}

#[derive(Clone, Copy, ValueEnum)]
enum LanguageArg {
    Rust,
    Python,
    Typescript,
    Dart,
}

#[derive(Clone, Copy, ValueEnum)]
enum TopologyArg {
    React,
    PlanExecute,
    Reflection,
}

#[derive(Clone, Copy, ValueEnum)]
enum ProviderArg {
    Anthropic,
    Openai,
    Ollama,
}

impl From<LanguageArg> for ProjectLanguage {
    fn from(value: LanguageArg) -> Self {
        match value {
            LanguageArg::Rust => Self::Rust,
            LanguageArg::Python => Self::Python,
            LanguageArg::Typescript => Self::Typescript,
            LanguageArg::Dart => Self::Dart,
        }
    }
}

impl From<TopologyArg> for TopologyKind {
    fn from(value: TopologyArg) -> Self {
        match value {
            TopologyArg::React => Self::React,
            TopologyArg::PlanExecute => Self::PlanExecute,
            TopologyArg::Reflection => Self::Reflection,
        }
    }
}

impl From<ProviderArg> for ProviderKind {
    fn from(value: ProviderArg) -> Self {
        match value {
            ProviderArg::Anthropic => Self::Anthropic,
            ProviderArg::Openai => Self::Openai,
            ProviderArg::Ollama => Self::Ollama,
        }
    }
}

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        // Render the user-facing `Display` form rather than letting `main`
        // return `Err`, which would print the `Debug` representation.
        eprintln!("orchustr: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), CliError> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init(args) => {
            init_project(&InitOptions {
                project_name: args.project_name,
                language: args.lang.into(),
                topology: args.topology.into(),
                provider: args.provider.into(),
                target_dir: std::env::current_dir()?,
            })?;
        }
        Command::Run { project_dir } => {
            let summary = run_project(&project_dir, &DefaultProjectRunner).await?;
            println!("validated project: {}", summary.project_name);
            if let Some(port) = summary.dashboard_port {
                println!("dashboard port (configured): {port}");
            }
        }
        Command::Lint { project_dir } => {
            let validated = lint_path(&project_dir)?;
            for path in validated {
                println!("ok: {}", path.display());
            }
        }
        Command::Trace { project_dir } => {
            let handle = trace_project(&project_dir).await?;
            println!("dashboard listening on http://127.0.0.1:{}", handle.port());
            println!("press Ctrl-C to stop");
            // Keep the dashboard alive until the user interrupts.
            let _ = tokio::signal::ctrl_c().await;
            handle.shutdown();
        }
        Command::New {
            command: NewArgs::Node { name, project_dir },
        } => {
            scaffold_node(&project_dir, &name)?;
        }
        Command::New {
            command: NewArgs::Topology { name, project_dir },
        } => {
            scaffold_topology(&project_dir, &name)?;
        }
    }
    Ok(())
}
