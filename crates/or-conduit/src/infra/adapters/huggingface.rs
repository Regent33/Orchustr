use crate::domain::entities::{
    CompletionMessage, CompletionResponse, ContentPart, FinishReason, MessageRole,
};
use crate::domain::errors::ConduitError;
use or_core::TokenUsage;
use serde_json::{Value, json};

/// Builds a HuggingFace Inference API payload.
pub(crate) fn huggingface_payload(
    messages: &[CompletionMessage],
    max_tokens: u32,
) -> Result<Value, ConduitError> {
    let inputs = messages
        .iter()
        .map(hf_message)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(json!({
        "inputs": inputs,
        "parameters": { "max_new_tokens": max_tokens },
    }))
}

/// Parses a HuggingFace Inference API response.
pub(crate) fn parse_huggingface_response(
    body: &Value,
) -> Result<CompletionResponse, ConduitError> {
    // HF Inference returns either an array or a single object.
    let text = if let Some(arr) = body.as_array() {
        arr.first()
            .and_then(|item| item["generated_text"].as_str())
            .unwrap_or_default()
            .to_owned()
    } else {
        body["generated_text"]
            .as_str()
            .unwrap_or_default()
            .to_owned()
    };
    if text.is_empty() {
        return Err(ConduitError::Serialization(
            "HuggingFace response missing generated_text".to_owned(),
        ));
    }
    // HF Inference does not typically return token usage, so we estimate.
    let estimated_tokens = (text.len() as u32 / 4).max(1);
    let usage = TokenUsage {
        prompt_tokens: 0,
        completion_tokens: estimated_tokens,
        total_tokens: estimated_tokens,
    };
    Ok(CompletionResponse {
        text,
        usage,
        finish_reason: FinishReason::Stop,
    })
}

fn hf_message(message: &CompletionMessage) -> Result<Value, ConduitError> {
    let role = match message.role {
        MessageRole::System => "system",
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::Tool => {
            return Err(ConduitError::NotImplemented(
                "tool role not supported for HuggingFace".to_owned(),
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
