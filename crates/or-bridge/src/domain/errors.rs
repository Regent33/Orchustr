use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum BridgeError {
    #[error("invalid bridge input: {0}")]
    InvalidInput(String),
    #[error("invalid JSON payload: {0}")]
    InvalidJson(String),
    #[error("unsupported crate binding: {0}")]
    UnsupportedCrate(String),
    #[error("unsupported operation `{operation}` for crate `{crate_name}`")]
    UnsupportedOperation { crate_name: String, operation: String },
    #[error("bridge invocation failed for `{crate_name}` / `{operation}`: {reason}")]
    Invocation {
        crate_name: String,
        operation: String,
        reason: String,
    },
    #[error("state must serialize to a JSON object")]
    InvalidState,
    #[error("prompt render failed: {0}")]
    Prompt(String),
}
