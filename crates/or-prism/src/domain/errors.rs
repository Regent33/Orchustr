use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum PrismError {
    #[error("invalid OTLP endpoint: {0}")]
    InvalidEndpoint(String),
    #[error("OTLP exporter error: {0}")]
    Exporter(String),
    #[error("dashboard initialization error: {0}")]
    Lens(String),
    #[error("subscriber install error: {0}")]
    Subscriber(String),
}
