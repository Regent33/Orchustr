# or-loom Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-loom/Cargo.toml
TYPE     : toml
LOC      : 14
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-loom or by its direct callers.
```

```text
PATH     : crates/or-loom/src/application/mod.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-loom/src/application/orchestrators.rs
TYPE     : rs
LOC      : 38
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub struct LoomOrchestrator;; pub async fn execute_graph<T: OrchState>(; pub async fn resume_graph<T: OrchState>(
IMPORTS  : crate::domain::errors::LoomError;; crate::infra::implementations::ExecutionGraph;; or_core::OrchState;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-loom/src/domain/contracts.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-loom/src/domain/entities.rs
TYPE     : rs
LOC      : 25
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub enum NodeResult<T> {; pub fn advance(state: T) -> Result<Self, LoomError> {; pub fn branch(state: T, next: impl Into<String>) -> Result<Self, LoomError> {; pub fn pause(checkpoint_id: impl Into<String>, state: T) -> Result<Self, LoomError> {
IMPORTS  : crate::domain::errors::LoomError;; serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-loom/src/domain/errors.rs
TYPE     : rs
LOC      : 29
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum LoomError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-loom/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-loom/src/infra/adapters.rs
TYPE     : rs
LOC      : 41
STATUS   : 🟢 Complete
PURPOSE  : Contains helper adapters and translation utilities.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::errors::LoomError;; std::collections::{BTreeSet, HashMap};
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-loom/src/infra/implementations.rs
TYPE     : rs
LOC      : 153
STATUS   : 🟢 Complete
PURPOSE  : Contains the main concrete implementations.
EXPORTS  : pub struct GraphBuilder<T: OrchState> {; pub fn new() -> Self {; pub fn add_node<F, Fut>(mut self, name: &str, handler: F) -> Self; pub fn add_edge(mut self, from: &str, to: &str) -> Self {; pub fn set_entry(mut self, name: &str) -> Self {; pub fn set_exit(mut self, name: &str) -> Self {; pub fn build(self) -> Result<ExecutionGraph<T>, LoomError> {; pub struct ExecutionGraph<T: OrchState> {
IMPORTS  : crate::domain::entities::NodeResult;; crate::domain::errors::LoomError;; crate::infra::adapters::validate_graph_shape;; or_core::OrchState;; std::collections::HashMap;; std::future::Future;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-loom/src/infra/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-loom/src/lib.rs
TYPE     : rs
LOC      : 7
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::orchestrators::LoomOrchestrator;; pub use domain::entities::NodeResult;; pub use domain::errors::LoomError;; pub use infra::implementations::{ExecutionGraph, GraphBuilder};
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-loom/tests/unit/mod.rs
TYPE     : rs
LOC      : 106
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_checkpoint::{CheckpointGate, CheckpointOrchestrator};; or_core::{InMemoryPersistenceBackend, OrchState};; or_loom::{GraphBuilder, LoomError, LoomOrchestrator, NodeResult};; serde::{Deserialize, Serialize};
USED BY  : Executed by cargo test for or-loom.
```

```text
PATH     : crates/or-loom/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-loom.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
