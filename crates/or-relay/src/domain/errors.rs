use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum RelayError {
    #[error("relay plan must contain at least one branch")]
    EmptyPlan,
    #[error("relay branch name must not be blank")]
    BlankBranchName,
    #[error("duplicate relay branch: {0}")]
    DuplicateBranch(String),
    #[error("branch execution failed: {0}")]
    BranchExecution(String),
}
