use or_bridge::{BridgeError, normalize_state_json, render_prompt_json};

#[test]
fn render_prompt_json_renders_template_with_json_context() {
    let result = render_prompt_json("Hello {{name}}", "{\"name\":\"Ralph\"}").unwrap();
    assert_eq!(result, "Hello Ralph");
}

#[test]
fn render_prompt_json_rejects_invalid_json_context() {
    let result = render_prompt_json("Hello {{name}}", "[]");
    assert_eq!(result, Err(BridgeError::InvalidState));
}

#[test]
fn normalize_state_json_preserves_object_payloads() {
    let result = normalize_state_json("{\"count\":1}").unwrap();
    assert!(result.contains("\"count\":1"));
}

#[test]
fn normalize_state_json_rejects_non_object_payloads() {
    let result = normalize_state_json("[1,2,3]");
    assert_eq!(result, Err(BridgeError::InvalidState));
}

#[cfg(feature = "dart")]
mod dart;
