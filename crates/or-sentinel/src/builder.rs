use crate::domain::errors::SentinelError;
use crate::infra::implementations::SentinelAgent;
use crate::topologies::{
    PlanExecuteTopology, ReActTopology, ReflectionTopology, bind_plan_execute, bind_react,
    bind_reflection,
};
use crate::topology::LoopTopology;
use or_conduit::ConduitProvider;
use or_forge::ForgeRegistry;
use std::any::Any;

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
        let graph = bind_topology(
            self.topology.build(),
            &self.topology,
            self.provider.clone(),
            self.registry.clone(),
        )
        .build()
        .map_err(|error| SentinelError::Loom(error.to_string()))?;
        Ok(SentinelAgent::from_graph(
            graph,
            self.provider,
            self.registry,
        ))
    }
}

fn bind_topology<T, P>(
    builder: or_loom::GraphBuilder<or_core::DynState>,
    topology: &T,
    provider: P,
    registry: ForgeRegistry,
) -> or_loom::GraphBuilder<or_core::DynState>
where
    T: LoopTopology,
    P: ConduitProvider + Clone + Send + Sync + 'static,
{
    let topology_any = topology as &dyn Any;
    if topology_any.is::<ReActTopology>() {
        return bind_react(builder, provider, registry);
    }
    if topology_any.is::<PlanExecuteTopology>() {
        return bind_plan_execute(builder, provider, registry);
    }
    if let Some(reflection) = topology_any.downcast_ref::<ReflectionTopology>() {
        return bind_reflection(builder, reflection, provider, registry);
    }
    builder
}
