use or_conduit::ConduitProvider;
use or_core::DynState;
use or_forge::ForgeRegistry;
use or_loom::GraphBuilder;

/// A pluggable loop topology for a Sentinel agent in `or-sentinel`.
///
/// Implement this trait to define custom agent loop shapes such as ReAct,
/// plan-execute, or reflection without modifying the core `SentinelAgent`
/// runtime. Before this additive API existed, callers had to rely on the
/// fixed loop hard-coded inside `SentinelAgent::new`.
pub trait LoopTopology: Send + Sync + 'static {
    /// Builds a configured graph builder for this topology.
    ///
    /// Custom topologies may attach handlers directly here and leave
    /// [`LoopTopology::bind`] as a no-op (its default behaviour). The
    /// built-in `or-sentinel` topologies (`ReActTopology`,
    /// `PlanExecuteTopology`, `ReflectionTopology`) declare structure
    /// first via `add_placeholder_node` and bind the standard runtime
    /// handlers in `bind` so callers can swap providers without
    /// reimplementing the loop.
    fn build(&self) -> GraphBuilder<DynState>;

    /// Returns a human-readable name for this topology.
    fn name(&self) -> &'static str;

    /// Binds runtime handlers onto the placeholder nodes declared in
    /// [`LoopTopology::build`].
    ///
    /// The default implementation returns the builder unchanged. Custom
    /// topologies that already attached their own handlers in `build` can
    /// rely on this default. Built-in topologies override this method to
    /// wire `provider` and `registry` into their think/act/exit nodes.
    fn bind<P>(
        &self,
        builder: GraphBuilder<DynState>,
        provider: P,
        registry: ForgeRegistry,
    ) -> GraphBuilder<DynState>
    where
        P: ConduitProvider + Clone + Send + Sync + 'static,
    {
        let _ = (provider, registry);
        builder
    }
}
