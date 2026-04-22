# or-cli API Reference

This page documents the main public surface exported by `crates/or-cli/src/lib.rs`.

## Types

- `InitOptions`: user-facing inputs for `orchustr init`.
- `ProjectLanguage`: supported scaffold languages: Rust, Python, TypeScript, and Dart.
- `TopologyKind`: supported scaffold topology presets: ReAct, Plan-Execute, Reflection.
- `ProviderKind`: supported scaffold provider presets: Anthropic, OpenAI, Ollama.
- `RunSummary`: returned metadata from `run_project`.
- `CliError`: error type for scaffolding, linting, and trace bootstrapping flows.

## Traits and Structs

- `ProjectRunner`: runtime hook that receives a parsed `RunRequest`.
- `DefaultProjectRunner`: default no-op runner used by the `orchustr` binary after validation succeeds.

## Functions

```rust
pub fn init_project(options: &InitOptions) -> Result<PathBuf, CliError>
pub fn lint_path(path: &Path) -> Result<Vec<PathBuf>, CliError>
pub async fn run_project<R: ProjectRunner>(project_dir: &Path, runner: &R) -> Result<RunSummary, CliError>
pub async fn trace_project(project_dir: &Path) -> Result<u16, CliError>
pub fn scaffold_node(project_dir: &Path, name: &str) -> Result<PathBuf, CliError>
pub fn scaffold_topology(project_dir: &Path, name: &str) -> Result<PathBuf, CliError>
```

## Notes

- `lint_path` accepts either a single graph file, a project directory containing `orchustr.yaml`, or a directory of graph examples.
- `trace_project` currently verifies that `or-lens` can bind and start for the configured port, then shuts the dashboard down.
