//! Prompt injection and template security tests.

use or_beacon::{BeaconError, PromptBuilder, PromptOrchestrator};
use serde_json::json;

#[test]
fn null_byte_injection_is_sanitized() {
    let template = PromptBuilder::new()
        .template("User said: {{input}}")
        .build()
        .unwrap();
    let rendered = PromptOrchestrator
        .render_template(&template, &json!({"input": "hi\0DROP TABLE"}))
        .unwrap();
    assert!(!rendered.contains('\0'), "null bytes must be stripped");
    assert!(rendered.contains("DROP TABLE"), "text content is preserved");
}

#[test]
fn template_injection_via_variable_value_does_not_expand() {
    // If the variable value itself contains {{other}}, it should be treated
    // as literal text and not re-expanded.
    let template = PromptBuilder::new()
        .template("Echo: {{msg}}")
        .build()
        .unwrap();
    let rendered = PromptOrchestrator
        .render_template(&template, &json!({"msg": "{{secret}}"}))
        .unwrap();
    assert_eq!(
        rendered, "Echo: {{secret}}",
        "template placeholders in values must not re-expand"
    );
}

#[test]
fn missing_template_is_rejected() {
    // Building without calling .template() should fail
    let result = PromptBuilder::new().build();
    assert!(
        result.is_err(),
        "building without template should be rejected"
    );
}

#[test]
fn empty_template_with_no_vars_is_valid() {
    // Empty string is technically a valid template (no variables)
    let template = PromptOrchestrator.build_template("").unwrap();
    assert!(template.variables.is_empty());
}

#[test]
fn unicode_variables_are_handled() {
    let template = PromptBuilder::new()
        .template("Translate: {{text}}")
        .build()
        .unwrap();
    let rendered = PromptOrchestrator
        .render_template(&template, &json!({"text": "こんにちは 🌍"}))
        .unwrap();
    assert_eq!(rendered, "Translate: こんにちは 🌍");
}

#[test]
fn multiple_variables_all_substituted() {
    let template = PromptBuilder::new()
        .template("{{a}} and {{b}} and {{c}}")
        .build()
        .unwrap();
    let rendered = PromptOrchestrator
        .render_template(
            &template,
            &json!({"a": "x", "b": "y", "c": "z"}),
        )
        .unwrap();
    assert_eq!(rendered, "x and y and z");
}

#[test]
fn missing_one_of_multiple_variables_fails() {
    let template = PromptBuilder::new()
        .template("{{a}} and {{b}}")
        .build()
        .unwrap();
    let result = PromptOrchestrator.render_template(&template, &json!({"a": "x"}));
    assert_eq!(result, Err(BeaconError::MissingVariable("b".to_owned())));
}
