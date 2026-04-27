# Error Codes

This page assigns stable documentation codes to the public error variants used across the Orchustr workspace. The codes are additive documentation identifiers; they do not replace the existing Rust enums or error messages.

## Core Crates

| Code | Variant |
|---|---|
| `ORC-ANCHOR-001` | `AnchorError::VectorStore` |
| `ORC-BEACON-001` | `BeaconError::MissingTemplate` |
| `ORC-BEACON-002` | `BeaconError::InvalidTemplate` |
| `ORC-BEACON-003` | `BeaconError::MissingVariable` |
| `ORC-BEACON-004` | `BeaconError::InvalidContext` |
| `ORC-BRIDGE-001` | `BridgeError::InvalidInput` |
| `ORC-BRIDGE-002` | `BridgeError::InvalidJson` |
| `ORC-BRIDGE-003` | `BridgeError::UnsupportedCrate` |
| `ORC-BRIDGE-004` | `BridgeError::UnsupportedOperation` |
| `ORC-BRIDGE-005` | `BridgeError::Invocation` |
| `ORC-BRIDGE-006` | `BridgeError::InvalidState` |
| `ORC-BRIDGE-007` | `BridgeError::Prompt` |
| `ORC-CHECKPOINT-001` | `CheckpointError::Storage` |
| `ORC-CHECKPOINT-002` | `CheckpointError::Serialization` |
| `ORC-CHECKPOINT-003` | `CheckpointError::MissingCheckpoint` |
| `ORC-CLI-001` | `CliError::Io` |
| `ORC-CLI-002` | `CliError::Schema` |
| `ORC-CLI-003` | `CliError::Config` |
| `ORC-CLI-004` | `CliError::Validation` |
| `ORC-CLI-005` | `CliError::InvalidProject` |
| `ORC-CLI-006` | `CliError::Lens` |
| `ORC-COLONY-001` | `ColonyError::EmptyColony` |
| `ORC-COLONY-002` | `ColonyError::DuplicateMember` |
| `ORC-COLONY-003` | `ColonyError::MissingTask` |
| `ORC-COLONY-004` | `ColonyError::InvalidState` |
| `ORC-COLONY-005` | `ColonyError::Serialization` |
| `ORC-COMPASS-001` | `CompassError::EmptyRouter` |
| `ORC-COMPASS-002` | `CompassError::BlankRouteName` |
| `ORC-COMPASS-003` | `CompassError::DuplicateRoute` |
| `ORC-COMPASS-004` | `CompassError::MissingDefaultRoute` |
| `ORC-COMPASS-005` | `CompassError::NoMatchingRoute` |
| `ORC-CONDUIT-001` | `ConduitError::MissingEnvironmentVariable` |
| `ORC-CONDUIT-002` | `ConduitError::InvalidRequest` |
| `ORC-CONDUIT-003` | `ConduitError::Http` |
| `ORC-CONDUIT-004` | `ConduitError::Api` |
| `ORC-CONDUIT-005` | `ConduitError::BudgetExceeded` |
| `ORC-CONDUIT-006` | `ConduitError::RateLimited` |
| `ORC-CONDUIT-007` | `ConduitError::Serialization` |
| `ORC-CONDUIT-008` | `ConduitError::NotImplemented` |
| `ORC-CONDUIT-009` | `ConduitError::Timeout` |
| `ORC-CONDUIT-010` | `ConduitError::AuthenticationFailed` |
| `ORC-CORE-001` | `CoreError::InvalidRetryAttempt` |
| `ORC-CORE-002` | `CoreError::BudgetExceeded` |
| `ORC-CORE-003` | `CoreError::InvalidState` |
| `ORC-FORGE-001` | `ForgeError::DuplicateTool` |
| `ORC-FORGE-002` | `ForgeError::UnknownTool` |
| `ORC-FORGE-003` | `ForgeError::InvalidArguments` |
| `ORC-FORGE-004` | `ForgeError::Invocation` |
| `ORC-LENS-001` | `LensError::Bind` |
| `ORC-LENS-002` | `LensError::Serve` |
| `ORC-LOOM-001` | `LoomError::EmptyGraph` |
| `ORC-LOOM-002` | `LoomError::BlankNodeName` |
| `ORC-LOOM-003` | `LoomError::DuplicateNode` |
| `ORC-LOOM-004` | `LoomError::UnboundNode` |
| `ORC-LOOM-005` | `LoomError::MissingEntry` |
| `ORC-LOOM-006` | `LoomError::MissingExit` |
| `ORC-LOOM-007` | `LoomError::UnknownNode` |
| `ORC-LOOM-008` | `LoomError::EdgeReferencesUnknownNode` |
| `ORC-LOOM-009` | `LoomError::NoEdgeFromNode` |
| `ORC-LOOM-010` | `LoomError::AmbiguousNextNode` |
| `ORC-LOOM-011` | `LoomError::InvalidBranchTarget` |
| `ORC-LOOM-012` | `LoomError::UnknownHandler` |
| `ORC-LOOM-013` | `LoomError::UnknownCondition` |
| `ORC-LOOM-014` | `LoomError::NoConditionalMatch` |
| `ORC-LOOM-015` | `LoomError::Paused` |
| `ORC-LOOM-016` | `LoomError::StepLimitExceeded` |
| `ORC-LOOM-017` | `LoomError::NodeExecution` |
| `ORC-MCP-001` | `McpError::Protocol` |
| `ORC-MCP-002` | `McpError::Transport` |
| `ORC-MCP-003` | `McpError::Auth` |
| `ORC-MCP-004` | `McpError::ToolExecution` |
| `ORC-MCP-005` | `McpError::TaskExpired` |
| `ORC-MCP-006` | `McpError::Serialization` |
| `ORC-PIPELINE-001` | `PipelineError::EmptyPipeline` |
| `ORC-PIPELINE-002` | `PipelineError::BlankNodeName` |
| `ORC-PIPELINE-003` | `PipelineError::DuplicateNode` |
| `ORC-PIPELINE-004` | `PipelineError::NodeExecution` |
| `ORC-PRISM-001` | `PrismError::InvalidEndpoint` |
| `ORC-PRISM-002` | `PrismError::Exporter` |
| `ORC-PRISM-003` | `PrismError::Lens` |
| `ORC-PRISM-004` | `PrismError::Subscriber` |
| `ORC-RECALL-001` | `RecallError::Storage` |
| `ORC-RECALL-002` | `RecallError::Serialization` |
| `ORC-RELAY-001` | `RelayError::EmptyPlan` |
| `ORC-RELAY-002` | `RelayError::BlankBranchName` |
| `ORC-RELAY-003` | `RelayError::DuplicateBranch` |
| `ORC-RELAY-004` | `RelayError::BranchExecution` |
| `ORC-SCHEMA-001` | `SchemaError::Json` |
| `ORC-SCHEMA-002` | `SchemaError::YamlFeatureDisabled` |
| `ORC-SCHEMA-003` | `SchemaError::Yaml` |
| `ORC-SENTINEL-001` | `SentinelError::MissingMessages` |
| `ORC-SENTINEL-002` | `SentinelError::InvalidState` |
| `ORC-SENTINEL-003` | `SentinelError::InvalidResponse` |
| `ORC-SENTINEL-004` | `SentinelError::Serialization` |
| `ORC-SENTINEL-005` | `SentinelError::Conduit` |
| `ORC-SENTINEL-006` | `SentinelError::Forge` |
| `ORC-SENTINEL-007` | `SentinelError::Loom` |
| `ORC-SENTINEL-008` | `SentinelError::Core` |
| `ORC-SIEVE-001` | `SieveError::InvalidJson` |
| `ORC-SIEVE-002` | `SieveError::SchemaViolation` |
| `ORC-SIEVE-003` | `SieveError::Deserialization` |
| `ORC-SIEVE-004` | `SieveError::EmptyText` |

