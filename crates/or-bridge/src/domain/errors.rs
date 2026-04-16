use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum BridgeError {
    #[error("invalid bridge input: {0}")]
    InvalidInput(String),
    #[error("invalid JSON payload: {0}")]
    InvalidJson(String),
    #[error("state must serialize to a JSON object")]
    InvalidState,
    #[error("prompt render failed: {0}")]
    Prompt(String),
}
