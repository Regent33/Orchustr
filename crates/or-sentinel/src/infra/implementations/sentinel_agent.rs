use crate::domain::contracts::SentinelAgentTrait;
use crate::domain::entities::{SentinelConfig, StepOutcome};
use crate::domain::errors::SentinelError;
use crate::infra::adapters::parsing::{ModelDecision, approx_prompt_tokens, parse_decision};
use crate::infra::adapters::state::{
    append_tool_observation, clear_internal_state, config_from_state, last_tool_call,
    messages_from_state, pending_tool_call, prepare_step_state, set_final_answer,
    set_last_tool_call, set_pending_tool_call, take_final_answer,
};
use crate::infra::implementations::support::{invoke_with_retry, node_error};
use or_conduit::ConduitProvider;
use or_core::{CoreOrchestrator, DynState, RetryPolicy, TokenBudget};
use or_forge::ForgeRegistry;
use or_loom::{ExecutionGraph, GraphBuilder, NodeResult};

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
        let graph = GraphBuilder::new()
            .add_node("think", {
                let provider = provider.clone();
                move |state: DynState| {
                    let provider = provider.clone();
                    async move {
                        let messages = messages_from_state(&state)
                            .map_err(|error| node_error("think", error))?;
                        let config = config_from_state(&state)
                            .map_err(|error| node_error("think", error))?;
                        CoreOrchestrator::new()
                            .enforce_completion_budget(
                                &config.step_budget,
                                approx_prompt_tokens(&messages),
                            )
                            .map_err(|error| {
                                node_error("think", SentinelError::Core(error.to_string()))
                            })?;
                        let response =
                            provider
                                .complete_messages(messages)
                                .await
                                .map_err(|error| {
                                    node_error("think", SentinelError::Conduit(error.to_string()))
                                })?;
                        let mut next = state.clone();
                        match parse_decision(&response.text)
                            .map_err(|error| node_error("think", error))?
                        {
                            ModelDecision::ToolCall { tool_name, args } => {
                                set_pending_tool_call(&mut next, tool_name, args);
                                NodeResult::branch(next, "act")
                            }
                            ModelDecision::FinalAnswer { answer } => {
                                set_final_answer(&mut next, answer);
                                NodeResult::branch(next, "exit")
                            }
                        }
                    }
                }
            })
            .add_node("act", {
                let registry = registry.clone();
                move |state: DynState| {
                    let registry = registry.clone();
                    async move {
                        let config =
                            config_from_state(&state).map_err(|error| node_error("act", error))?;
                        let (tool_name, args) =
                            pending_tool_call(&state).map_err(|error| node_error("act", error))?;
                        let tool_result =
                            invoke_with_retry(&registry, &tool_name, args.clone(), &config)
                                .await
                                .map_err(|error| node_error("act", error))?;
                        let mut next = state.clone();
                        append_tool_observation(&mut next, &tool_result)
                            .map_err(|error| node_error("act", error))?;
                        set_last_tool_call(&mut next, tool_name, args);
                        NodeResult::branch(next, "exit")
                    }
                }
            })
            .add_node("exit", |state: DynState| async move {
                NodeResult::advance(state)
            })
            .add_edge("think", "act")
            .add_edge("think", "exit")
            .add_edge("act", "exit")
            .set_entry("think")
            .set_exit("exit")
            .build()
            .map_err(|error| SentinelError::Loom(error.to_string()))?;
        Ok(Self {
            graph,
            _provider: provider,
            _registry: registry,
        })
    }

    pub(crate) async fn step_once(
        &self,
        state: DynState,
        config: SentinelConfig,
        step_index: u32,
    ) -> Result<(StepOutcome, DynState), SentinelError> {
        let mut state = prepare_step_state(state, &config, step_index)?;
        state = self
            .graph
            .execute(state)
            .await
            .map_err(|error| SentinelError::Loom(error.to_string()))?;
        if let Some(answer) = take_final_answer(&mut state) {
            let cleaned = clear_internal_state(state);
            return Ok((
                StepOutcome::FinalAnswer {
                    answer,
                    state: cleaned.clone(),
                },
                cleaned,
            ));
        }
        let (tool_name, args) = last_tool_call(&state)?;
        let cleaned = clear_internal_state(state);
        Ok((
            StepOutcome::ToolCall {
                tool_name,
                args,
                step_index,
            },
            cleaned,
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
