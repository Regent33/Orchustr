#![allow(dead_code)]

use napi_derive::napi;

#[napi]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[napi]
pub fn render_prompt_json(template: String, context_json: String) -> napi::Result<String> {
    crate::render_prompt_json(&template, &context_json)
        .map_err(|error| napi::Error::from_reason(error.to_string()))
}

#[napi]
pub fn normalize_state_json(raw_state: String) -> napi::Result<String> {
    crate::normalize_state_json(&raw_state)
        .map_err(|error| napi::Error::from_reason(error.to_string()))
}

#[napi]
pub fn workspace_catalog_json() -> napi::Result<String> {
    crate::workspace_catalog_json().map_err(|error| napi::Error::from_reason(error.to_string()))
}

#[napi]
pub fn invoke_crate_json(
    crate_name: String,
    operation: String,
    payload_json: String,
) -> napi::Result<String> {
    crate::invoke_crate_json(&crate_name, &operation, &payload_json)
        .map_err(|error| napi::Error::from_reason(error.to_string()))
}
