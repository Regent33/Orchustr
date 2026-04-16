use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum RecallError {
    #[error("recall storage failed: {0}")]
    Storage(String),
    #[error("recall serialization failed: {0}")]
    Serialization(String),
}
