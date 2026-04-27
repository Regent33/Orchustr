use or_tools_core::ToolError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum VectorError {
    #[error("missing credential `{0}`")]
    MissingCredential(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: u32, actual: u32 },

    #[error("collection `{0}` not found")]
    CollectionNotFound(String),

    #[error("upstream `{provider}` returned {status}: {body}")]
    Upstream {
        provider: String,
        status: u16,
        body: String,
    },

    #[error("transport error for `{provider}`: {reason}")]
    Transport { provider: String, reason: String },

    #[error("serialization error for `{provider}`: {reason}")]
    Serialization { provider: String, reason: String },
}

impl From<VectorError> for ToolError {
    fn from(err: VectorError) -> Self {
        match err {
            VectorError::MissingCredential(env_var) => ToolError::MissingCredential {
                tool: "vector".into(),
                env_var,
            },
            VectorError::InvalidInput(r) => ToolError::invalid_input("vector", r),
            VectorError::DimensionMismatch { expected, actual } => ToolError::invalid_input(
                "vector",
                format!("dimension mismatch: expected {expected}, got {actual}"),
            ),
            VectorError::CollectionNotFound(c) => {
                ToolError::invalid_input("vector", format!("collection `{c}` not found"))
            }
            VectorError::Upstream {
                provider,
                status,
                body,
            } => ToolError::Upstream {
                tool: provider,
                status,
                body,
            },
            VectorError::Transport { provider, reason } => ToolError::Transport {
                tool: provider,
                reason,
            },
            VectorError::Serialization { provider, reason } => ToolError::Serialization {
                tool: provider,
                reason,
            },
        }
    }
}
