use crate::domain::entities::{
    CompletionMessage, CompletionResponse, ContentPart, FinishReason, MessageRole,
};
use crate::domain::errors::ConduitError;
use or_core::TokenUsage;
use serde_json::{Value, json};

/// Builds an AWS Bedrock `InvokeModel` payload (Anthropic Claude format).
/// The model ID is specified at the URL level, not in the payload.
pub(crate) fn bedrock_claude_payload(
    messages: &[CompletionMessage],
    max_tokens: u32,
) -> Result<Value, ConduitError> {
    let mut system_parts = Vec::new();
    let mut conversation = Vec::new();
    for message in messages {
        match message.role {
            MessageRole::System => {
                for part in &message.content {
                    if let ContentPart::Text { text } = part {
                        system_parts.push(json!({ "text": text }));
                    }
                }
            }
            MessageRole::User | MessageRole::Assistant => {
                let role = if matches!(message.role, MessageRole::User) {
                    "user"
                } else {
                    "assistant"
                };
                let content = bedrock_content(&message.content);
                conversation.push(json!({ "role": role, "content": content }));
            }
            MessageRole::Tool => {
                return Err(ConduitError::NotImplemented(
                    "tool role not supported for Bedrock".to_owned(),
                ));
            }
        }
    }
    Ok(json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": max_tokens,
        "system": system_parts,
        "messages": conversation,
    }))
}

/// Parses an AWS Bedrock response (Anthropic Claude format).
pub(crate) fn parse_bedrock_claude_response(
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
            "Bedrock response missing text".to_owned(),
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

fn bedrock_content(parts: &[ContentPart]) -> Vec<Value> {
    parts
        .iter()
        .map(|part| match part {
            ContentPart::Text { text } => json!({ "type": "text", "text": text }),
            ContentPart::Image { url, .. } => {
                json!({ "type": "image", "source": { "type": "base64", "media_type": "image/png", "data": url } })
            }
            ContentPart::Document { data, media_type } => {
                json!({ "type": "document", "source": { "type": "base64", "media_type": media_type, "data": data } })
            }
        })
        .collect()
}
