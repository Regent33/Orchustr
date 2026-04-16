use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum ColonyError {
    #[error("colony must contain at least one member")]
    EmptyColony,
    #[error("duplicate colony member: {0}")]
    DuplicateMember(String),
    #[error("task missing from state")]
    MissingTask,
    #[error("invalid state: {0}")]
    InvalidState(String),
    #[error("serialization error: {0}")]
    Serialization(String),
}
