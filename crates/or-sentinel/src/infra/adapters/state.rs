use crate::domain::entities::SentinelConfig;
use crate::domain::errors::SentinelError;
use crate::infra::adapters::parsing::config_to_value;
use or_conduit::{CompletionMessage, MessageRole};
use or_core::DynState;

pub(crate) const CONFIG_KEY: &str = "__sentinel_config";
pub(crate) const STEP_INDEX_KEY: &str = "__sentinel_step_index";
const FINAL_ANSWER_KEY: &str = "__sentinel_final_answer";
const LAST_TOOL_CALL_KEY: &str = "__sentinel_last_tool_call";
const MESSAGES_KEY: &str = "messages";
const PENDING_TOOL_CALL_KEY: &str = "__sentinel_pending_tool_call";

pub(crate) fn prepare_step_state(
    mut state: DynState,
    config: &SentinelConfig,
    step_index: u32,
) -> Result<DynState, SentinelError> {
    state.insert(CONFIG_KEY.to_owned(), config_to_value(config)?);
    state.insert(STEP_INDEX_KEY.to_owned(), serde_json::json!(step_index));
    Ok(state)
}

pub(crate) fn config_from_state(state: &DynState) -> Result<SentinelConfig, SentinelError> {
    let value = state
        .get(CONFIG_KEY)
        .cloned()
        .ok_or_else(|| SentinelError::InvalidState("sentinel config missing".to_owned()))?;
    serde_json::from_value(value).map_err(|error| SentinelError::Serialization(error.to_string()))
}

pub(crate) fn messages_from_state(
    state: &DynState,
) -> Result<Vec<CompletionMessage>, SentinelError> {
    let value = state
        .get(MESSAGES_KEY)
        .cloned()
        .ok_or(SentinelError::MissingMessages)?;
    serde_json::from_value(value).map_err(|error| SentinelError::Serialization(error.to_string()))
}

pub(crate) fn write_messages(
    state: &mut DynState,
    messages: &[CompletionMessage],
) -> Result<(), SentinelError> {
    state.insert(
        MESSAGES_KEY.to_owned(),
        serde_json::to_value(messages)
            .map_err(|error| SentinelError::Serialization(error.to_string()))?,
    );
    Ok(())
}

pub(crate) fn append_tool_observation(
    state: &mut DynState,
    tool_result: &serde_json::Value,
) -> Result<(), SentinelError> {
    let mut messages = messages_from_state(state)?;
    let text = serde_json::to_string(tool_result)
        .map_err(|error| SentinelError::Serialization(error.to_string()))?;
    messages.push(CompletionMessage::single_text(MessageRole::Tool, text));
    write_messages(state, &messages)
}

pub(crate) fn set_pending_tool_call(
    state: &mut DynState,
    tool_name: String,
    args: serde_json::Value,
) {
    state.insert(
        PENDING_TOOL_CALL_KEY.to_owned(),
        serde_json::json!({ "tool_name": tool_name, "args": args }),
    );
}

pub(crate) fn pending_tool_call(
    state: &DynState,
) -> Result<(String, serde_json::Value), SentinelError> {
    let value = state
        .get(PENDING_TOOL_CALL_KEY)
        .cloned()
        .ok_or_else(|| SentinelError::InvalidState("pending tool call missing".to_owned()))?;
    let tool_name = value["tool_name"]
        .as_str()
        .ok_or_else(|| SentinelError::InvalidState("tool name missing".to_owned()))?;
    Ok((
        tool_name.to_owned(),
        value
            .get("args")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({})),
    ))
}

pub(crate) fn set_final_answer(state: &mut DynState, answer: String) {
    state.insert(FINAL_ANSWER_KEY.to_owned(), serde_json::json!(answer));
}

pub(crate) fn take_final_answer(state: &mut DynState) -> Option<String> {
    state
        .remove(FINAL_ANSWER_KEY)
        .and_then(|value| value.as_str().map(ToOwned::to_owned))
}

pub(crate) fn set_last_tool_call(state: &mut DynState, tool_name: String, args: serde_json::Value) {
    state.insert(
        LAST_TOOL_CALL_KEY.to_owned(),
        serde_json::json!({ "tool_name": tool_name, "args": args }),
    );
    state.remove(PENDING_TOOL_CALL_KEY);
}

pub(crate) fn last_tool_call(
    state: &DynState,
) -> Result<(String, serde_json::Value), SentinelError> {
    let value = state
        .get(LAST_TOOL_CALL_KEY)
        .cloned()
        .ok_or_else(|| SentinelError::InvalidState("completed tool call missing".to_owned()))?;
    let tool_name = value["tool_name"]
        .as_str()
        .ok_or_else(|| SentinelError::InvalidState("tool name missing".to_owned()))?;
    Ok((
        tool_name.to_owned(),
        value
            .get("args")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({})),
    ))
}

pub(crate) fn clear_internal_state(mut state: DynState) -> DynState {
    state.remove(CONFIG_KEY);
    state.remove(FINAL_ANSWER_KEY);
    state.remove(LAST_TOOL_CALL_KEY);
    state.remove(PENDING_TOOL_CALL_KEY);
    state.remove(STEP_INDEX_KEY);
    state
}
