# or-compass Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-compass/Cargo.toml
TYPE     : toml
LOC      : 13
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-compass or by its direct callers.
```

```text
PATH     : crates/or-compass/src/application/mod.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-compass/src/application/orchestrators.rs
TYPE     : rs
LOC      : 23
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub struct CompassOrchestrator;; pub fn select_route<T: OrchState>(
IMPORTS  : crate::domain::entities::RouteSelection;; crate::domain::errors::CompassError;; crate::infra::implementations::CompassRouter;; or_core::OrchState;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-compass/src/domain/contracts.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-compass/src/domain/entities.rs
TYPE     : rs
LOC      : 5
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub struct RouteSelection {
IMPORTS  : serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-compass/src/domain/errors.rs
TYPE     : rs
LOC      : 15
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum CompassError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-compass/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-compass/src/infra/adapters.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Contains helper adapters and translation utilities.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-compass/src/infra/implementations.rs
TYPE     : rs
LOC      : 98
STATUS   : 🟢 Complete
PURPOSE  : Contains the main concrete implementations.
EXPORTS  : pub struct CompassRouterBuilder<T: OrchState> {; pub fn new() -> Self {; pub fn add_route<F>(mut self, name: &str, predicate: F) -> Self; pub fn set_default(mut self, route: &str) -> Self {; pub fn build(self) -> Result<CompassRouter<T>, CompassError> {; pub struct CompassRouter<T: OrchState> {; pub fn select(&self, state: &T) -> Result<RouteSelection, CompassError> {
IMPORTS  : crate::domain::entities::RouteSelection;; crate::domain::errors::CompassError;; or_core::OrchState;; std::sync::Arc;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-compass/src/infra/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-compass/src/lib.rs
TYPE     : rs
LOC      : 7
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::orchestrators::CompassOrchestrator;; pub use domain::entities::RouteSelection;; pub use domain::errors::CompassError;; pub use infra::implementations::{CompassRouter, CompassRouterBuilder};
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-compass/tests/unit/mod.rs
TYPE     : rs
LOC      : 53
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_compass::{CompassError, CompassOrchestrator, CompassRouterBuilder};; or_core::OrchState;; serde::{Deserialize, Serialize};
USED BY  : Executed by cargo test for or-compass.
```

```text
PATH     : crates/or-compass/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-compass.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
