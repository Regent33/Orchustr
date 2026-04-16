use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum SieveError {
    #[error("invalid json: {0}")]
    InvalidJson(String),
    #[error("schema violation at {path}: {message}")]
    SchemaViolation { path: String, message: String },
    #[error("deserialization failed: {0}")]
    Deserialization(String),
    #[error("text output must not be empty")]
    EmptyText,
}
