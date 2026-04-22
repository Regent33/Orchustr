use crate::domain::contracts::TraceRepository;
use crate::domain::entities::{LensSpan, LensSpanStatus};
use dashmap::DashMap;
use serde_json::{Value, json};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Metadata, Subscriber};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

#[derive(Clone)]
struct PendingSpan {
    trace_id: String,
    span_id: String,
    parent_span_id: Option<String>,
    name: String,
    started_at_ms: u64,
    status: LensSpanStatus,
    state_before: Value,
    state_after: Value,
}

#[derive(Default)]
struct FieldCapture {
    values: Vec<(String, Value)>,
}

impl Visit for FieldCapture {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.values.push((field.name().to_owned(), json!(format!("{value:?}"))));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.values.push((field.name().to_owned(), json!(value)));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.values.push((field.name().to_owned(), json!(value)));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.values.push((field.name().to_owned(), json!(value)));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.values.push((field.name().to_owned(), json!(value)));
    }
}

/// A tracing layer that mirrors completed spans into the `or-lens` trace repository.
#[derive(Clone, Default)]
pub struct LensLayer<R: TraceRepository> {
    repository: R,
    pending: Arc<DashMap<u64, PendingSpan>>,
}

impl<R: TraceRepository> LensLayer<R> {
    /// Creates a new `or-lens` tracing layer backed by the given repository.
    #[must_use]
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            pending: Arc::new(DashMap::new()),
        }
    }
}

impl<R, S> Layer<S> for LensLayer<R>
where
    R: TraceRepository,
    S: Subscriber,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let mut capture = FieldCapture::default();
        attrs.record(&mut capture);

        let parent_id = attrs.parent().cloned().or_else(|| ctx.current_span().id().cloned());
        let parent_span_id = parent_id.as_ref().map(span_key);
        let trace_id = parent_id
            .as_ref()
            .and_then(|parent| self.pending.get(&parent.clone().into_u64()))
            .map(|span| span.trace_id.clone())
            .unwrap_or_else(|| span_key(id));
        let mut pending = PendingSpan {
            trace_id,
            span_id: span_key(id),
            parent_span_id,
            name: attrs.metadata().name().to_owned(),
            started_at_ms: timestamp_ms(),
            status: LensSpanStatus::InProgress,
            state_before: Value::Null,
            state_after: Value::Null,
        };
        apply_fields(&mut pending, capture.values);
        self.pending.insert(id.clone().into_u64(), pending);
    }

    fn on_record(&self, id: &Id, values: &Record<'_>, _ctx: Context<'_, S>) {
        if let Some(mut span) = self.pending.get_mut(&id.clone().into_u64()) {
            let mut capture = FieldCapture::default();
            values.record(&mut capture);
            apply_fields(&mut span, capture.values);
        }
    }

    fn on_close(&self, id: Id, _ctx: Context<'_, S>) {
        if let Some((_, span)) = self.pending.remove(&id.into_u64()) {
            self.repository.record_span(LensSpan {
                trace_id: span.trace_id,
                span_id: span.span_id,
                parent_span_id: span.parent_span_id,
                name: span.name,
                started_at_ms: span.started_at_ms,
                ended_at_ms: Some(timestamp_ms()),
                status: span.status,
                state_before: span.state_before,
                state_after: span.state_after,
            });
        }
    }

    fn enabled(&self, _metadata: &Metadata<'_>, _ctx: Context<'_, S>) -> bool {
        true
    }
}

fn apply_fields(span: &mut PendingSpan, fields: Vec<(String, Value)>) {
    for (name, value) in fields {
        match name.as_str() {
            "state_before" => span.state_before = value,
            "state_after" => span.state_after = value,
            "status" => {
                span.status = match value.as_str() {
                    Some("error") | Some("failed") => LensSpanStatus::Errored,
                    Some("in_progress") => LensSpanStatus::InProgress,
                    _ => LensSpanStatus::Completed,
                };
            }
            _ => {}
        }
    }
}

fn span_key(id: &Id) -> String {
    id.clone().into_u64().to_string()
}

fn timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}
