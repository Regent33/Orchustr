use crate::domain::entities::{
    CompletionMessage, CompletionResponse, ContentPart, FinishReason, MessageRole,
};
use crate::domain::errors::ConduitError;
use or_core::TokenUsage;
use serde_json::{Value, json};

/// Builds a Cohere Chat API payload.
pub(crate) fn cohere_payload(
    model: &str,
    messages: &[CompletionMessage],
) -> Result<Value, ConduitError> {
    let msgs = messages
        .iter()
        .map(cohere_message)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(json!({ "model": model, "messages": msgs }))
}

/// Parses a Cohere Chat API response.
pub(crate) fn parse_cohere_response(body: &Value) -> Result<CompletionResponse, ConduitError> {
    let text = body["message"]["content"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|part| part["text"].as_str())
        .collect::<String>();
    if text.is_empty() {
        return Err(ConduitError::Serialization(
            "Cohere response missing text".to_owned(),
        ));
    }
    let input_tokens =
        body["usage"]["billed_units"]["input_tokens"].as_u64().unwrap_or_default() as u32;
    let output_tokens =
        body["usage"]["billed_units"]["output_tokens"].as_u64().unwrap_or_default() as u32;
    let usage = TokenUsage {
        prompt_tokens: input_tokens,
        completion_tokens: output_tokens,
        total_tokens: input_tokens.saturating_add(output_tokens),
    };
    let finish_reason = match body["finish_reason"].as_str() {
        Some("MAX_TOKENS") => FinishReason::Length,
        _ => FinishReason::Stop,
    };
    Ok(CompletionResponse { text, usage, finish_reason })
}

fn cohere_message(message: &CompletionMessage) -> Result<Value, ConduitError> {
    let role = match message.role {
        MessageRole::System => "system",
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::Tool => {
            return Err(ConduitError::NotImplemented(
                "tool role not supported for Cohere".to_owned(),
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
