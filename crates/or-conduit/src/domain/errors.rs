use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum ConduitError {
    #[error("missing environment variable: {0}")]
    MissingEnvironmentVariable(String),
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("HTTP request failed: {0}")]
    Http(String),
    #[error("API error: status={status}, body={body}")]
    Api { status: u16, body: String },
    #[error("token budget exceeded: requested={requested}, budget={budget}")]
    BudgetExceeded { requested: u32, budget: u32 },
    #[error("rate limited - retry after {retry_after_ms}ms")]
    RateLimited { retry_after_ms: u64 },
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("not implemented: {0}")]
    NotImplemented(String),
    #[error("request timed out")]
    Timeout,
    #[error("authentication failed: {0}")]
    AuthenticationFailed(String),
}

impl From<reqwest::Error> for ConduitError {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error.to_string())
    }
}
