# or-mcp API Reference

This page documents the main public surface re-exported by `crates/or-mcp/src/lib.rs`.

## Core Runtime

- `NexusClient<T: McpTransport>`: transport-parameterized MCP client with HTTP and stdio constructors.
- `NexusServer<T: McpTransport>`: transport-driven MCP server runtime for tool registration and request handling.
- `NexusClientTrait`: transport-agnostic client contract.
- `NexusServerTrait`: transport-agnostic server contract.
- `McpTransport`: abstract send/receive transport used by the client and server runtimes.
- `StreamableHttpTransport`: reqwest-backed transport for MCP over streamable HTTP, including optional bearer-token support through `with_bearer_token(...)`.
- `StdioTransport`: subprocess-backed stdio transport for local MCP hosts.
- `JsonRpcOrchestrator`: wrapper around JSON-RPC encoding and decoding helpers.

## Additive Multi-Server Surface

- `McpServerTransport`: typed configuration enum for HTTP or stdio MCP servers.
- `McpServerConfig`: connection settings consumed by `MultiMcpClient`.
- `DiscoveredMcpTool`: resolved MCP tool plus server metadata and collision-safe registered name.
- `MultiMcpClient`: additive multi-server builder.
- `MultiMcpSession`: merged session returned by `MultiMcpClient::connect_all`.

Key methods:

```rust
pub fn new() -> Self
pub fn add_server(self, config: McpServerConfig) -> Self
pub async fn connect_all(self) -> Result<MultiMcpSession, McpError>
pub fn tools(&self) -> &[DiscoveredMcpTool]
pub async fn invoke(&self, registered_name: &str, args: serde_json::Value) -> Result<serde_json::Value, McpError>
```

## Presets and Entities

- `known_servers::known::*`: curated MCP server presets for filesystem, Brave Search, GitHub, Slack, and Postgres.
- `McpTool`, `McpTask`, `ServerCard`: primary MCP domain entities exposed by the crate.
- `McpError`: protocol, transport, auth, task, and tool-execution error type.

## Known Gaps & Limitations

- The server runtime is transport-driven and does not yet expose its own standalone HTTP listener.
- `or-mcp` intentionally stops at merged discovery and invocation; adapting merged tools into `ForgeRegistry` happens in `or-forge`.
