use crate::domain::entities::PrismConfig;
use crate::domain::errors::PrismError;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_otlp::WithHttpConfig;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tokio::runtime::Handle;
use tracing_subscriber::prelude::*;

pub(crate) fn install(config: &PrismConfig) -> Result<(), PrismError> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_http_client(reqwest_otel::Client::new())
        .with_endpoint(config.otlp_endpoint.clone())
        .build()
        .map_err(|error| PrismError::Exporter(error.to_string()))?;
    let provider = if Handle::try_current().is_ok() {
        SdkTracerProvider::builder()
            .with_batch_exporter(exporter)
            .build()
    } else {
        SdkTracerProvider::builder()
            .with_simple_exporter(exporter)
            .build()
    };
    let tracer = provider.tracer(config.service_name.clone());
    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .with(tracing_opentelemetry::layer().with_tracer(tracer));
    subscriber
        .try_init()
        .map_err(|error| PrismError::Subscriber(error.to_string()))?;
    opentelemetry::global::set_tracer_provider(provider);
    Ok(())
}