## Tool Crates

| Code | Variant |
|---|---|
| `ORC-TOOLS-COMMS-001` | `CommsError::MissingCredential` |
| `ORC-TOOLS-COMMS-002` | `CommsError::Transport` |
| `ORC-TOOLS-COMMS-003` | `CommsError::Upstream` |
| `ORC-TOOLS-COMMS-004` | `CommsError::InvalidInput` |
| `ORC-TOOLS-COMMS-005` | `CommsError::UnsupportedChannel` |
| `ORC-TOOLS-CORE-001` | `ToolError::NotFound` |
| `ORC-TOOLS-CORE-002` | `ToolError::AlreadyRegistered` |
| `ORC-TOOLS-CORE-003` | `ToolError::InvalidInput` |
| `ORC-TOOLS-CORE-004` | `ToolError::Transport` |
| `ORC-TOOLS-CORE-005` | `ToolError::Upstream` |
| `ORC-TOOLS-CORE-006` | `ToolError::MissingCredential` |
| `ORC-TOOLS-CORE-007` | `ToolError::Timeout` |
| `ORC-TOOLS-CORE-008` | `ToolError::Unavailable` |
| `ORC-TOOLS-CORE-009` | `ToolError::Serialization` |
| `ORC-TOOLS-EXEC-001` | `ExecError::UnsupportedLanguage` |
| `ORC-TOOLS-EXEC-002` | `ExecError::ExecutorNotFound` |
| `ORC-TOOLS-EXEC-003` | `ExecError::MissingCredential` |
| `ORC-TOOLS-EXEC-004` | `ExecError::Timeout` |
| `ORC-TOOLS-EXEC-005` | `ExecError::Spawn` |
| `ORC-TOOLS-EXEC-006` | `ExecError::Upstream` |
| `ORC-TOOLS-EXEC-007` | `ExecError::Transport` |
| `ORC-TOOLS-EXEC-008` | `ExecError::Io` |
| `ORC-TOOLS-FILE-001` | `FileError::NotFound` |
| `ORC-TOOLS-FILE-002` | `FileError::PermissionDenied` |
| `ORC-TOOLS-FILE-003` | `FileError::Io` |
| `ORC-TOOLS-FILE-004` | `FileError::Json` |
| `ORC-TOOLS-FILE-005` | `FileError::MissingCredential` |
| `ORC-TOOLS-FILE-006` | `FileError::Upstream` |
| `ORC-TOOLS-FILE-007` | `FileError::Transport` |
| `ORC-TOOLS-LOADERS-001` | `LoaderError::UnsupportedFormat` |
| `ORC-TOOLS-LOADERS-002` | `LoaderError::Io` |
| `ORC-TOOLS-LOADERS-003` | `LoaderError::Parse` |
| `ORC-TOOLS-LOADERS-004` | `LoaderError::InvalidSource` |
| `ORC-TOOLS-PRODUCTIVITY-001` | `ProductivityError::MissingCredential` |
| `ORC-TOOLS-PRODUCTIVITY-002` | `ProductivityError::Transport` |
| `ORC-TOOLS-PRODUCTIVITY-003` | `ProductivityError::Upstream` |
| `ORC-TOOLS-PRODUCTIVITY-004` | `ProductivityError::InvalidInput` |
| `ORC-TOOLS-PRODUCTIVITY-005` | `ProductivityError::NotFound` |
| `ORC-TOOLS-SEARCH-001` | `SearchError::MissingApiKey` |
| `ORC-TOOLS-SEARCH-002` | `SearchError::EmptyQuery` |
| `ORC-TOOLS-SEARCH-003` | `SearchError::Upstream` |
| `ORC-TOOLS-SEARCH-004` | `SearchError::Transport` |
| `ORC-TOOLS-SEARCH-005` | `SearchError::Serialization` |
| `ORC-TOOLS-SEARCH-006` | `SearchError::NoProviders` |
| `ORC-TOOLS-VECTOR-001` | `VectorError::MissingCredential` |
| `ORC-TOOLS-VECTOR-002` | `VectorError::InvalidInput` |
| `ORC-TOOLS-VECTOR-003` | `VectorError::DimensionMismatch` |
| `ORC-TOOLS-VECTOR-004` | `VectorError::CollectionNotFound` |
| `ORC-TOOLS-VECTOR-005` | `VectorError::Upstream` |
| `ORC-TOOLS-VECTOR-006` | `VectorError::Transport` |
| `ORC-TOOLS-VECTOR-007` | `VectorError::Serialization` |
| `ORC-TOOLS-WEB-001` | `WebError::MissingCredential` |
| `ORC-TOOLS-WEB-002` | `WebError::InvalidUrl` |
| `ORC-TOOLS-WEB-003` | `WebError::UnsafeScheme` |
| `ORC-TOOLS-WEB-004` | `WebError::Upstream` |
| `ORC-TOOLS-WEB-005` | `WebError::Transport` |
| `ORC-TOOLS-WEB-006` | `WebError::HtmlParse` |
| `ORC-TOOLS-WEB-007` | `WebError::Timeout` |
| `ORC-TOOLS-WEB-008` | `WebError::MethodUnsupported` |

