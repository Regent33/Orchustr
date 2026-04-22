use or_core::DynState;
use or_loom::GraphBuilder;
use std::any::Any;

/// A pluggable loop topology for a Sentinel agent in `or-sentinel`.
///
/// Implement this trait to define custom agent loop shapes such as ReAct,
/// plan-execute, or reflection without modifying the core `SentinelAgent`
/// runtime. Before this additive API existed, callers had to rely on the
/// fixed loop hard-coded inside `SentinelAgent::new`.
pub trait LoopTopology: Any + Send + Sync + 'static {
    /// Builds a configured graph builder for this topology.
    ///
    /// Custom topologies may attach handlers directly. The built-in
    /// `or-sentinel` topologies declare structure first and let
    /// `SentinelAgentBuilder` bind the standard runtime handlers afterward.
    fn build(&self) -> GraphBuilder<DynState>;

    /// Returns a human-readable name for this topology.
    fn name(&self) -> &'static str;
}
