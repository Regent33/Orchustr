mod security;

use or_forge::{ForgeError, ForgeRegistry, ForgeTool};
use or_mcp::{McpTool, NexusClientTrait};
use schemars::schema::RootSchema;

#[derive(Clone)]
struct MockClient;

impl NexusClientTrait for MockClient {
    async fn send(
        &self,
        _method: &str,
        _params: serde_json::Value,
    ) -> Result<serde_json::Value, or_mcp::McpError> {
        Ok(serde_json::json!({}))
    }

    async fn list_tools(&self) -> Result<Vec<McpTool>, or_mcp::McpError> {
        Ok(vec![McpTool {
            name: "echo".to_owned(),
            description: "Echo arguments".to_owned(),
            input_schema: schema_object(),
        }])
    }

    async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<serde_json::Value, or_mcp::McpError> {
        Ok(serde_json::json!({ "tool": name, "args": args }))
    }
}

#[tokio::test]
async fn register_and_invoke_local_tool() {
    let mut registry = ForgeRegistry::new();
    registry
        .register(
            ForgeTool {
                name: "sum".to_owned(),
                description: "Adds numbers".to_owned(),
                input_schema: schema_object(),
            },
            |args| async move { Ok(serde_json::json!({ "echo": args })) },
        )
        .unwrap();
    let result = registry
        .invoke("sum", serde_json::json!({ "value": 1 }))
        .await
        .unwrap();
    assert_eq!(result["echo"]["value"], 1);
}

#[tokio::test]
async fn import_from_mcp_registers_proxy_tools() {
    let mut registry = ForgeRegistry::new();
    registry.import_from_mcp(&MockClient).await.unwrap();
    let result = registry
        .invoke("echo", serde_json::json!({ "value": 2 }))
        .await
        .unwrap();
    assert_eq!(result["tool"], "echo");
}

#[tokio::test]
async fn invoke_rejects_unknown_tools() {
    let registry = ForgeRegistry::new();
    let result = registry.invoke("missing", serde_json::json!({})).await;
    assert_eq!(result, Err(ForgeError::UnknownTool("missing".to_owned())));
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
