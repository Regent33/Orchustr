use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum ForgeError {
    #[error("duplicate tool: {0}")]
    DuplicateTool(String),
    #[error("unknown tool: {0}")]
    UnknownTool(String),
    #[error("tool input validation failed: {0}")]
    InvalidArguments(String),
    #[error("tool invocation failed: {0}")]
    Invocation(String),
}
