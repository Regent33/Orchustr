use crate::domain::entities::PlanStep;
use crate::domain::errors::SentinelError;
use crate::infra::adapters::parsing::{ModelDecision, parse_decision, parse_plan};
use crate::infra::adapters::state::{
    append_tool_observation, config_from_state, messages_from_state, set_final_answer,
    write_messages,
};
use crate::infra::implementations::support::{invoke_with_retry, node_error};
use crate::topology::LoopTopology;
use or_conduit::{CompletionMessage, ConduitProvider, ContentPart, MessageRole};
use or_core::DynState;
use or_forge::ForgeRegistry;
use or_loom::{GraphBuilder, NodeResult};

const PLAN_CURSOR_KEY: &str = "__sentinel_plan_cursor";
const PLAN_NOTES_KEY: &str = "__sentinel_plan_notes";
const PLAN_STEPS_KEY: &str = "__sentinel_plan_steps";
const PLAN_ORDER_KEY: &str = "plan_execution_order";

/// A plan-execute topology for `or-sentinel`.
///
/// This built-in topology adds an internal planning pass, then executes each
/// generated step in order before synthesizing a final answer. Before this
/// topology existed, the separate `PlanExecuteAgent` type was the only
/// out-of-the-box plan/execution path.
#[derive(Debug, Clone, Default)]
pub struct PlanExecuteTopology;

impl LoopTopology for PlanExecuteTopology {
    fn build(&self) -> GraphBuilder<DynState> {
        GraphBuilder::new()
            .add_placeholder_node("plan")
            .add_placeholder_node("execute_step")
            .add_placeholder_node("check_done")
            .add_placeholder_node("exit")
            .add_edge("plan", "execute_step")
            .add_edge("execute_step", "check_done")
            .add_edge("check_done", "execute_step")
            .add_edge("check_done", "exit")
            .set_entry("plan")
            .set_exit("exit")
    }

    fn name(&self) -> &'static str {
        "plan_execute"
    }
}

