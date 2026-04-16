use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum PipelineError {
    #[error("pipeline must contain at least one node")]
    EmptyPipeline,
    #[error("pipeline node name must not be blank")]
    BlankNodeName,
    #[error("duplicate pipeline node: {0}")]
    DuplicateNode(String),
    #[error("node execution failed: {0}")]
    NodeExecution(String),
}
