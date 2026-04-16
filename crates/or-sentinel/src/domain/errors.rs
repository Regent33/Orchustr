use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum SentinelError {
    #[error("messages are missing from state")]
    MissingMessages,
    #[error("invalid state: {0}")]
    InvalidState(String),
    #[error("invalid model response: {0}")]
    InvalidResponse(String),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("conduit error: {0}")]
    Conduit(String),
    #[error("forge error: {0}")]
    Forge(String),
    #[error("graph error: {0}")]
    Loom(String),
    #[error("core error: {0}")]
    Core(String),
}
