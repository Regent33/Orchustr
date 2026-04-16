# or-mcp Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-mcp/Cargo.toml
TYPE     : toml
LOC      : 17
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-mcp or by its direct callers.
```

```text
PATH     : crates/or-mcp/src/application/client.rs
TYPE     : rs
LOC      : 100
STATUS   : 🟢 Complete
PURPOSE  : Implements a focused part of the crate.
EXPORTS  : pub struct NexusClient<T: McpTransport> {; pub fn new(transport: T) -> Self {; pub async fn initialize(&self) -> Result<serde_json::Value, McpError> {; pub async fn ping(&self) -> Result<serde_json::Value, McpError> {; pub async fn get_task(&self, id: &str) -> Result<McpTask, McpError> {; pub fn connect_http(endpoint: impl Into<String>) -> Self {; pub fn connect_stdio(command: &str, args: &[&str]) -> Result<Self, McpError> {
IMPORTS  : crate::domain::contracts::{McpTransport, NexusClientTrait};; crate::domain::entities::{; crate::domain::errors::McpError;; crate::infra::http_transport::StreamableHttpTransport;; crate::infra::stdio_transport::StdioTransport;; std::sync::Arc;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-mcp/src/application/mod.rs
TYPE     : rs
LOC      : 5
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-mcp/src/application/orchestrators.rs
TYPE     : rs
LOC      : 29
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub struct JsonRpcOrchestrator;; pub fn encode(&self, message: JsonRpcMessage) -> Result<String, McpError> {; pub fn decode(&self, raw: &str) -> Result<JsonRpcPacket, McpError> {
IMPORTS  : crate::domain::entities::{JsonRpcMessage, JsonRpcPacket};; crate::domain::errors::McpError;; crate::infra::jsonrpc::{decode_packet, encode_packet};
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-mcp/src/application/server.rs
TYPE     : rs
LOC      : 166
STATUS   : 🟢 Complete
PURPOSE  : Implements a focused part of the crate.
EXPORTS  : pub struct NexusServer<T: McpTransport> {; pub fn new(transport: T, server_card: ServerCard) -> Self {; pub async fn register_tool_handler<F, Fut>(; pub async fn register_task(&self, task: McpTask) {; pub fn server_card(&self) -> &ServerCard {; pub fn server_card_path(&self) -> &'static str {; pub fn server_card_json(&self) -> Result<String, McpError> {; pub async fn handle_message(
IMPORTS  : crate::application::server_handlers::{call_tool, get_task};; crate::domain::contracts::{McpTransport, NexusServerTrait};; crate::domain::entities::{; crate::domain::errors::McpError;; std::collections::HashMap;; std::future::Future;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-mcp/src/application/server_handlers.rs
TYPE     : rs
LOC      : 46
STATUS   : 🟢 Complete
PURPOSE  : Implements a focused part of the crate.
EXPORTS  : `(none)`
IMPORTS  : crate::application::server::RegisteredTool;; crate::application::server_validation::validate_input;; crate::domain::entities::McpTask;; crate::domain::errors::McpError;; std::collections::HashMap;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-mcp/src/application/server_validation.rs
TYPE     : rs
LOC      : 61
STATUS   : 🟢 Complete
PURPOSE  : Implements a focused part of the crate.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::errors::McpError;; schemars::schema::{InstanceType, RootSchema, Schema, SingleOrVec};
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-mcp/src/domain/contracts.rs
TYPE     : rs
LOC      : 27
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : pub trait NexusClientTrait: Send + Sync + 'static {; pub trait NexusServerTrait: Send + Sync + 'static {; pub trait McpTransport: Send + Sync + 'static {
IMPORTS  : crate::domain::entities::{JsonRpcMessage, McpTool};; crate::domain::errors::McpError;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-mcp/src/domain/entities.rs
TYPE     : rs
LOC      : 84
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub enum JsonRpcId {; pub struct JsonRpcRequest {; pub struct JsonRpcNotification {; pub struct JsonRpcSuccessResponse {; pub struct JsonRpcErrorDetail {; pub struct JsonRpcErrorResponse {; pub enum JsonRpcMessage {; pub enum JsonRpcPacket {
IMPORTS  : schemars::schema::RootSchema;; serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-mcp/src/domain/errors.rs
TYPE     : rs
LOC      : 22
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum McpError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-mcp/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-mcp/src/infra/http_transport.rs
TYPE     : rs
LOC      : 78
STATUS   : 🟢 Complete
PURPOSE  : Contains the streamable HTTP transport implementation.
EXPORTS  : pub struct StreamableHttpTransport {; pub fn new(endpoint: impl Into<String>) -> Self {
IMPORTS  : crate::domain::contracts::McpTransport;; crate::domain::entities::JsonRpcMessage;; crate::domain::errors::McpError;; crate::infra::jsonrpc::{decode_streamable_body, encode_message};; reqwest::Client;; reqwest::header::{ACCEPT, HeaderMap, HeaderValue};
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-mcp/src/infra/jsonrpc.rs
TYPE     : rs
LOC      : 41
STATUS   : 🟢 Complete
PURPOSE  : Contains JSON-RPC encoding and decoding helpers.
EXPORTS  : pub fn encode_message(message: &JsonRpcMessage) -> Result<String, McpError> {; pub fn encode_packet(packet: &JsonRpcPacket) -> Result<String, McpError> {; pub fn decode_message(raw: &str) -> Result<JsonRpcMessage, McpError> {; pub fn decode_packet(raw: &str) -> Result<JsonRpcPacket, McpError> {; pub fn decode_streamable_body(raw: &str) -> Result<Option<JsonRpcMessage>, McpError> {
IMPORTS  : crate::domain::entities::{JsonRpcMessage, JsonRpcPacket};; crate::domain::errors::McpError;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-mcp/src/infra/mod.rs
TYPE     : rs
LOC      : 5
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-mcp/src/infra/stdio_transport.rs
TYPE     : rs
LOC      : 66
STATUS   : 🟢 Complete
PURPOSE  : Contains the stdio transport implementation.
EXPORTS  : pub struct StdioTransport {; pub fn spawn(command: &str, args: &[&str]) -> Result<Self, McpError> {
IMPORTS  : crate::domain::contracts::McpTransport;; crate::domain::entities::JsonRpcMessage;; crate::domain::errors::McpError;; crate::infra::jsonrpc::{decode_message, encode_message};; std::process::Stdio;; std::sync::Arc;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-mcp/src/lib.rs
TYPE     : rs
LOC      : 15
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::client::NexusClient;; pub use application::orchestrators::JsonRpcOrchestrator;; pub use application::server::NexusServer;; pub use domain::contracts::{McpTransport, NexusClientTrait, NexusServerTrait};; pub use domain::entities::{; pub use domain::errors::McpError;; pub use infra::http_transport::StreamableHttpTransport;; pub use infra::stdio_transport::StdioTransport;
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-mcp/tests/unit/client_server.rs
TYPE     : rs
LOC      : 136
STATUS   : 🟢 Complete
PURPOSE  : Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_mcp::{; schemars::schema::{InstanceType, RootSchema, SchemaObject, SingleOrVec};; std::collections::VecDeque;; std::sync::Arc;; tokio::sync::Mutex;
USED BY  : Executed by cargo test for or-mcp.
```

```text
PATH     : crates/or-mcp/tests/unit/mod.rs
TYPE     : rs
LOC      : 44
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_mcp::{
USED BY  : Executed by cargo test for or-mcp.
```

```text
PATH     : crates/or-mcp/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-mcp.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
