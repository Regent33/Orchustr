# Error Codes

This page indexes every crate-local error enum found in `src/domain/errors.rs`.

## Error Types

| Error type | Defined in | Variants | Retryable guidance |
|---|---|---|---|
| `AnchorError` | `crates/or-anchor/src/domain/errors.rs` | `VectorStore` | Depends on the backing store error. |
| `BeaconError` | `crates/or-beacon/src/domain/errors.rs` | `MissingTemplate`, `InvalidTemplate`, `MissingVariable`, `InvalidContext` | Usually no; fix input/template first. |
| `BridgeError` | `crates/or-bridge/src/domain/errors.rs` | `InvalidJson`, `InvalidState`, `Prompt` | Usually no; correct the bridge payload. |
| `CheckpointError` | `crates/or-checkpoint/src/domain/errors.rs` | `Storage`, `Serialization`, `MissingCheckpoint` | `Storage` may be retryable depending on backend; `MissingCheckpoint` is not. |
| `ColonyError` | `crates/or-colony/src/domain/errors.rs` | `EmptyColony`, `DuplicateMember`, `MissingTask`, `InvalidState`, `Serialization` | Mostly no; fix orchestration setup or state. |
| `CompassError` | `crates/or-compass/src/domain/errors.rs` | `EmptyRouter`, `BlankRouteName`, `DuplicateRoute`, `MissingDefaultRoute`, `NoMatchingRoute` | No; fix router construction or routing coverage. |
| `ConduitError` | `crates/or-conduit/src/domain/errors.rs` | `MissingEnvironmentVariable`, `InvalidRequest`, `Http`, `Api`, `BudgetExceeded`, `RateLimited`, `Serialization`, `NotImplemented` | `Http` and `RateLimited` are the main retry candidates. |
| `CoreError` | `crates/or-core/src/domain/errors.rs` | `InvalidRetryAttempt`, `BudgetExceeded`, `InvalidState` | Usually no; fix caller configuration or state. |
| `ForgeError` | `crates/or-forge/src/domain/errors.rs` | `DuplicateTool`, `UnknownTool`, `InvalidArguments`, `Invocation` | `Invocation` may be retryable depending on tool behavior. |
| `LoomError` | `crates/or-loom/src/domain/errors.rs` | `EmptyGraph`, `BlankNodeName`, `DuplicateNode`, `MissingEntry`, `MissingExit`, `UnknownNode`, `EdgeReferencesUnknownNode`, `NoEdgeFromNode`, `AmbiguousNextNode`, `InvalidBranchTarget`, `Paused`, `StepLimitExceeded` | `Paused` is a control-flow signal; most others require graph fixes. |
| `McpError` | `crates/or-mcp/src/domain/errors.rs` | `Protocol`, `Transport`, `Auth`, `ToolExecution`, `TaskExpired`, `Serialization` | `Transport` may be retryable; `Auth` and `Protocol` usually are not. |
| `PipelineError` | `crates/or-pipeline/src/domain/errors.rs` | `EmptyPipeline`, `BlankNodeName`, `DuplicateNode`, `NodeExecution` | `NodeExecution` depends on the failing node. |
| `PrismError` | `crates/or-prism/src/domain/errors.rs` | `InvalidEndpoint`, `Exporter`, `Subscriber` | `Exporter` may be transient; `InvalidEndpoint` is not. |
| `RecallError` | `crates/or-recall/src/domain/errors.rs` | `Storage`, `Serialization` | `Storage` may be retryable depending on backend. |
| `RelayError` | `crates/or-relay/src/domain/errors.rs` | `EmptyPlan`, `BlankBranchName`, `DuplicateBranch`, `BranchExecution` | `BranchExecution` depends on the failing branch. |
| `SentinelError` | `crates/or-sentinel/src/domain/errors.rs` | `MissingMessages`, `InvalidState`, `InvalidResponse`, `Serialization`, `Conduit`, `Forge`, `Loom`, `Core` | Retryability depends on the wrapped subsystem string. |
| `SieveError` | `crates/or-sieve/src/domain/errors.rs` | `InvalidJson`, `SchemaViolation`, `Deserialization`, `EmptyText` | No; fix model output or parser expectations. |

## Propagation Notes

- Lower-level crate errors are often converted into strings in higher-level runtimes such as `or-sentinel`.
- Provider and transport layers are the main places where transient failures can realistically benefit from retry logic.

⚠️ Known Gaps & Limitations
- The repository does not define numeric error codes; error types are enum-based only.
- Retryability is guidance derived from the current code paths, not a hard guarantee encoded in the types.
