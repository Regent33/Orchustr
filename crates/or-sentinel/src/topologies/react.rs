use crate::domain::errors::SentinelError;
use crate::infra::adapters::parsing::{ModelDecision, approx_prompt_tokens, parse_decision};
use crate::infra::adapters::state::{
    append_tool_observation, config_from_state, messages_from_state, pending_tool_call,
    set_final_answer, set_last_tool_call, set_pending_tool_call,
};
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
                    let config =
                        config_from_state(&state).map_err(|error| node_error("think", error))?;
                    CoreOrchestrator::new()
                        .enforce_completion_budget(
                            &config.step_budget,
                            approx_prompt_tokens(&messages),
                        )
                        .map_err(|error| {
                            node_error("think", SentinelError::Core(error.to_string()))
                        })?;
                    let response = provider
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
        .bind_node("act", {
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
        .bind_node("exit", |state: DynState| async move {
            NodeResult::advance(state)
        })
}
