//! State helpers operating on the *user-facing* portion of `DynState`.
//!
//! Sentinel-internal control data (config, step index, pending tool
//! call, final answer, completed tool call) lives in
//! [`crate::infra::adapters::context::SentinelStepContext`] and is no
//! longer smuggled through `DynState`.

use crate::domain::errors::SentinelError;
use or_conduit::{CompletionMessage, MessageRole};
use or_core::DynState;

const MESSAGES_KEY: &str = "messages";

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
