use crate::domain::errors::PrismError;
use crate::infra::adapters::prism_config;
use crate::infra::implementations::install;

pub fn install_global_subscriber(otlp_endpoint: &str) -> Result<(), PrismError> {
    let span = tracing::info_span!(
        "prism.install_global_subscriber",
        otel.name = "prism.install_global_subscriber",
        status = tracing::field::Empty
    );
    let _guard = span.enter();
    let result = install(&prism_config(otlp_endpoint)?);
    span.record("status", if result.is_ok() { "success" } else { "failure" });
    result
}
