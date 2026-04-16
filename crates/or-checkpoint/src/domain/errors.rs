use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum CheckpointError {
    #[error("checkpoint storage failed: {0}")]
    Storage(String),
    #[error("checkpoint serialization failed: {0}")]
    Serialization(String),
    #[error("checkpoint not found: {0}")]
    MissingCheckpoint(String),
}
