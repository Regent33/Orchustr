use or_core::CoreError;
use or_loom::LoomError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// `Eq` is intentionally not derived: `Loom(LoomError)` carries
// `serde_json::Value` (via `LoomError::Paused`), which only implements
// `PartialEq`. `assert_eq!` and `==` still work because `PartialEq`
// is what those macros require.
#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq)]
pub enum SentinelError {
    #[error("messages are missing from state")]
    MissingMessages,
    #[error("invalid state: {0}")]
    InvalidState(String),
    #[error("invalid model response: {0}")]
    InvalidResponse(String),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("conduit error: {0}")]
    Conduit(String),
    #[error("forge error: {0}")]
    Forge(String),
    /// Wraps a typed `or_loom::LoomError` so callers can match on the
    /// underlying graph failure (paused checkpoint, step-limit, unbound
    /// node, etc.) instead of pattern-matching on a stringified message.
    #[error("graph error: {0}")]
    Loom(#[from] LoomError),
    /// Wraps a typed `or_core::CoreError` so callers can match on the
    /// underlying budget/retry failure cause.
    #[error("core error: {0}")]
    Core(#[from] CoreError),
}
