//! Bridge entry points for the small "native" crates that don't need
//! per-provider configuration: `or-core`, `or-beacon`, `or-bridge`
//! itself, `or-conduit`, `or-prism`, `or-sieve`.

use super::helpers::{from_field, invocation, required_str, required_u64, unsupported};
use crate::domain::errors::BridgeError;
use or_beacon::PromptOrchestrator;
use or_conduit::ConduitOrchestrator;
use or_core::{CoreOrchestrator, RetryPolicy, TokenBudget};
use or_prism::install_global_subscriber;
use or_sieve::{SieveOrchestrator, TextParser};
use serde_json::{Value, json};

pub(crate) fn invoke_core(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    let orchestrator = CoreOrchestrator::new();
    match operation {
        "enforce_completion_budget" => {
            let budget: TokenBudget = from_field(&payload, "budget", "or-core", operation)?;
            let prompt_tokens =
                required_u64(&payload, "prompt_tokens", "or-core", operation)? as u32;
            orchestrator
                .enforce_completion_budget(&budget, prompt_tokens)
                .map_err(|error| invocation("or-core", operation, error))?;
            Ok(json!({ "status": "ok" }))
        }
        "next_retry_delay" => {
            let policy: RetryPolicy = from_field(&payload, "policy", "or-core", operation)?;
            let attempt = required_u64(&payload, "attempt", "or-core", operation)? as u32;
            let delay = orchestrator
                .next_retry_delay(&policy, attempt)
                .map_err(|error| invocation("or-core", operation, error))?;
            Ok(json!({ "delay_ms": delay.as_millis() as u64 }))
        }
        _ => Err(unsupported("or-core", operation)),
    }
}

pub(crate) fn invoke_beacon(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    let orchestrator = PromptOrchestrator;
    match operation {
        "render_template" => {
            let template = required_str(&payload, "template", "or-beacon", operation)?;
            let context = payload
                .get("context")
                .cloned()
                .unwrap_or_else(|| Value::Object(Default::default()));
            let built = orchestrator
                .build_template(template)
                .map_err(|error| invocation("or-beacon", operation, error))?;
            let rendered = orchestrator
                .render_template(&built, &context)
                .map_err(|error| invocation("or-beacon", operation, error))?;
            Ok(json!({ "text": rendered }))
        }
        _ => Err(unsupported("or-beacon", operation)),
    }
}

pub(crate) fn invoke_bridge(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    match operation {
        "render_prompt_json" => {
            let template = required_str(&payload, "template", "or-bridge", operation)?;
            let context = payload
                .get("context")
                .cloned()
                .unwrap_or_else(|| Value::Object(Default::default()));
            let rendered = crate::render_prompt_json(
                template,
                &serde_json::to_string(&context)
                    .map_err(|error| BridgeError::InvalidJson(error.to_string()))?,
            )?;
            Ok(json!({ "text": rendered }))
        }
        "normalize_state_json" => {
            let state = payload
                .get("state")
                .cloned()
                .unwrap_or_else(|| Value::Object(Default::default()));
            let normalized = crate::normalize_state_json(
                &serde_json::to_string(&state)
                    .map_err(|error| BridgeError::InvalidJson(error.to_string()))?,
            )?;
            Ok(json!({
                "state": serde_json::from_str::<Value>(&normalized)
                    .map_err(|error| BridgeError::InvalidJson(error.to_string()))?,
            }))
        }
        _ => Err(unsupported("or-bridge", operation)),
    }
}

pub(crate) fn invoke_conduit(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    match operation {
        "prepare_text_request" => {
            let prompt = required_str(&payload, "prompt", "or-conduit", operation)?;
            let messages = ConduitOrchestrator
                .prepare_text_request(prompt)
                .map_err(|error| invocation("or-conduit", operation, error))?;
            serde_json::to_value(messages)
                .map_err(|error| BridgeError::InvalidJson(error.to_string()))
        }
        _ => Err(unsupported("or-conduit", operation)),
    }
}

pub(crate) fn invoke_prism(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    match operation {
        "install_global_subscriber" => {
            let endpoint = required_str(&payload, "otlp_endpoint", "or-prism", operation)?;
            install_global_subscriber(endpoint)
                .map_err(|error| invocation("or-prism", operation, error))?;
            Ok(json!({ "status": "ok" }))
        }
        _ => Err(unsupported("or-prism", operation)),
    }
}

pub(crate) fn invoke_sieve(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    match operation {
        "parse_text" => {
            let raw = required_str(&payload, "raw", "or-sieve", operation)?;
            let parsed = SieveOrchestrator
                .parse_text(&TextParser, raw)
                .map_err(|error| invocation("or-sieve", operation, error))?;
            serde_json::to_value(parsed)
                .map_err(|error| BridgeError::InvalidJson(error.to_string()))
        }
        _ => Err(unsupported("or-sieve", operation)),
    }
}
