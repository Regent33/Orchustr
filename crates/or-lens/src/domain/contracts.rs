use crate::domain::entities::{LensSpan, TraceSummary};

/// Repository contract used by `or-lens` application services to persist and query traces.
pub trait TraceRepository: Clone + Send + Sync + 'static {
    /// Records a collected span in the repository implementation.
    fn record_span(&self, span: LensSpan);

    /// Returns all collected spans for the given trace identifier.
    fn trace(&self, trace_id: &str) -> Option<Vec<LensSpan>>;

    /// Returns recent trace summaries ordered by latest activity.
    fn traces(&self) -> Vec<TraceSummary>;
}
