# or-relay Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-relay/Cargo.toml
TYPE     : toml
LOC      : 14
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-relay or by its direct callers.
```

```text
PATH     : crates/or-relay/src/application/mod.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-relay/src/application/orchestrators.rs
TYPE     : rs
LOC      : 23
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub struct RelayOrchestrator;; pub async fn execute_parallel<T: OrchState>(
IMPORTS  : crate::domain::errors::RelayError;; crate::infra::implementations::{RelayExecutor, RelayPlan};; or_core::OrchState;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-relay/src/domain/contracts.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-relay/src/domain/entities.rs
TYPE     : rs
LOC      : 5
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub struct RelayBranchMetadata {
IMPORTS  : serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-relay/src/domain/errors.rs
TYPE     : rs
LOC      : 13
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum RelayError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-relay/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-relay/src/infra/adapters.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Contains helper adapters and translation utilities.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-relay/src/infra/implementations.rs
TYPE     : rs
LOC      : 98
STATUS   : 🟢 Complete
PURPOSE  : Contains the main concrete implementations.
EXPORTS  : pub struct RelayBuilder<T: OrchState> {; pub fn new() -> Self {; pub fn add_branch<F, Fut>(mut self, name: &str, handler: F) -> Self; pub fn build(self) -> Result<RelayPlan<T>, RelayError> {; pub struct RelayPlan<T: OrchState> {; pub struct RelayExecutor;; pub async fn execute<T: OrchState>(
IMPORTS  : crate::domain::entities::RelayBranchMetadata;; crate::domain::errors::RelayError;; futures::stream::{FuturesUnordered, StreamExt};; or_core::OrchState;; std::future::Future;; std::pin::Pin;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-relay/src/infra/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-relay/src/lib.rs
TYPE     : rs
LOC      : 6
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::orchestrators::RelayOrchestrator;; pub use domain::errors::RelayError;; pub use infra::implementations::{RelayBuilder, RelayExecutor, RelayPlan};
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-relay/tests/unit/mod.rs
TYPE     : rs
LOC      : 67
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_core::OrchState;; or_relay::{RelayBuilder, RelayError, RelayExecutor, RelayOrchestrator};; serde::{Deserialize, Serialize};
USED BY  : Executed by cargo test for or-relay.
```

```text
PATH     : crates/or-relay/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-relay.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
