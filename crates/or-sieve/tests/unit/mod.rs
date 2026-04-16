mod validation;

use or_sieve::{JsonParser, SieveError, SieveOrchestrator, TextParser};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
struct SummaryOutput {
    text: String,
}

#[tokio::test]
async fn parse_structured_returns_typed_output() {
    let parser = JsonParser::<SummaryOutput>::new();
    let result = SieveOrchestrator
        .parse_structured(&parser, r#"{"text":"done"}"#)
        .unwrap();
    assert_eq!(
        result,
        SummaryOutput {
            text: "done".to_owned()
        }
    );
}

#[tokio::test]
async fn parse_structured_rejects_missing_required_fields() {
    let parser = JsonParser::<SummaryOutput>::new();
    let result = SieveOrchestrator.parse_structured(&parser, r#"{}"#);
    assert_eq!(
        result,
        Err(SieveError::SchemaViolation {
            path: "$.text".to_owned(),
            message: "missing required property".to_owned(),
        })
    );
}

#[tokio::test]
async fn parse_text_returns_trimmed_output() {
    let parser = TextParser;
    let result = SieveOrchestrator.parse_text(&parser, "  hello  ").unwrap();
    assert_eq!(result.text, "hello");
}

#[tokio::test]
async fn parse_text_rejects_empty_output() {
    let parser = TextParser;
    let result = SieveOrchestrator.parse_text(&parser, "   ");
    assert_eq!(result, Err(SieveError::EmptyText));
}
