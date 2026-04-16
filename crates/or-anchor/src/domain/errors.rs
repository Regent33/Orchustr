use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum AnchorError {
    #[error("vector store operation failed: {0}")]
    VectorStore(String),
}
