# or-mcp

**Status**: Partial | **Version**: `0.1.2` | **Deps**: reqwest, schemars, serde, serde_json, thiserror, tokio, tracing

Model Context Protocol crate implementing JSON-RPC message types, streamable HTTP and stdio transports, transport-driven client/server orchestration, and additive multi-server discovery helpers.

## Position in the Workspace

```mermaid
graph LR
  OR_CORE[or-core] --> THIS[or-mcp]
  THIS --> CALLERS[Callers]
```

## Implementation Status

| Component | Status | Notes |
|---|---|---|
| Protocol model | Complete | JSON-RPC and MCP entities are implemented and re-exported. |
| Transports | Complete | Streamable HTTP and stdio transports are implemented and tested through scripted transports. |
| Multi-server discovery | Complete | `MultiMcpClient` merges multiple servers concurrently and prefixes duplicate tool names. |
| Server runtime | Partial | The crate handles requests over a transport, but it does not yet host a standalone HTTP listener of its own. |

## Public Surface

- `NexusClient` (struct): Transport-parameterized MCP client with HTTP and stdio constructors.
- `NexusServer` (struct): Transport-driven MCP server runtime for tool registration and request handling.
- `NexusClientTrait` (trait): Transport-agnostic client contract for sending requests and invoking tools.
- `NexusServerTrait` (trait): Transport-agnostic server contract for registering tools and serving.
- `McpTransport` (trait): Abstract send/receive transport used by client and server runtimes.
- `StreamableHttpTransport` (struct): Reqwest-backed transport for MCP over streamable HTTP.
- `StdioTransport` (struct): Subprocess-backed stdio transport for local MCP hosts.
- `MultiMcpClient` / `MultiMcpSession` (structs): Additive multi-server connection and merged-tool session layer.
- `McpServerConfig` / `McpServerTransport` (struct + enum): Typed server configuration used by `MultiMcpClient`.
- `known_servers` (module): Curated typed presets for common MCP server setups.
- `JsonRpcOrchestrator` (struct): Small wrapper around JSON-RPC encoding and decoding helpers.
- `McpTool / McpTask / ServerCard` (structs): Primary MCP domain entities exposed by the crate.
- `McpError` (enum): Error type for protocol, transport, auth, task, and tool-execution failures.

## Known Gaps & Limitations

- The server runtime is transport-driven and does not yet expose its own standalone HTTP listener.
- OAuth token issuance or full auth flows are not implemented inside this crate.
- `MultiMcpClient` intentionally returns a merged MCP session instead of a `ForgeRegistry` so `or-mcp` stays independent from `or-forge`.
