# or-lens API Reference

This page documents the main public surface re-exported by `or-lens/src/lib.rs`.

### `LensHandle`
- **Kind**: struct
- **File**: `crates/or-lens/src/application/orchestrators.rs`
- **Description**: Handle returned by the dashboard startup path. Exposes the bound port, collector access, and shutdown signaling.

### `start_dashboard_server`
- **Kind**: async fn
- **File**: `crates/or-lens/src/application/orchestrators.rs`
- **Description**: Starts the local dashboard server with a fresh in-memory collector.

### `LensLayer`
- **Kind**: struct
- **File**: `crates/or-lens/src/infra/tracing.rs`
- **Description**: Tracing layer that mirrors completed spans into a trace repository.

### `SpanCollector`
- **Kind**: struct
- **File**: `crates/or-lens/src/infra/repositories.rs`
- **Description**: In-memory trace repository used by the dashboard server and tests.

### `LensSpan`
- **Kind**: struct
- **File**: `crates/or-lens/src/domain/entities.rs`
- **Description**: Serializable span record collected by the dashboard.

### `LensSpanStatus`
- **Kind**: enum
- **File**: `crates/or-lens/src/domain/entities.rs`
- **Description**: Execution status for a collected span.

### `TraceSummary`
- **Kind**: struct
- **File**: `crates/or-lens/src/domain/entities.rs`
- **Description**: Summary metadata returned by the trace listing endpoint.

### `ExecutionSnapshot`
- **Kind**: struct
- **File**: `crates/or-lens/src/domain/entities.rs`
- **Description**: Serializable execution timeline produced from collected spans.

### `ExecutionNodeSnapshot`
- **Kind**: struct
- **File**: `crates/or-lens/src/domain/entities.rs`
- **Description**: Node-level view of a collected span inside an execution snapshot.

### `LensError`
- **Kind**: enum
- **File**: `crates/or-lens/src/domain/errors.rs`
- **Description**: Error type for bind and serve failures.

## Known Gaps & Limitations

- The crate only exports its public surface when feature `dashboard` is enabled.
- The current implementation mirrors spans in-process instead of accepting them through a standalone receiver service.
