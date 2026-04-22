use crate::domain::errors::BridgeError;
use crate::infra::adapters::{value_from_json, value_to_json};
use crate::infra::facades::{invoke, workspace_catalog};
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

pub fn workspace_catalog_json() -> Result<String, BridgeError> {
    let span = tracing::info_span!(
        "bridge.workspace_catalog_json",
        otel.name = "bridge.workspace_catalog_json",
        status = tracing::field::Empty
    );
    let _guard = span.enter();
    let result = workspace_catalog();
    span.record("status", if result.is_ok() { "success" } else { "failure" });
    result
}

pub fn invoke_crate_json(
    crate_name: &str,
    operation: &str,
    payload_json: &str,
) -> Result<String, BridgeError> {
    let span = tracing::info_span!(
        "bridge.invoke_crate_json",
        otel.name = "bridge.invoke_crate_json",
        crate_name,
        operation,
        status = tracing::field::Empty
    );
    let _guard = span.enter();
    let result = value_from_json(payload_json)
        .and_then(|payload| invoke(crate_name, operation, payload))
        .and_then(|value| value_to_json(&value));
    span.record("status", if result.is_ok() { "success" } else { "failure" });
    result
}
