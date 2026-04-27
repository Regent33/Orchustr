use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProductivityError {
    #[error("missing credential env var: {0}")]
    MissingCredential(String),
    #[error("transport error: {0}")]
    Transport(String),
    #[error("upstream error from {provider}: HTTP {status}")]
    Upstream {
        provider: String,
        status: u16,
        body: String,
    },
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("not found: {0}")]
    NotFound(String),
}
