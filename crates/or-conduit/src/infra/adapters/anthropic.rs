use crate::domain::entities::{
    CompletionMessage, CompletionResponse, ContentPart, FinishReason, MessageRole,
};
use crate::domain::errors::ConduitError;
use or_core::TokenUsage;
use serde_json::{Value, json};

pub(crate) fn anthropic_payload(
    model: &str,
    messages: &[CompletionMessage],
    max_tokens: u32,
) -> Result<Value, ConduitError> {
    let mut system_parts = Vec::new();
    let mut conversation = Vec::new();
    for message in messages {
        match message.role {
            MessageRole::System => system_parts.extend(anthropic_content(&message.content)?),
            MessageRole::User | MessageRole::Assistant => {
                conversation.push(json!({
                    "role": anthropic_role(&message.role),
                    "content": anthropic_content(&message.content)?,
                }));
            }
            MessageRole::Tool => {
                return Err(ConduitError::NotImplemented(
                    "tool role is not wired in Phase 1".to_owned(),
                ));
            }
        }
    }
    Ok(json!({
        "model": model,
        "max_tokens": max_tokens,
        "system": system_parts,
        "messages": conversation,
    }))
}

pub(crate) fn parse_anthropic_response(
    body: &Value,
) -> Result<CompletionResponse, ConduitError> {
    let text = body["content"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|part| part["type"].as_str() == Some("text"))
        .filter_map(|part| part["text"].as_str())
        .collect::<String>();
    if text.is_empty() {
        return Err(ConduitError::Serialization(
            "Anthropic response missing text output".to_owned(),
        ));
    }
    let input_tokens = body["usage"]["input_tokens"].as_u64().unwrap_or_default() as u32;
    let output_tokens = body["usage"]["output_tokens"].as_u64().unwrap_or_default() as u32;
    let usage = TokenUsage {
        prompt_tokens: input_tokens,
        completion_tokens: output_tokens,
        total_tokens: input_tokens.saturating_add(output_tokens),
    };
    let finish_reason = match body["stop_reason"].as_str() {
        Some("max_tokens") => FinishReason::Length,
        Some("tool_use") => FinishReason::ToolCall,
        _ => FinishReason::Stop,
    };
    Ok(CompletionResponse { text, usage, finish_reason })
}

fn anthropic_content(parts: &[ContentPart]) -> Result<Vec<Value>, ConduitError> {
    parts
        .iter()
        .map(|part| match part {
            ContentPart::Text { text } => Ok(json!({ "type": "text", "text": text })),
            ContentPart::Image { url, .. } => {
                Ok(json!({ "type": "image", "source": { "type": "url", "url": url } }))
            }
            ContentPart::Document { data, media_type } => Ok(json!({
                "type": "document",
                "source": { "type": "base64", "media_type": media_type, "data": data }
            })),
        })
        .collect()
}

fn anthropic_role(role: &MessageRole) -> &'static str {
    match role {
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::System | MessageRole::Tool => "user",
    }
}
