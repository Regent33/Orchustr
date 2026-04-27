use thiserror::Error;

/// Errors returned by the `or-cli` project scaffolding and validation flows.
#[derive(Debug, Error)]
pub enum CliError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("graph schema error: {0}")]
    Schema(#[from] or_schema::SchemaError),
    #[error("project config error: {0}")]
    Config(String),
    #[error("project validation error: {0}")]
    Validation(String),
    #[error("invalid project: {0}")]
    InvalidProject(String),
    /// Wraps a typed `or_lens::LensError` so callers can match on the
    /// underlying dashboard failure (bind vs serve) instead of pattern
    /// matching on a stringified message.
    #[error("dashboard error: {0}")]
    Lens(#[from] or_lens::LensError),
}
