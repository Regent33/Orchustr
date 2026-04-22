use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Execution status recorded for a collected `or-lens` span.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LensSpanStatus {
    /// The span completed successfully.
    Completed,
    /// The span is still open and has not been closed yet.
    InProgress,
    /// The span completed with an error status.
    Errored,
}

/// A serializable span record collected by the `or-lens` dashboard.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LensSpan {
    /// Trace identifier for grouping related spans.
    pub trace_id: String,
    /// Unique identifier for this span inside its trace.
    pub span_id: String,
    /// Optional parent span identifier for tree reconstruction.
    pub parent_span_id: Option<String>,
    /// Human-readable span name.
    pub name: String,
    /// Start timestamp in Unix milliseconds.
    pub started_at_ms: u64,
    /// End timestamp in Unix milliseconds when the span closed.
    pub ended_at_ms: Option<u64>,
    /// Completion status for the span.
    pub status: LensSpanStatus,
    /// Optional state snapshot before the span executed.
    pub state_before: Value,
    /// Optional state snapshot after the span executed.
    pub state_after: Value,
}

/// Summary metadata returned by the `or-lens` trace listing endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceSummary {
    /// Trace identifier for the summary row.
    pub trace_id: String,
    /// Number of spans recorded for the trace.
    pub span_count: usize,
    /// Latest start time across all spans in the trace.
    pub latest_started_at_ms: u64,
}

/// A serializable execution timeline produced from collected `or-lens` spans.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExecutionSnapshot {
    /// Trace identifier represented by this snapshot.
    pub trace_id: String,
    /// Ordered node execution records for the trace.
    pub nodes: Vec<ExecutionNodeSnapshot>,
}

/// A serializable node-level view of a collected `or-lens` span.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExecutionNodeSnapshot {
    /// Span identifier for the node execution.
    pub span_id: String,
    /// Optional parent span identifier for tree rendering.
    pub parent_span_id: Option<String>,
    /// Display name for the node execution.
    pub name: String,
    /// Execution status for the node.
    pub status: LensSpanStatus,
    /// Start timestamp in Unix milliseconds.
    pub started_at_ms: u64,
    /// Duration in milliseconds when the span has completed.
    pub duration_ms: u64,
    /// Serialized state delta captured for the node.
    pub state_delta: Value,
}
