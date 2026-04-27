use crate::domain::contracts::SentinelAgentTrait;
use crate::domain::entities::{SentinelConfig, StepOutcome};
use crate::domain::errors::SentinelError;
use crate::infra::adapters::context::{SENTINEL_CTX, SentinelStepContext};
use crate::topologies::{ReActTopology, bind_react};
use crate::topology::LoopTopology;
use or_conduit::ConduitProvider;
use or_core::{DynState, RetryPolicy, TokenBudget};
use or_forge::ForgeRegistry;
use or_loom::{ExecutionGraph, GraphInspection};

#[derive(Clone)]
pub struct SentinelAgent<P> {
    graph: ExecutionGraph<DynState>,
    _provider: P,
    _registry: ForgeRegistry,
}

impl<P> SentinelAgent<P>
where
    P: ConduitProvider + Clone + Send + Sync + 'static,
{
    pub fn new(provider: P, registry: ForgeRegistry) -> Result<Self, SentinelError> {
        let graph = bind_react(ReActTopology.build(), provider.clone(), registry.clone())
            .build()
            .map_err(SentinelError::from)?;
        Ok(Self::from_graph(graph, provider, registry))
    }

    pub(crate) fn from_graph(
        graph: ExecutionGraph<DynState>,
        provider: P,
        registry: ForgeRegistry,
    ) -> Self {
        Self {
            graph,
            _provider: provider,
            _registry: registry,
        }
    }

    /// Returns a structural snapshot of the internal `or-loom` graph.
    ///
    /// This additive helper lives in `or-sentinel` so callers can compare the
    /// legacy constructor and builder-backed topologies without reaching into
    /// private graph internals.
    #[must_use]
    pub fn graph_inspection(&self) -> GraphInspection {
        self.graph.inspect()
    }

    pub(crate) async fn step_once(
        &self,
        state: DynState,
        config: SentinelConfig,
        step_index: u32,
    ) -> Result<(StepOutcome, DynState), SentinelError> {
        // Run the graph inside a task-local scope so topology nodes can
        // reach the typed `SentinelStepContext` instead of the previous
        // five magic `__sentinel_*` keys in `DynState`.
        let context = SentinelStepContext::new(config, step_index);
        let context_for_scope = context.clone();
        let new_state = SENTINEL_CTX
            .scope(context_for_scope, self.graph.execute(state))
            .await
            .map_err(SentinelError::from)?;
        if let Some(answer) = context.take_final_answer() {
            return Ok((
                StepOutcome::FinalAnswer {
                    answer,
                    state: new_state.clone(),
                },
                new_state,
            ));
        }
        let (tool_name, args) = context.take_last_tool_call()?;
        Ok((
            StepOutcome::ToolCall {
                tool_name,
                args,
                step_index,
            },
            new_state,
        ))
    }
}

impl<P> SentinelAgentTrait for SentinelAgent<P>
where
    P: ConduitProvider + Clone + Send + Sync + 'static,
{
    async fn run(
        &self,
        initial_state: DynState,
        config: SentinelConfig,
    ) -> Result<StepOutcome, SentinelError> {
        let mut state = initial_state;
        for step_index in 1..=config.max_steps {
            let (outcome, next_state) = self.step_once(state, config.clone(), step_index).await?;
            match outcome {
                StepOutcome::ToolCall { .. } => state = next_state,
                StepOutcome::FinalAnswer { .. } => return Ok(outcome),
                StepOutcome::StepLimitReached { .. } => return Ok(outcome),
            }
        }
        Ok(StepOutcome::StepLimitReached {
            last_state: state,
            steps_taken: config.max_steps,
        })
    }

    async fn step(&self, state: DynState, step_index: u32) -> Result<StepOutcome, SentinelError> {
        let config = SentinelConfig {
            max_steps: step_index.max(1),
            step_budget: TokenBudget {
                max_context_tokens: 8_192,
                max_completion_tokens: 1_024,
            },
            tool_retry: RetryPolicy::no_retry(),
        };
        self.step_once(state, config, step_index)
            .await
            .map(|(outcome, _)| outcome)
    }
}
