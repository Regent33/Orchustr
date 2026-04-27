use crate::domain::entities::{
    CompletionMessage, CompletionResponse, ContentPart, FinishReason, ImageDetail, MessageRole,
};
use crate::domain::errors::ConduitError;
use or_core::TokenUsage;
use serde_json::{Value, json};

/// Estimates prompt token count by walking content parts directly.
pub(crate) fn estimate_prompt_tokens(messages: &[CompletionMessage]) -> u32 {
    messages
        .iter()
        .flat_map(|msg| &msg.content)
        .map(|part| match part {
            ContentPart::Text { text } => (text.len() as u32 / 4).max(1),
            ContentPart::Image { .. } => 256,
            ContentPart::Document { data, .. } => (data.len() as u32 / 4).max(1),
        })
        .sum::<u32>()
        .max(1)
}

pub(crate) fn openai_payload(
    model: &str,
    messages: &[CompletionMessage],
    max_output_tokens: u32,
) -> Result<Value, ConduitError> {
    let input = messages
        .iter()
        .map(openai_message)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(json!({ "model": model, "input": input, "max_output_tokens": max_output_tokens }))
}

pub(crate) fn parse_openai_response(body: &Value) -> Result<CompletionResponse, ConduitError> {
    let text = extract_openai_text(body)?;
    let usage = TokenUsage {
        prompt_tokens: body["usage"]["input_tokens"].as_u64().unwrap_or_default() as u32,
        completion_tokens: body["usage"]["output_tokens"].as_u64().unwrap_or_default() as u32,
        total_tokens: body["usage"]["total_tokens"].as_u64().unwrap_or_default() as u32,
    };
    let finish_reason = parse_openai_finish_reason(body);
    Ok(CompletionResponse {
        text,
        usage,
        finish_reason,
    })
}

/// Builds an OpenAI chat completions payload (for `/v1/chat/completions`).
pub(crate) fn openai_chat_payload(
    model: &str,
    messages: &[CompletionMessage],
    max_tokens: u32,
) -> Result<Value, ConduitError> {
    let msgs = messages
        .iter()
        .map(openai_chat_message)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(json!({ "model": model, "messages": msgs, "max_tokens": max_tokens }))
}

/// Parses a standard OpenAI chat completions response.
pub(crate) fn parse_openai_chat_response(body: &Value) -> Result<CompletionResponse, ConduitError> {
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
            "empty response content".to_owned(),
        ));
    }
    let usage = TokenUsage {
        prompt_tokens: body["usage"]["prompt_tokens"].as_u64().unwrap_or_default() as u32,
        completion_tokens: body["usage"]["completion_tokens"]
            .as_u64()
            .unwrap_or_default() as u32,
        total_tokens: body["usage"]["total_tokens"].as_u64().unwrap_or_default() as u32,
    };
    let finish_reason = match choice["finish_reason"].as_str() {
        Some("length") => FinishReason::Length,
        Some("tool_calls") => FinishReason::ToolCall,
        Some("content_filter") => FinishReason::ContentFilter,
        _ => FinishReason::Stop,
    };
    Ok(CompletionResponse {
        text,
        usage,
        finish_reason,
    })
}

fn openai_message(message: &CompletionMessage) -> Result<Value, ConduitError> {
    let role = message_role_str(&message.role)?;
    let content = message
        .content
        .iter()
        .map(openai_content)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(json!({ "type": "message", "role": role, "content": content }))
}

fn openai_chat_message(message: &CompletionMessage) -> Result<Value, ConduitError> {
    let role = message_role_str(&message.role)?;
    let content = openai_chat_content(&message.content)?;
    Ok(json!({ "role": role, "content": content }))
}

/// Serializes content for the Chat Completions API (`/v1/chat/completions`).
/// Uses a flat string for single-text messages (maximally compatible with all providers),
/// and an array with correct type tags for multimodal messages.
fn openai_chat_content(parts: &[ContentPart]) -> Result<Value, ConduitError> {
    if let [ContentPart::Text { text }] = parts {
        return Ok(Value::String(text.clone()));
    }
    parts
        .iter()
        .map(openai_chat_part)
        .collect::<Result<Vec<_>, _>>()
        .map(Value::Array)
}

fn openai_chat_part(part: &ContentPart) -> Result<Value, ConduitError> {
    match part {
        ContentPart::Text { text } => Ok(json!({ "type": "text", "text": text })),
        ContentPart::Image { url, detail } => Ok(json!({
            "type": "image_url",
            "image_url": { "url": url, "detail": openai_detail(detail) }
        })),
        ContentPart::Document { data, media_type } => Ok(json!({
            "type": "text",
            "text": format!("[document/{media_type}]\n{data}")
        })),
    }
}

fn message_role_str(role: &MessageRole) -> Result<&'static str, ConduitError> {
    match role {
        MessageRole::System => Ok("system"),
        MessageRole::User => Ok("user"),
        MessageRole::Assistant => Ok("assistant"),
        MessageRole::Tool => Err(ConduitError::NotImplemented(
            "tool role is not wired in Phase 1".to_owned(),
        )),
    }
}

fn openai_content(part: &ContentPart) -> Result<Value, ConduitError> {
    match part {
        ContentPart::Text { text } => Ok(json!({ "type": "input_text", "text": text })),
        ContentPart::Image { url, detail } => {
            Ok(json!({ "type": "input_image", "image_url": url, "detail": openai_detail(detail) }))
        }
        ContentPart::Document { data, media_type } => Ok(json!({
            "type": "input_file",
            "file_data": data,
            "filename": document_filename(media_type),
        })),
    }
}

fn openai_detail(detail: &ImageDetail) -> &'static str {
    match detail {
        ImageDetail::Auto => "auto",
        ImageDetail::Low => "low",
        ImageDetail::High => "high",
    }
}

pub(crate) fn document_filename(media_type: &str) -> String {
    let extension = media_type.split('/').next_back().unwrap_or("bin");
    format!("document.{extension}")
}

fn extract_openai_text(body: &Value) -> Result<String, ConduitError> {
    let text = body["output"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|item| item["type"].as_str() == Some("message"))
        .flat_map(|item| item["content"].as_array().into_iter().flatten())
        .filter(|part| part["type"].as_str() == Some("output_text"))
        .filter_map(|part| part["text"].as_str())
        .collect::<String>();
    if text.is_empty() {
        Err(ConduitError::Serialization(
            "OpenAI response missing output text".to_owned(),
        ))
    } else {
        Ok(text)
    }
}

fn parse_openai_finish_reason(body: &Value) -> FinishReason {
    let reason = body["output"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|item| item["type"].as_str() == Some("message"))
        .find_map(|item| item["status"].as_str());
    match reason {
        Some("incomplete") => FinishReason::Length,
        _ => FinishReason::Stop,
    }
}
