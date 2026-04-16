use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum BeaconError {
    #[error("template is required")]
    MissingTemplate,
    #[error("invalid template: {0}")]
    InvalidTemplate(String),
    #[error("missing template variable: {0}")]
    MissingVariable(String),
    #[error("invalid context: {0}")]
    InvalidContext(String),
}
