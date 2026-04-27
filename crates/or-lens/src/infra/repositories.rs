use crate::domain::contracts::TraceRepository;
use crate::domain::entities::{LensSpan, TraceSummary};
use dashmap::DashMap;
use std::sync::Arc;

/// Default per-trace span retention cap. Beyond this the collector drops
/// the oldest spans to keep memory bounded.
pub const DEFAULT_SPANS_PER_TRACE: usize = 10_000;

/// Default maximum number of distinct trace ids retained.
pub const DEFAULT_MAX_TRACES: usize = 1_024;

/// In-memory trace repository used by the `or-lens` dashboard server.
///
/// Bounded on two axes:
/// * `spans_per_trace` — when exceeded the oldest spans for that trace are
///   evicted (FIFO by `started_at_ms`).
/// * `max_traces` — when exceeded the trace with the oldest activity is
///   evicted entirely.
///
/// These caps replace the previous unbounded behaviour, where every span
/// was retained for the lifetime of the process.
#[derive(Clone)]
pub struct SpanCollector {
    spans: Arc<DashMap<String, Vec<LensSpan>>>,
    spans_per_trace: usize,
    max_traces: usize,
}

impl Default for SpanCollector {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_SPANS_PER_TRACE, DEFAULT_MAX_TRACES)
    }
}

impl SpanCollector {
    /// Creates a span collector with default bounds
    /// (`DEFAULT_SPANS_PER_TRACE`, `DEFAULT_MAX_TRACES`).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a span collector with explicit retention bounds. Both
    /// values must be at least 1; smaller values are clamped to 1.
    #[must_use]
    pub fn with_capacity(spans_per_trace: usize, max_traces: usize) -> Self {
        Self {
            spans: Arc::new(DashMap::new()),
            spans_per_trace: spans_per_trace.max(1),
            max_traces: max_traces.max(1),
        }
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

    fn evict_oldest_trace(&self) {
        // Find the trace with the smallest most-recent activity timestamp
        // and drop it. Tie-broken on trace id for determinism.
        let candidate = self
            .spans
            .iter()
            .map(|entry| {
                let latest = entry
                    .value()
                    .iter()
                    .map(|span| span.started_at_ms)
                    .max()
                    .unwrap_or(0);
                (entry.key().clone(), latest)
            })
            .min_by(|left, right| left.1.cmp(&right.1).then_with(|| left.0.cmp(&right.0)));
        if let Some((trace_id, _)) = candidate {
            self.spans.remove(&trace_id);
        }
    }
}

impl TraceRepository for SpanCollector {
    fn record_span(&self, span: LensSpan) {
        // If recording this span would introduce a new trace id and we are
        // already at the trace cap, evict the oldest existing trace first.
        // The check + insert is racy under heavy contention, which is fine:
        // worst case we briefly hold `max_traces + 1` traces.
        if !self.spans.contains_key(&span.trace_id) && self.spans.len() >= self.max_traces {
            self.evict_oldest_trace();
        }
        let mut entry = self.spans.entry(span.trace_id.clone()).or_default();
        entry.push(span);
        entry.sort_by(|left, right| {
            left.started_at_ms
                .cmp(&right.started_at_ms)
                .then(left.span_id.cmp(&right.span_id))
        });
        // Drop oldest spans past the per-trace cap.
        if entry.len() > self.spans_per_trace {
            let drop_count = entry.len() - self.spans_per_trace;
            entry.drain(0..drop_count);
        }
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
