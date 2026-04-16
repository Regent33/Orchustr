use crate::domain::entities::{
    CompletionMessage, CompletionResponse, ContentPart, FinishReason,
};
use crate::domain::errors::ConduitError;
use or_core::TokenUsage;
use serde_json::{Value, json};

/// Builds a Replicate Predictions API payload.
pub(crate) fn replicate_payload(
    messages: &[CompletionMessage],
) -> Result<Value, ConduitError> {
    let prompt = messages
        .iter()
        .flat_map(|msg| &msg.content)
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    Ok(json!({ "input": { "prompt": prompt } }))
}

/// Parses a Replicate Predictions API response.
/// Supports both synchronous and polling-based responses.
pub(crate) fn parse_replicate_response(
    body: &Value,
) -> Result<CompletionResponse, ConduitError> {
    // Replicate returns output as a string or array of strings.
    let text = if let Some(output_str) = body["output"].as_str() {
        output_str.to_owned()
    } else if let Some(output_arr) = body["output"].as_array() {
        output_arr
            .iter()
            .filter_map(|item| item.as_str())
            .collect::<Vec<_>>()
            .join("")
    } else {
        return Err(ConduitError::Serialization(
            "Replicate response missing output".to_owned(),
        ));
    };
    if text.is_empty() {
        return Err(ConduitError::Serialization(
            "Replicate response empty output".to_owned(),
        ));
    }
    let estimated_tokens = (text.len() as u32 / 4).max(1);
    let usage = TokenUsage {
        prompt_tokens: 0,
        completion_tokens: estimated_tokens,
        total_tokens: estimated_tokens,
    };
    let status = body["status"].as_str().unwrap_or("succeeded");
    let finish_reason = if status == "succeeded" {
        FinishReason::Stop
    } else {
        FinishReason::Length
    };
    Ok(CompletionResponse { text, usage, finish_reason })
}

/// Checks if a Replicate prediction is still processing.
pub(crate) fn replicate_is_pending(body: &Value) -> bool {
    matches!(
        body["status"].as_str(),
        Some("starting") | Some("processing")
    )
}

/// Extracts the polling URL from a Replicate prediction response.
pub(crate) fn replicate_poll_url(body: &Value) -> Option<String> {
    body["urls"]["get"].as_str().map(ToOwned::to_owned)
}
