use crate::domain::errors::BridgeError;
use crate::infra::implementations::{normalize, render};

pub fn render_prompt_json(template: &str, context_json: &str) -> Result<String, BridgeError> {
    let span = tracing::info_span!(
        "bridge.render_prompt_json",
        otel.name = "bridge.render_prompt_json",
        status = tracing::field::Empty
    );
    let _guard = span.enter();
    let result = render(template, context_json);
    span.record("status", if result.is_ok() { "success" } else { "failure" });
    result
}

pub fn normalize_state_json(raw_state: &str) -> Result<String, BridgeError> {
    let span = tracing::info_span!(
        "bridge.normalize_state_json",
        otel.name = "bridge.normalize_state_json",
        status = tracing::field::Empty
    );
    let _guard = span.enter();
    let result = normalize(raw_state);
    span.record("status", if result.is_ok() { "success" } else { "failure" });
    result
}