## Notes

- These codes are stable documentation identifiers intended for runbooks, support notes, and changelog references.
- The runtime still returns the existing Rust enum variants and human-readable messages; no API surface was renamed to introduce these codes.

## Schema Notes

### `LoomError::Paused`

Carries the merged graph state at the point of pause as a
`serde_json::Value` field named `state` alongside the original
`checkpoint_id`. Callers can resume directly from this field without
round-tripping through a `PersistenceBackend`. (Earlier versions of
this variant only carried `checkpoint_id`, dropping any state changes
written by the paused node.)

### `SentinelError::Loom` / `SentinelError::Core`

These variants now wrap the underlying typed error (`or_loom::LoomError`
and `or_core::CoreError` respectively) instead of stringifying it. Use
`#[from]` propagation or pattern-match the inner error to recover full
context; `to_string()` still produces the same human-readable message
as before.

### `CliError::Lens`

Wraps `or_lens::LensError` directly via `#[from]`, so callers can
distinguish bind failures from serve failures by matching the inner
variant.

### `ExecError::ExecutorNotFound { executor: "shell" }`

Returned by `or-tools-exec::ShellExecutor::execute` when
`ORCHUSTR_ALLOW_UNSANDBOXED_SHELL` is not set. The `reason` field
includes guidance toward sandboxed alternatives.
