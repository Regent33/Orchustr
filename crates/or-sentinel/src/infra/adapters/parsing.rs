use crate::domain::entities::{PlanStep, SentinelConfig};
use crate::domain::errors::SentinelError;
use or_conduit::{CompletionMessage, ContentPart};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum ModelDecision {
    ToolCall {
        tool_name: String,
        args: serde_json::Value,
    },
    FinalAnswer {
        answer: String,
    },
}

pub(crate) fn approx_prompt_tokens(messages: &[CompletionMessage]) -> u32 {
    messages
        .iter()
        .flat_map(|message| &message.content)
        .map(|part| match part {
            ContentPart::Text { text } => (text.len() as u32 / 4).max(1),
            ContentPart::Image { .. } | ContentPart::Document { .. } => 256,
        })
        .sum()
}

pub(crate) fn parse_decision(raw: &str) -> Result<ModelDecision, SentinelError> {
    const MAX_DECISION_BYTES: usize = 64 * 1024; // 64 KB
    if raw.len() > MAX_DECISION_BYTES {
        return Err(SentinelError::InvalidResponse(format!(
            "decision payload too large: {} bytes (max {MAX_DECISION_BYTES})",
            raw.len()
        )));
    }
    if let Ok(decision) = serde_json::from_str::<ModelDecision>(raw) {
        return Ok(decision);
    }
    let value: serde_json::Value = serde_json::from_str(raw)
        .map_err(|error| SentinelError::InvalidResponse(error.to_string()))?;
    if let Some(answer) = value.get("final_answer").and_then(|item| item.as_str()) {
        return Ok(ModelDecision::FinalAnswer {
            answer: answer.to_owned(),
        });
    }
    if let Some(tool_name) = value.get("tool_name").and_then(|item| item.as_str()) {
        return Ok(ModelDecision::ToolCall {
            tool_name: tool_name.to_owned(),
            args: value
                .get("args")
                .cloned()
                .unwrap_or_else(|| serde_json::json!({})),
        });
    }
    Err(SentinelError::InvalidResponse(
        "response did not contain a final answer or tool call".to_owned(),
    ))
}

pub(crate) fn parse_plan(raw: &str) -> Result<Vec<PlanStep>, SentinelError> {
    let value: serde_json::Value = serde_json::from_str(raw)
        .map_err(|error| SentinelError::InvalidResponse(error.to_string()))?;
    let steps = value
        .get("steps")
        .and_then(|item| item.as_array())
        .ok_or_else(|| SentinelError::InvalidResponse("plan steps missing".to_owned()))?;
    Ok(steps
        .iter()
        .enumerate()
        .filter_map(|(index, step)| {
            step.as_str().map(|description| PlanStep {
                step_index: index as u32 + 1,
                description: description.to_owned(),
            })
        })
        .collect())
}

pub(crate) fn config_to_value(config: &SentinelConfig) -> Result<serde_json::Value, SentinelError> {
    serde_json::to_value(config).map_err(|error| SentinelError::Serialization(error.to_string()))
}
