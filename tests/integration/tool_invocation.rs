use or_forge::{ForgeError, ForgeRegistry, ForgeTool};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct EchoArgs {
    message: String,
}

#[tokio::test]
async fn forge_registry_invokes_registered_tool_with_schema_validation() {
    let mut registry = ForgeRegistry::new();
    registry
        .register(
            ForgeTool {
                name: "echo".into(),
                description: "Echoes a message".into(),
                input_schema: schema_for!(EchoArgs),
            },
            |args| async move { Ok(serde_json::json!({ "echo": args["message"] })) },
        )
        .expect("tool should register");

    let result = registry
        .invoke("echo", serde_json::json!({ "message": "hello" }))
        .await
        .expect("tool should execute");

    assert_eq!(result["echo"], "hello");
}

#[tokio::test]
async fn forge_registry_rejects_invalid_arguments_before_invocation() {
    let mut registry = ForgeRegistry::new();
    registry
        .register(
            ForgeTool {
                name: "echo".into(),
                description: "Echoes a message".into(),
                input_schema: schema_for!(EchoArgs),
            },
            |_args| async move { Ok(serde_json::json!({ "ok": true })) },
        )
        .expect("tool should register");

    let error = registry
        .invoke("echo", serde_json::json!({}))
        .await
        .expect_err("missing required fields should fail");

    assert!(matches!(error, ForgeError::InvalidArguments(_)));
}
