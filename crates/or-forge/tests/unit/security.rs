//! Tests for the forge arg size guard and tool registration edge cases.

use or_forge::{ForgeError, ForgeRegistry, ForgeTool};
use schemars::schema::RootSchema;

#[tokio::test]
async fn invoke_rejects_oversized_arguments() {
    let mut registry = ForgeRegistry::new();
    registry
        .register(
            ForgeTool {
                name: "big".to_owned(),
                description: "Accepts big payloads".to_owned(),
                input_schema: schema_object(),
            },
            |args| async move { Ok(serde_json::json!({ "echo": args })) },
        )
        .unwrap();
    // Build a payload larger than 1MB
    let big_value = "x".repeat(1_100_000);
    let result = registry
        .invoke("big", serde_json::json!({ "data": big_value }))
        .await;
    assert!(
        matches!(result, Err(ForgeError::InvalidArguments(ref msg)) if msg.contains("too large")),
        "expected oversized payload rejection, got: {result:?}"
    );
}

#[tokio::test]
async fn register_duplicate_tool_fails() {
    let mut registry = ForgeRegistry::new();
    registry
        .register(
            ForgeTool {
                name: "dup".to_owned(),
                description: "First".to_owned(),
                input_schema: schema_object(),
            },
            |_| async move { Ok(serde_json::json!({})) },
        )
        .unwrap();
    let result = registry.register(
        ForgeTool {
            name: "dup".to_owned(),
            description: "Second".to_owned(),
            input_schema: schema_object(),
        },
        |_| async move { Ok(serde_json::json!({})) },
    );
    assert!(result.is_err(), "duplicate tool registration should fail");
}

#[tokio::test]
async fn registry_tracks_tool_count() {
    let mut registry = ForgeRegistry::new();
    assert!(registry.is_empty());
    registry
        .register(
            ForgeTool {
                name: "alpha".to_owned(),
                description: "A".to_owned(),
                input_schema: schema_object(),
            },
            |_| async move { Ok(serde_json::json!({})) },
        )
        .unwrap();
    registry
        .register(
            ForgeTool {
                name: "beta".to_owned(),
                description: "B".to_owned(),
                input_schema: schema_object(),
            },
            |_| async move { Ok(serde_json::json!({})) },
        )
        .unwrap();
    assert_eq!(registry.len(), 2);
    assert!(!registry.is_empty());
}

fn schema_object() -> RootSchema {
    schemars::schema::RootSchema {
        meta_schema: None,
        schema: schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
                schemars::schema::InstanceType::Object,
            ))),
            ..Default::default()
        },
        definitions: Default::default(),
    }
}
