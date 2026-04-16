use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum LoomError {
    #[error("graph must contain at least one node")]
    EmptyGraph,
    #[error("graph node name must not be blank")]
    BlankNodeName,
    #[error("duplicate graph node: {0}")]
    DuplicateNode(String),
    #[error("entry node not set")]
    MissingEntry,
    #[error("exit node not set")]
    MissingExit,
    #[error("unknown node: {0}")]
    UnknownNode(String),
    #[error("edge references unknown node: {0}")]
    EdgeReferencesUnknownNode(String),
    #[error("node has no outgoing edge: {0}")]
    NoEdgeFromNode(String),
    #[error("node has multiple outgoing edges and requires an explicit branch: {0}")]
    AmbiguousNextNode(String),
    #[error("invalid branch target from {from} to {to}")]
    InvalidBranchTarget { from: String, to: String },
    #[error("graph paused at checkpoint: {checkpoint_id}")]
    Paused { checkpoint_id: String },
    #[error("graph exceeded execution limit: {max_steps}")]
    StepLimitExceeded { max_steps: usize },
    #[error("node execution failed in {node}: {message}")]
    NodeExecution { node: String, message: String },
}
