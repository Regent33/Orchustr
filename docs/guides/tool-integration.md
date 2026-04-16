# Tool Integration

`or-forge` is the current tool integration point for native Rust tools and imported MCP tools.

## Register a Local Tool

```rust
use or_forge::{ForgeRegistry, ForgeTool};
use schemars::schema_for;

# async fn example() -> anyhow::Result<()> {
let mut registry = ForgeRegistry::new();
let tool = ForgeTool {
    name: "echo".into(),
    description: "Echoes a message".into(),
    input_schema: schema_for!(serde_json::Value),
};
registry.register(tool, |args| async move { Ok(args) })?;
# Ok(()) }
```

## Import MCP Tools

```rust
use or_forge::ForgeRegistry;
use or_mcp::NexusClient;

# async fn example() -> anyhow::Result<()> {
let client = NexusClient::connect_http("https://example.test/mcp").await?;
let mut registry = ForgeRegistry::new();
registry.import_from_mcp(&client).await?;
# Ok(()) }
```

⚠️ Known Gaps & Limitations
- JSON Schema validation is intentionally lightweight and focused on repository use cases.
- No derive macro or annotation-based tool registration layer exists today.
