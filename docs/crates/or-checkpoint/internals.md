# or-checkpoint Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-checkpoint/Cargo.toml
TYPE     : toml
LOC      : 14
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-checkpoint or by its direct callers.
```

```text
PATH     : crates/or-checkpoint/src/application/mod.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-checkpoint/src/application/orchestrators.rs
TYPE     : rs
LOC      : 40
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub struct CheckpointOrchestrator;; pub async fn pause<T: OrchState, B: PersistenceBackend>(; pub async fn resume<T: OrchState, B: PersistenceBackend>(
IMPORTS  : crate::domain::entities::CheckpointRecord;; crate::domain::errors::CheckpointError;; crate::infra::implementations::CheckpointGate;; or_core::{OrchState, PersistenceBackend};
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-checkpoint/src/domain/contracts.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-checkpoint/src/domain/entities.rs
TYPE     : rs
LOC      : 7
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub struct CheckpointRecord<T> {
IMPORTS  : serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-checkpoint/src/domain/errors.rs
TYPE     : rs
LOC      : 11
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum CheckpointError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-checkpoint/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-checkpoint/src/infra/adapters.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Contains helper adapters and translation utilities.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-checkpoint/src/infra/implementations.rs
TYPE     : rs
LOC      : 56
STATUS   : 🟢 Complete
PURPOSE  : Contains the main concrete implementations.
EXPORTS  : pub struct CheckpointGate<B> {; pub fn new(graph_id: impl Into<String>, backend: B) -> Self {; pub async fn pause<T: OrchState>(; pub async fn resume<T: OrchState>(
IMPORTS  : crate::domain::entities::CheckpointRecord;; crate::domain::errors::CheckpointError;; or_core::{OrchState, PersistenceBackend};
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-checkpoint/src/infra/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-checkpoint/src/lib.rs
TYPE     : rs
LOC      : 7
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::orchestrators::CheckpointOrchestrator;; pub use domain::entities::CheckpointRecord;; pub use domain::errors::CheckpointError;; pub use infra::implementations::CheckpointGate;
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-checkpoint/tests/unit/mod.rs
TYPE     : rs
LOC      : 38
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_checkpoint::{CheckpointError, CheckpointGate, CheckpointOrchestrator};; or_core::{InMemoryPersistenceBackend, OrchState};; serde::{Deserialize, Serialize};
USED BY  : Executed by cargo test for or-checkpoint.
```

```text
PATH     : crates/or-checkpoint/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-checkpoint.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
