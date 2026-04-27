use crate::domain::errors::SentinelError;
use crate::infra::implementations::SentinelAgent;
use crate::topologies::ReActTopology;
use crate::topology::LoopTopology;
use or_conduit::ConduitProvider;
use or_forge::ForgeRegistry;

/// Constructs a `SentinelAgent` in `or-sentinel` with a user-supplied loop topology.
///
/// Before this builder existed, callers had to use `SentinelAgent::new`, which
/// always created the fixed built-in ReAct graph.
///
/// ```no_run
/// # use or_forge::ForgeRegistry;
/// # use or_sentinel::{ReActTopology, SentinelAgentBuilder};
/// # fn make_agent<P>(provider: P) -> Result<(), or_sentinel::SentinelError>
/// # where
/// #     P: or_conduit::ConduitProvider + Clone + Send + Sync + 'static,
/// # {
/// let _agent = SentinelAgentBuilder::new()
///     .topology(ReActTopology::default())
///     .conduit(provider)
///     .tool_registry(ForgeRegistry::new())
///     .build()?;
/// # Ok(())
/// # }
/// ```
pub struct SentinelAgentBuilder<T = ReActTopology, P = ()> {
    topology: T,
    provider: P,
    registry: ForgeRegistry,
}

impl SentinelAgentBuilder<ReActTopology, ()> {
    /// Creates a builder with the legacy ReAct topology selected by default.
    #[must_use]
    pub fn new() -> Self {
        Self {
            topology: ReActTopology,
            provider: (),
            registry: ForgeRegistry::new(),
        }
    }
}

impl Default for SentinelAgentBuilder<ReActTopology, ()> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, P> SentinelAgentBuilder<T, P> {
    /// Replaces the current topology with a new one.
    #[must_use]
    pub fn topology<U: LoopTopology>(self, topology: U) -> SentinelAgentBuilder<U, P> {
        SentinelAgentBuilder {
            topology,
            provider: self.provider,
            registry: self.registry,
        }
    }

    /// Sets the conduit provider used by built-in sentinel topologies.
    #[must_use]
    pub fn conduit<NP>(self, provider: NP) -> SentinelAgentBuilder<T, NP> {
        SentinelAgentBuilder {
            topology: self.topology,
            provider,
            registry: self.registry,
        }
    }

    /// Sets the tool registry used by built-in sentinel topologies.
    #[must_use]
    pub fn tool_registry(mut self, registry: ForgeRegistry) -> Self {
        self.registry = registry;
        self
    }
}

impl<T, P> SentinelAgentBuilder<T, P>
where
    T: LoopTopology,
    P: ConduitProvider + Clone + Send + Sync + 'static,
{
    /// Builds a `SentinelAgent` using the configured topology and runtime dependencies.
    pub fn build(self) -> Result<SentinelAgent<P>, SentinelError> {
        // Topologies own their own binding via `LoopTopology::bind`. Custom
        // topologies that attach handlers in `build` rely on the trait's
        // default `bind` (a no-op). Built-in topologies override `bind` to
        // wire `provider` and `registry` into their nodes.
        let builder = self.topology.build();
        let graph = self
            .topology
            .bind(builder, self.provider.clone(), self.registry.clone())
            .build()
            .map_err(SentinelError::from)?;
        Ok(SentinelAgent::from_graph(
            graph,
            self.provider,
            self.registry,
        ))
    }
}
