use crate::domain::entities::{ColonyMessage, ColonyResult};
use crate::domain::errors::ColonyError;
use or_core::DynState;

pub(crate) fn seed_message(state: &DynState) -> Result<ColonyMessage, ColonyError> {
    let task = state
        .get("task")
        .and_then(|value| value.as_str())
        .ok_or(ColonyError::MissingTask)?;
    Ok(ColonyMessage {
        from: "user".to_owned(),
        to: "colony".to_owned(),
        content: task.to_owned(),
    })
}

pub(crate) fn record_message(
    state: &mut DynState,
    message: &ColonyMessage,
) -> Result<(), ColonyError> {
    let mut outputs = state
        .get("member_outputs")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let map = outputs
        .as_object_mut()
        .ok_or_else(|| ColonyError::InvalidState("member_outputs must be an object".to_owned()))?;
    map.insert(message.from.clone(), serde_json::json!(message.content));
    state.insert("member_outputs".to_owned(), outputs);
    Ok(())
}

pub(crate) fn result_from_parts(
    mut state: DynState,
    transcript: Vec<ColonyMessage>,
) -> Result<ColonyResult, ColonyError> {
    let summary = transcript
        .iter()
        .skip(1)
        .map(|message| format!("{} -> {}: {}", message.from, message.to, message.content))
        .collect::<Vec<_>>()
        .join("\n");
    state.insert(
        "colony_summary".to_owned(),
        serde_json::json!(summary.clone()),
    );
    state.insert(
        "colony_messages".to_owned(),
        serde_json::to_value(&transcript)
            .map_err(|error| ColonyError::Serialization(error.to_string()))?,
    );
    Ok(ColonyResult {
        summary,
        state,
        transcript,
    })
}
