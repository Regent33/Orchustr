//! Structured parsing security and edge case tests.

use or_sieve::{JsonParser, SieveError, SieveOrchestrator, TextParser};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
struct NumberOutput {
    count: u32,
}

#[test]
fn parse_structured_wrong_type_returns_schema_violation() {
    let parser = JsonParser::<NumberOutput>::new();
    // count is a string instead of u32
    let result = SieveOrchestrator.parse_structured(&parser, r#"{"count":"abc"}"#);
    assert!(
        matches!(result, Err(SieveError::SchemaViolation { .. })),
        "type mismatch should yield SchemaViolation: {result:?}"
    );
}

#[test]
fn parse_structured_rejects_invalid_json() {
    let parser = JsonParser::<NumberOutput>::new();
    let result = SieveOrchestrator.parse_structured(&parser, "not json at all");
    assert!(result.is_err(), "invalid JSON must be rejected");
}

#[test]
fn parse_structured_accepts_valid_json() {
    let parser = JsonParser::<NumberOutput>::new();
    let result = SieveOrchestrator
        .parse_structured(&parser, r#"{"count":42}"#)
        .unwrap();
    assert_eq!(result, NumberOutput { count: 42 });
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
struct NestedOutput {
    inner: InnerOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
struct InnerOutput {
    value: String,
}

#[test]
fn parse_nested_schema_validates_inner_fields() {
    let parser = JsonParser::<NestedOutput>::new();
    // inner.value is missing — serde reports this as a deserialization error
    let result = SieveOrchestrator.parse_structured(&parser, r#"{"inner":{}}"#);
    assert!(
        result.is_err(),
        "nested required field should be flagged"
    );
    let err_msg = format!("{:?}", result.unwrap_err());
    assert!(
        err_msg.contains("value"),
        "error should reference the missing 'value' field: {err_msg}"
    );
}

#[test]
fn parse_text_with_only_whitespace_is_rejected() {
    let parser = TextParser;
    let result = SieveOrchestrator.parse_text(&parser, "\t\n  \r");
    assert_eq!(result, Err(SieveError::EmptyText));
}

#[test]
fn parse_text_preserves_inner_whitespace() {
    let parser = TextParser;
    let result = SieveOrchestrator
        .parse_text(&parser, "  hello  world  ")
        .unwrap();
    assert_eq!(result.text, "hello  world");
}
