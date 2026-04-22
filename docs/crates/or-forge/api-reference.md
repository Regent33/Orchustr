# or-forge API Reference

This page documents the main public surface re-exported by `crates/or-forge/src/lib.rs`.

## `ForgeTool`

- Kind: `pub struct`
- File: `crates/or-forge/src/domain/entities.rs`
- Description: Named tool definition with description and input schema.

```rust
pub struct ForgeTool {
    pub name: String,
    pub description: String,
    pub input_schema: RootSchema,
}
```

## `ImportSummary`

- Kind: `pub struct`
- File: `crates/or-forge/src/application/orchestrators.rs`
- Description: Summary returned by additive MCP bulk import flows.

```rust
pub struct ImportSummary {
    pub tools_imported: usize,
    pub tool_names: Vec<String>,
    pub server_name: Option<String>,
}
```

## `ForgeRegistry`

- Kind: `pub struct`
- File: `crates/or-forge/src/application/orchestrators.rs`
- Description: Registry for local async handlers, imported MCP proxies, and additive bulk MCP discovery flows.

Key methods:

```rust
pub fn new() -> Self
pub fn register<F, Fut>(&mut self, tool: ForgeTool, handler: F) -> Result<(), ForgeError>
pub async fn import_from_mcp<C>(&mut self, client: &C) -> Result<usize, ForgeError>
pub async fn import_all_from_mcp(&mut self, server_url: &str) -> Result<ImportSummary, ForgeError>
pub async fn import_all_from_multi_mcp(&mut self, client: MultiMcpClient) -> Result<ImportSummary, ForgeError>
pub async fn import_all_from_multi_session(&mut self, session: MultiMcpSession) -> Result<ImportSummary, ForgeError>
pub async fn invoke(&self, name: &str, args: serde_json::Value) -> Result<serde_json::Value, ForgeError>
pub fn len(&self) -> usize
pub fn is_empty(&self) -> bool
```

## `ForgeError`

- Kind: `pub enum`
- File: `crates/or-forge/src/domain/errors.rs`
- Description: Error type for duplicate tools, invalid arguments, and invocation failures.

## Known Gaps & Limitations

- Schema validation is intentionally lightweight rather than exhaustive JSON Schema support.
- Multi-server MCP connection lives in `or-mcp`; `or-forge` only performs the registry adaptation layer.
