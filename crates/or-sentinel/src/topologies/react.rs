use crate::domain::errors::SentinelError;
use crate::infra::adapters::context::with_context;
use crate::infra::adapters::parsing::{ModelDecision, approx_prompt_tokens, parse_decision};
use crate::infra::adapters::state::{append_tool_observation, messages_from_state};
use crate::infra::implementations::support::{invoke_with_retry, node_error};
use crate::topology::LoopTopology;
use or_conduit::ConduitProvider;
use or_core::{CoreOrchestrator, DynState};
use or_forge::ForgeRegistry;
use or_loom::{GraphBuilder, NodeResult};

/// The legacy ReAct loop topology used by `or-sentinel`.
///
/// This built-in topology preserves the existing `SentinelAgent::new`
/// structure: `think -> act/exit -> exit`, with repeated outer-loop stepping
/// still handled by `SentinelAgent::run`.
#[derive(Debug, Clone, Default)]
pub struct ReActTopology;

impl LoopTopology for ReActTopology {
    fn build(&self) -> GraphBuilder<DynState> {
        GraphBuilder::new()
            .add_placeholder_node("think")
            .add_placeholder_node("act")
            .add_placeholder_node("exit")
            .add_edge("think", "act")
            .add_edge("think", "exit")
            .add_edge("act", "exit")
            .set_entry("think")
            .set_exit("exit")
    }

    fn name(&self) -> &'static str {
        "react"
    }

    fn bind<P>(
        &self,
        builder: GraphBuilder<DynState>,
        provider: P,
        registry: ForgeRegistry,
    ) -> GraphBuilder<DynState>
    where
        P: ConduitProvider + Clone + Send + Sync + 'static,
    {
        bind_react(builder, provider, registry)
    }
}

pub(crate) fn bind_react<P>(
    builder: GraphBuilder<DynState>,
    provider: P,
    registry: ForgeRegistry,
) -> GraphBuilder<DynState>
where
    P: ConduitProvider + Clone + Send + Sync + 'static,
{
    builder
        .bind_node("think", {
            let provider = provider.clone();
            move |state: DynState| {
                let provider = provider.clone();
                async move {
                    let messages =
                        messages_from_state(&state).map_err(|error| node_error("think", error))?;
                    let config = with_context(|ctx| ctx.config())
                        .map_err(|error| node_error("think", error))?;
                    CoreOrchestrator::new()
                        .enforce_completion_budget(
                            &config.step_budget,
                            approx_prompt_tokens(&messages),
                        )
                        .map_err(|error| node_error("think", SentinelError::from(error)))?;
                    let response = provider
                        .complete_messages(messages)
                        .await
                        .map_err(|error| {
                            node_error("think", SentinelError::Conduit(error.to_string()))
                        })?;
                    match parse_decision(&response.text)
                        .map_err(|error| node_error("think", error))?
                    {
                        ModelDecision::ToolCall { tool_name, args } => {
                            with_context(|ctx| ctx.set_pending_tool_call(tool_name, args))
                                .map_err(|error| node_error("think", error))?;
                            NodeResult::branch(state, "act")
                        }
                        ModelDecision::FinalAnswer { answer } => {
                            with_context(|ctx| ctx.set_final_answer(answer))
                                .map_err(|error| node_error("think", error))?;
                            NodeResult::branch(state, "exit")
                        }
                    }
                }
            }
        })
        .bind_node("act", {
            let registry = registry.clone();
            move |state: DynState| {
                let registry = registry.clone();
                async move {
                    let config = with_context(|ctx| ctx.config())
                        .map_err(|error| node_error("act", error))?;
                    let (tool_name, args) = with_context(|ctx| ctx.take_pending_tool_call())
                        .map_err(|error| node_error("act", error))?
                        .map_err(|error| node_error("act", error))?;
                    let tool_result =
                        invoke_with_retry(&registry, &tool_name, args.clone(), &config)
                            .await
                            .map_err(|error| node_error("act", error))?;
                    // The closure receives `state` by value (the executor
                    // already cloned for us). Mutate in place instead of
                    // cloning again — saves one full DynState copy per step.
                    let mut state = state;
                    append_tool_observation(&mut state, &tool_result)
                        .map_err(|error| node_error("act", error))?;
                    with_context(|ctx| ctx.set_last_tool_call(tool_name, args))
                        .map_err(|error| node_error("act", error))?;
                    NodeResult::branch(state, "exit")
                }
            }
        })
        .bind_node("exit", |state: DynState| async move {
            NodeResult::advance(state)
        })
}
