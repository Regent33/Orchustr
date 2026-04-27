use or_tools_core::ToolError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum ExecError {
    #[error("unsupported language `{0}`")]
    UnsupportedLanguage(String),

    #[error("executor `{executor}` not found: {reason}")]
    ExecutorNotFound { executor: String, reason: String },

    #[error("missing credential `{0}`")]
    MissingCredential(String),

    #[error("execution timed out after {0}ms")]
    Timeout(u64),

    #[error("spawn error: {0}")]
    Spawn(String),

    #[error("upstream `{provider}` error {status}: {body}")]
    Upstream {
        provider: String,
        status: u16,
        body: String,
    },

    #[error("transport error: {0}")]
    Transport(String),

    #[error("io error: {0}")]
    Io(String),
}

impl From<ExecError> for ToolError {
    fn from(err: ExecError) -> Self {
        match err {
            ExecError::UnsupportedLanguage(l) => {
                ToolError::invalid_input("exec", format!("unsupported language: {l}"))
            }
            ExecError::MissingCredential(env_var) => ToolError::MissingCredential {
                tool: "exec".into(),
                env_var,
            },
            ExecError::Timeout(ms) => ToolError::Timeout {
                tool: "exec".into(),
                timeout_ms: ms,
            },
            ExecError::Spawn(r) => ToolError::Unavailable {
                tool: "exec".into(),
                reason: r,
            },
            ExecError::Upstream {
                provider,
                status,
                body,
            } => ToolError::Upstream {
                tool: provider,
                status,
                body,
            },
            ExecError::Transport(r) => ToolError::Transport {
                tool: "exec".into(),
                reason: r,
            },
            ExecError::Io(r) => ToolError::Transport {
                tool: "exec".into(),
                reason: r,
            },
            ExecError::ExecutorNotFound { executor, reason } => ToolError::Unavailable {
                tool: executor,
                reason,
            },
        }
    }
}
