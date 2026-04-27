//! Command-line scaffolding and validation helpers for Orchustr projects.

pub mod application;
pub mod domain;
pub mod infra;

pub use application::orchestrators::{
    DefaultProjectRunner, ProjectRunner, init_project, lint_path, run_project, scaffold_node,
    scaffold_topology, trace_project,
};
pub use domain::entities::{InitOptions, ProjectLanguage, ProviderKind, RunSummary, TopologyKind};
pub use domain::errors::CliError;
