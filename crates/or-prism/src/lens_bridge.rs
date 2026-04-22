use crate::domain::errors::PrismError;
use or_lens::{LensHandle, LensLayer, start_dashboard_server};
use tracing_subscriber::prelude::*;

/// Initialises `or-prism` with a local `or-lens` dashboard server on the given port.
#[cfg(feature = "lens")]
pub async fn init_with_dashboard(port: u16) -> Result<LensHandle, PrismError> {
    let handle = start_dashboard_server(port)
        .await
        .map_err(|error| PrismError::Lens(error.to_string()))?;
    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .with(LensLayer::new(handle.collector()));
    subscriber
        .try_init()
        .map_err(|error| PrismError::Subscriber(error.to_string()))?;
    Ok(handle)
}
