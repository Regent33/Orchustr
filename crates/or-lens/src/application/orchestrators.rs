use crate::domain::entities::{ExecutionNodeSnapshot, ExecutionSnapshot, LensSpan};
use crate::domain::errors::LensError;
use crate::infra::http::router;
use crate::infra::repositories::SpanCollector;
use axum::Router;
use serde_json::{Map, Value, json};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Notify;

/// Handle returned by the `or-lens` dashboard server startup path.
#[derive(Clone)]
pub struct LensHandle {
    port: u16,
    collector: SpanCollector,
    shutdown: Arc<Notify>,
}

impl LensHandle {
    /// Returns the local port bound by the `or-lens` dashboard server.
    #[must_use]
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the shared `or-lens` span collector backing the dashboard.
    #[must_use]
    pub fn collector(&self) -> SpanCollector {
        self.collector.clone()
    }

    /// Signals the `or-lens` dashboard server to stop accepting new requests.
    pub fn shutdown(&self) {
        self.shutdown.notify_waiters();
    }
}

/// Starts an `or-lens` dashboard server with a fresh in-memory collector.
pub async fn start_dashboard_server(port: u16) -> Result<LensHandle, LensError> {
    start_dashboard_server_with_collector(SpanCollector::new(), port).await
}

/// Starts an `or-lens` dashboard server with an existing collector instance.
pub async fn start_dashboard_server_with_collector(
    collector: SpanCollector,
    port: u16,
) -> Result<LensHandle, LensError> {
    let listener = TcpListener::bind(("127.0.0.1", port))
        .await
        .map_err(|error| LensError::Bind(error.to_string()))?;
    let bound_port = listener
        .local_addr()
        .map_err(|error| LensError::Bind(error.to_string()))?
        .port();
    let shutdown = Arc::new(Notify::new());
    let notify = Arc::clone(&shutdown);
    let app = router(collector.clone());

    tracing::info!(target: "or_lens", port = bound_port, "starting dashboard server");
    tokio::spawn(async move {
        serve(listener, app, notify).await;
    });

    Ok(LensHandle {
        port: bound_port,
        collector,
        shutdown,
    })
}

/// Builds an `or-lens` execution snapshot from a collected span list.
#[must_use]
pub fn snapshot_from_spans(trace_id: &str, spans: &[LensSpan]) -> ExecutionSnapshot {
    let mut ordered = spans.to_vec();
    ordered.sort_by(|left, right| {
        left.started_at_ms
            .cmp(&right.started_at_ms)
            .then(left.span_id.cmp(&right.span_id))
    });

    ExecutionSnapshot {
        trace_id: trace_id.to_owned(),
        nodes: ordered
            .into_iter()
            .map(|span| ExecutionNodeSnapshot {
                span_id: span.span_id,
                parent_span_id: span.parent_span_id,
                name: span.name,
                status: span.status,
                started_at_ms: span.started_at_ms,
                duration_ms: span
                    .ended_at_ms
                    .map(|end| end.saturating_sub(span.started_at_ms))
                    .unwrap_or(0),
                state_delta: state_delta(&span.state_before, &span.state_after),
            })
            .collect(),
    }
}

async fn serve(listener: TcpListener, app: Router, shutdown: Arc<Notify>) {
    let result = axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            shutdown.notified().await;
        })
        .await;
    if let Err(error) = result {
        tracing::warn!(target: "or_lens", error = %error, "dashboard server stopped");
    }
}

fn state_delta(before: &Value, after: &Value) -> Value {
    match (before.as_object(), after.as_object()) {
        (Some(before), Some(after)) => {
            let mut delta = Map::new();
            for key in before.keys().chain(after.keys()) {
                let previous = before.get(key);
                let next = after.get(key);
                if previous != next {
                    delta.insert(
                        key.clone(),
                        json!({
                            "before": previous.cloned().unwrap_or(Value::Null),
                            "after": next.cloned().unwrap_or(Value::Null),
                        }),
                    );
                }
            }
            Value::Object(delta)
        }
        _ if before == after => Value::Object(Map::new()),
        _ => json!({
            "before": before,
            "after": after,
        }),
    }
}
