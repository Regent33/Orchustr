use std::error::Error;
use std::fmt::{Display, Formatter};

/// Errors returned while starting or serving the `or-lens` dashboard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LensError {
    /// Binding the dashboard server socket failed.
    Bind(String),
    /// Serving the dashboard HTTP application failed.
    Serve(String),
}

impl Display for LensError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bind(message) => write!(f, "or-lens bind error: {message}"),
            Self::Serve(message) => write!(f, "or-lens serve error: {message}"),
        }
    }
}

impl Error for LensError {}
