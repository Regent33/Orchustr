use crate::domain::entities::{CompletionMessage, CompletionResponse, ContentPart, FinishReason, MessageRole};
use crate::domain::errors::ConduitError;
use or_core::TokenUsage;
use serde_json::{Value, json};

/// Builds a Google Gemini `generateContent` payload.
pub(crate) fn gemini_payload(
    messages: &[CompletionMessage],
) -> Result<Value, ConduitError> {
    let contents = messages
        .iter()
        .map(gemini_content)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(json!({ "contents": contents }))
}

/// Parses a Google Gemini `generateContent` response.
pub(crate) fn parse_gemini_response(body: &Value) -> Result<CompletionResponse, ConduitError> {
    let candidate = body["candidates"]
        .as_array()
        .and_then(|arr| arr.first())
        .ok_or_else(|| ConduitError::Serialization("missing candidates".to_owned()))?;
    let text = candidate["content"]["parts"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|part| part["text"].as_str())
        .collect::<String>();
    if text.is_empty() {
        return Err(ConduitError::Serialization(
            "Gemini response missing text".to_owned(),
        ));
    }
    let prompt_tokens =
        body["usageMetadata"]["promptTokenCount"].as_u64().unwrap_or_default() as u32;
    let completion_tokens =
        body["usageMetadata"]["candidatesTokenCount"].as_u64().unwrap_or_default() as u32;
    let usage = TokenUsage {
        prompt_tokens,
        completion_tokens,
        total_tokens: prompt_tokens.saturating_add(completion_tokens),
    };
    let finish_reason = match candidate["finishReason"].as_str() {
        Some("MAX_TOKENS") => FinishReason::Length,
        Some("SAFETY") => FinishReason::ContentFilter,
        _ => FinishReason::Stop,
    };
    Ok(CompletionResponse { text, usage, finish_reason })
}

fn gemini_content(message: &CompletionMessage) -> Result<Value, ConduitError> {
    let role = match message.role {
        MessageRole::User => "user",
        MessageRole::Assistant => "model",
        MessageRole::System => "user",
        MessageRole::Tool => {
            return Err(ConduitError::NotImplemented(
                "tool role not supported for Gemini".to_owned(),
            ));
        }
    };
    let parts = message
        .content
        .iter()
        .map(gemini_part)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(json!({ "role": role, "parts": parts }))
}

fn gemini_part(part: &ContentPart) -> Result<Value, ConduitError> {
    match part {
        ContentPart::Text { text } => Ok(json!({ "text": text })),
        ContentPart::Image { url, .. } => {
            Ok(json!({ "inlineData": { "mimeType": "image/png", "data": url } }))
        }
        ContentPart::Document { data, media_type } => {
            Ok(json!({ "inlineData": { "mimeType": media_type, "data": data } }))
        }
    }
}

