use or_tools_core::ToolError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum FileError {
    #[error("not found: `{0}`")]
    NotFound(String),
    #[error("permission denied: `{0}`")]
    PermissionDenied(String),
    #[error("io error for `{path}`: {reason}")]
    Io { path: String, reason: String },
    #[error("json error: {0}")]
    Json(String),
    #[error("missing credential `{0}`")]
    MissingCredential(String),
    #[error("upstream `{provider}` {status}: {body}")]
    Upstream {
        provider: String,
        status: u16,
        body: String,
    },
    #[error("transport: {0}")]
    Transport(String),
}

impl From<FileError> for ToolError {
    fn from(err: FileError) -> Self {
        match err {
            FileError::NotFound(p) => ToolError::invalid_input("file", format!("not found: {p}")),
            FileError::PermissionDenied(p) => {
                ToolError::invalid_input("file", format!("denied: {p}"))
            }
            FileError::Io { path, reason } => ToolError::Transport {
                tool: "file".into(),
                reason: format!("{path}: {reason}"),
            },
            FileError::Json(r) => ToolError::Serialization {
                tool: "file".into(),
                reason: r,
            },
            FileError::MissingCredential(env_var) => ToolError::MissingCredential {
                tool: "file".into(),
                env_var,
            },
            FileError::Upstream {
                provider,
                status,
                body,
            } => ToolError::Upstream {
                tool: provider,
                status,
                body,
            },
            FileError::Transport(r) => ToolError::Transport {
                tool: "file".into(),
                reason: r,
            },
        }
    }
}
