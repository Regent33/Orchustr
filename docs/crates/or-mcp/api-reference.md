# or-mcp API Reference

This page documents the main public surface re-exported by `or-mcp/src/lib.rs` and the key entry points behind those re-exports. 
### `NexusClient`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-mcp/src/application/client.rs |
| **Status** | 🟡 |

**Description**: Transport-parameterized MCP client with HTTP and stdio constructors.

**Signature**
```rust
pub struct NexusClient<T: McpTransport> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `NexusServer`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-mcp/src/application/server.rs |
| **Status** | 🟡 |

**Description**: Transport-driven MCP server runtime for tool registration and request handling.

**Signature**
```rust
pub struct NexusServer<T: McpTransport> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `NexusClientTrait`
| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | crates/or-mcp/src/domain/contracts.rs |
| **Status** | 🟡 |

**Description**: Transport-agnostic client contract for sending requests and invoking tools.

**Signature**
```rust
pub trait NexusClientTrait: Send + Sync + 'static
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `NexusServerTrait`
| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | crates/or-mcp/src/domain/contracts.rs |
| **Status** | 🟡 |

**Description**: Transport-agnostic server contract for registering tools and serving.

**Signature**
```rust
pub trait NexusServerTrait: Send + Sync + 'static
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `McpTransport`
| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | crates/or-mcp/src/domain/contracts.rs |
| **Status** | 🟡 |

**Description**: Abstract send/receive transport used by client and server runtimes.

**Signature**
```rust
pub trait McpTransport: Send + Sync + 'static
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `StreamableHttpTransport`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-mcp/src/infra/http_transport.rs |
| **Status** | 🟡 |

**Description**: Reqwest-backed transport for MCP over streamable HTTP.

**Signature**
```rust
pub struct StreamableHttpTransport { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `StdioTransport`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-mcp/src/infra/stdio_transport.rs |
| **Status** | 🟡 |

**Description**: Subprocess-backed stdio transport for local MCP hosts.

**Signature**
```rust
pub struct StdioTransport { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `JsonRpcOrchestrator`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-mcp/src/application/orchestrators.rs |
| **Status** | 🟡 |

**Description**: Small wrapper around JSON-RPC encoding and decoding helpers.

**Signature**
```rust
pub struct JsonRpcOrchestrator;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `McpTool / McpTask / ServerCard`
| Property | Value |
|---|---|
| **Kind** | structs |
| **Visibility** | pub |
| **File** | crates/or-mcp/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Primary MCP domain entities exposed by the crate.

**Signature**
```rust
pub struct McpTool { ... }; pub struct McpTask { ... }; pub struct ServerCard { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `McpError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-mcp/src/domain/errors.rs |
| **Status** | 🟡 |

**Description**: Error type for protocol, transport, auth, task, and tool-execution failures.

**Signature**
```rust
pub enum McpError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- The server runtime is transport-driven and does not yet expose its own standalone HTTP listener.
- OAuth token issuance or full auth flows are not implemented inside this crate.
