use crate::domain::contracts::TraceRepository;
use crate::domain::entities::{LensSpan, TraceSummary};
use dashmap::DashMap;
use std::sync::Arc;

/// In-memory trace repository used by the `or-lens` dashboard server.
#[derive(Clone, Default)]
pub struct SpanCollector {
    spans: Arc<DashMap<String, Vec<LensSpan>>>,
}

impl SpanCollector {
    /// Creates an empty `or-lens` span collector.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a single collected span into the in-memory `or-lens` trace store.
    pub fn record_span(&self, span: LensSpan) {
        <Self as TraceRepository>::record_span(self, span);
    }

    /// Returns all recorded spans for a trace, ordered by start time.
    #[must_use]
    pub fn trace(&self, trace_id: &str) -> Option<Vec<LensSpan>> {
        <Self as TraceRepository>::trace(self, trace_id)
    }

    /// Returns recent trace summaries ordered by latest activity.
    #[must_use]
    pub fn traces(&self) -> Vec<TraceSummary> {
        <Self as TraceRepository>::traces(self)
    }
}

impl TraceRepository for SpanCollector {
    fn record_span(&self, span: LensSpan) {
        let mut entry = self.spans.entry(span.trace_id.clone()).or_default();
        entry.push(span);
        entry.sort_by(|left, right| {
            left.started_at_ms
                .cmp(&right.started_at_ms)
                .then(left.span_id.cmp(&right.span_id))
        });
    }

    fn trace(&self, trace_id: &str) -> Option<Vec<LensSpan>> {
        self.spans.get(trace_id).map(|spans| spans.clone())
    }

    fn traces(&self) -> Vec<TraceSummary> {
        let mut traces = self
            .spans
            .iter()
            .map(|entry| TraceSummary {
                trace_id: entry.key().clone(),
                span_count: entry.value().len(),
                latest_started_at_ms: entry
                    .value()
                    .iter()
                    .map(|span| span.started_at_ms)
                    .max()
                    .unwrap_or(0),
            })
            .collect::<Vec<_>>();
        traces.sort_by(|left, right| {
            right
                .latest_started_at_ms
                .cmp(&left.latest_started_at_ms)
                .then(left.trace_id.cmp(&right.trace_id))
        });
        traces
    }
}
