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
