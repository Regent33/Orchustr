mod security;

use or_beacon::{BeaconError, PromptBuilder, PromptOrchestrator};
use serde_json::json;

#[tokio::test]
async fn build_template_extracts_variables() {
    let template = PromptOrchestrator
        .build_template("Summarize {{text}}")
        .unwrap();
    assert_eq!(template.variables, vec!["text"]);
}

#[tokio::test]
async fn build_template_rejects_invalid_variables() {
    let result = PromptOrchestrator.build_template("Summarize {{bad var}}");
    assert_eq!(
        result,
        Err(BeaconError::InvalidTemplate(
            "invalid variable name: bad var".to_owned()
        ))
    );
}

#[tokio::test]
async fn render_template_injects_sanitized_values() {
    let template = PromptBuilder::new()
        .template("Summarize {{text}}")
        .build()
        .unwrap();
    let rendered = PromptOrchestrator
        .render_template(&template, &json!({"text": "hi\u{0000}there"}))
        .unwrap();
    assert_eq!(rendered, "Summarize hithere");
}

#[tokio::test]
async fn render_template_rejects_missing_values() {
    let template = PromptBuilder::new()
        .template("Summarize {{text}}")
        .build()
        .unwrap();
    let result = PromptOrchestrator.render_template(&template, &json!({}));
    assert_eq!(result, Err(BeaconError::MissingVariable("text".to_owned())));
}