pub(crate) fn bind_plan_execute<P>(
    builder: GraphBuilder<DynState>,
    provider: P,
    registry: ForgeRegistry,
) -> GraphBuilder<DynState>
where
    P: ConduitProvider + Clone + Send + Sync + 'static,
{
    builder
        .bind_node("plan", {
            let provider = provider.clone();
            move |state: DynState| {
                let provider = provider.clone();
                async move {
                    if state.contains_key(PLAN_STEPS_KEY) {
                        return NodeResult::advance(state);
                    }

                    let objective = messages_from_state(&state)
                        .map_err(|error| node_error("plan", error))?
                        .into_iter()
                        .flat_map(|message| message.content)
                        .filter_map(|part| match part {
                            ContentPart::Text { text } => Some(text),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    let response = provider
                        .complete_text(&format!(
                            "Return JSON {{\"steps\":[...]}} for this objective:\n{objective}"
                        ))
                        .await
                        .map_err(|error| {
                            node_error("plan", SentinelError::Conduit(error.to_string()))
                        })?;
                    let steps =
                        parse_plan(&response.text).map_err(|error| node_error("plan", error))?;
                    let mut next = state.clone();
                    insert_value(&mut next, "plan", &steps, "plan")?;
                    insert_value(&mut next, PLAN_STEPS_KEY, &steps, "plan")?;
                    next.insert(PLAN_CURSOR_KEY.to_owned(), serde_json::json!(0usize));
                    insert_value(&mut next, PLAN_NOTES_KEY, &Vec::<String>::new(), "plan")?;
                    insert_value(&mut next, PLAN_ORDER_KEY, &Vec::<String>::new(), "plan")?;
                    NodeResult::advance(next)
                }
            }
        })
        .bind_node("execute_step", {
            let provider = provider.clone();
            let registry = registry.clone();
            move |state: DynState| {
                let provider = provider.clone();
                let registry = registry.clone();
                async move {
                    let steps = plan_steps_from_state(&state)?;
                    let cursor = plan_cursor_from_state(&state)?;
                    if cursor >= steps.len() {
                        return NodeResult::advance(state);
                    }

                    let step = steps[cursor].clone();
                    let mut messages = messages_from_state(&state)
                        .map_err(|error| node_error("execute_step", error))?;
                    messages.push(CompletionMessage::single_text(
                        MessageRole::User,
                        format!(
                            "Execute plan step {}: {}",
                            step.step_index, step.description
                        ),
                    ));
                    let response =
                        provider
                            .complete_messages(messages.clone())
                            .await
                            .map_err(|error| {
                                node_error(
                                    "execute_step",
                                    SentinelError::Conduit(error.to_string()),
                                )
                            })?;

                    let mut next = state.clone();
                    write_messages(&mut next, &messages)
                        .map_err(|error| node_error("execute_step", error))?;
                    push_string(
                        &mut next,
                        PLAN_ORDER_KEY,
                        step.description.clone(),
                        "execute_step",
                    )?;

                    match parse_decision(&response.text)
                        .map_err(|error| node_error("execute_step", error))?
                    {
                        ModelDecision::ToolCall { tool_name, args } => {
                            let config = config_from_state(&state)
                                .map_err(|error| node_error("execute_step", error))?;
                            let tool_result =
                                invoke_with_retry(&registry, &tool_name, args, &config)
                                    .await
                                    .map_err(|error| node_error("execute_step", error))?;
                            append_tool_observation(&mut next, &tool_result)
                                .map_err(|error| node_error("execute_step", error))?;
                        }
                        ModelDecision::FinalAnswer { answer } => {
                            push_string(&mut next, PLAN_NOTES_KEY, answer, "execute_step")?;
                        }
                    }

                    next.insert(PLAN_CURSOR_KEY.to_owned(), serde_json::json!(cursor + 1));
                    NodeResult::advance(next)
                }
            }
        })
        .bind_node("check_done", |state: DynState| async move {
            let steps = plan_steps_from_state(&state)?;
            let cursor = plan_cursor_from_state(&state)?;
            let mut next = state.clone();
            if cursor >= steps.len() {
                let notes = string_list_from_state(&state, PLAN_NOTES_KEY)?;
                let answer = if notes.is_empty() {
                    "plan executed without a synthesized answer".to_owned()
                } else {
                    notes.join("\n")
                };
                set_final_answer(&mut next, answer);
                return NodeResult::branch(next, "exit");
            }
            NodeResult::branch(next, "execute_step")
        })
        .bind_node("exit", |state: DynState| async move {
            NodeResult::advance(state)
        })
}

fn insert_value<T: serde::Serialize>(
    state: &mut DynState,
    key: &str,
    value: &T,
    node: &str,
) -> Result<(), or_loom::LoomError> {
    let serialized = serde_json::to_value(value)
        .map_err(|error| node_error(node, SentinelError::Serialization(error.to_string())))?;
    state.insert(key.to_owned(), serialized);
    Ok(())
}

fn plan_steps_from_state(state: &DynState) -> Result<Vec<PlanStep>, or_loom::LoomError> {
    let value = state.get(PLAN_STEPS_KEY).cloned().ok_or_else(|| {
        node_error(
            "plan_execute",
            SentinelError::InvalidState("plan missing".to_owned()),
        )
    })?;
    serde_json::from_value(value).map_err(|error| {
        node_error(
            "plan_execute",
            SentinelError::Serialization(error.to_string()),
        )
    })
}

fn plan_cursor_from_state(state: &DynState) -> Result<usize, or_loom::LoomError> {
    let cursor = state
        .get(PLAN_CURSOR_KEY)
        .and_then(|value| value.as_u64())
        .unwrap_or(0);
    usize::try_from(cursor).map_err(|error| {
        node_error(
            "plan_execute",
            SentinelError::InvalidState(format!("plan cursor overflow: {error}")),
        )
    })
}

fn push_string(
    state: &mut DynState,
    key: &str,
    value: String,
    node: &str,
) -> Result<(), or_loom::LoomError> {
    let mut list = string_list_from_state(state, key)?;
    list.push(value);
    insert_value(state, key, &list, node)
}

fn string_list_from_state(state: &DynState, key: &str) -> Result<Vec<String>, or_loom::LoomError> {
    let value = state
        .get(key)
        .cloned()
        .unwrap_or_else(|| serde_json::json!([]));
    serde_json::from_value(value).map_err(|error| {
        node_error(
            "plan_execute",
            SentinelError::Serialization(error.to_string()),
        )
    })
}
