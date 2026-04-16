use crate::domain::entities::{
    CompletionMessage, CompletionResponse, ContentPart, FinishReason, MessageRole,
};
use crate::domain::errors::ConduitError;
use or_core::TokenUsage;
use serde_json::{Value, json};

/// Builds an AI21 Jamba API payload.
pub(crate) fn ai21_payload(
    model: &str,
    messages: &[CompletionMessage],
    max_tokens: u32,
) -> Result<Value, ConduitError> {
    let msgs = messages
        .iter()
        .map(ai21_message)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(json!({ "model": model, "messages": msgs, "max_tokens": max_tokens }))
}

/// Parses an AI21 Jamba API response.
pub(crate) fn parse_ai21_response(body: &Value) -> Result<CompletionResponse, ConduitError> {
    let choice = body["choices"]
        .as_array()
        .and_then(|arr| arr.first())
        .ok_or_else(|| ConduitError::Serialization("missing choices array".to_owned()))?;
    let text = choice["message"]["content"]
        .as_str()
        .unwrap_or_default()
        .to_owned();
    if text.is_empty() {
        return Err(ConduitError::Serialization(
            "AI21 response missing text".to_owned(),
        ));
    }
    let usage = TokenUsage {
        prompt_tokens: body["usage"]["prompt_tokens"].as_u64().unwrap_or_default() as u32,
        completion_tokens: body["usage"]["completion_tokens"].as_u64().unwrap_or_default() as u32,
        total_tokens: body["usage"]["total_tokens"].as_u64().unwrap_or_default() as u32,
    };
    let finish_reason = match choice["finish_reason"].as_str() {
        Some("length") => FinishReason::Length,
        _ => FinishReason::Stop,
    };
    Ok(CompletionResponse { text, usage, finish_reason })
}

fn ai21_message(message: &CompletionMessage) -> Result<Value, ConduitError> {
    let role = match message.role {
        MessageRole::System => "system",
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::Tool => {
            return Err(ConduitError::NotImplemented(
                "tool role not supported for AI21".to_owned(),
            ));
        }
    };
    let text = message
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    Ok(json!({ "role": role, "content": text }))
}
