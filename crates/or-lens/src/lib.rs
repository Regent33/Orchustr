//! Local execution timeline dashboard support for Orchustr.

#[cfg(feature = "dashboard")]
pub mod application;
#[cfg(feature = "dashboard")]
pub mod domain;
#[cfg(feature = "dashboard")]
pub mod collector;
#[cfg(feature = "dashboard")]
pub mod infra;
#[cfg(feature = "dashboard")]
pub mod server;
#[cfg(feature = "dashboard")]
pub mod snapshot;
#[cfg(feature = "dashboard")]
pub mod tracing_layer;

#[cfg(feature = "dashboard")]
pub use collector::{LensSpan, LensSpanStatus, SpanCollector, TraceSummary};
#[cfg(feature = "dashboard")]
pub use server::{LensError, LensHandle, start_dashboard_server};
#[cfg(feature = "dashboard")]
pub use snapshot::{ExecutionNodeSnapshot, ExecutionSnapshot};
#[cfg(feature = "dashboard")]
pub use tracing_layer::LensLayer;
