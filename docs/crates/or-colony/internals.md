# or-colony Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-colony/Cargo.toml
TYPE     : toml
LOC      : 15
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-colony or by its direct callers.
```

```text
PATH     : crates/or-colony/src/application/mod.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-colony/src/application/orchestrators.rs
TYPE     : rs
LOC      : 56
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub struct ColonyOrchestrator {; pub fn new() -> Self {; pub fn add_member<A>(mut self, name: &str, role: &str, agent: A) -> Result<Self, ColonyError>; pub async fn coordinate(
IMPORTS  : crate::domain::contracts::ColonyAgentTrait;; crate::domain::entities::ColonyResult;; crate::domain::errors::ColonyError;; crate::infra::adapters::{record_message, result_from_parts, seed_message};; crate::infra::implementations::ColonyRoster;; or_core::DynState;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-colony/src/domain/contracts.rs
TYPE     : rs
LOC      : 16
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : pub type ColonyFuture =; pub trait ColonyAgentTrait: Send + Sync + 'static {
IMPORTS  : crate::domain::entities::{ColonyMember, ColonyMessage};; crate::domain::errors::ColonyError;; or_core::DynState;; std::future::Future;; std::pin::Pin;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-colony/src/domain/entities.rs
TYPE     : rs
LOC      : 19
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub struct ColonyMember {; pub struct ColonyMessage {; pub struct ColonyResult {
IMPORTS  : or_core::DynState;; serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-colony/src/domain/errors.rs
TYPE     : rs
LOC      : 15
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum ColonyError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-colony/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-colony/src/infra/adapters.rs
TYPE     : rs
LOC      : 54
STATUS   : 🟢 Complete
PURPOSE  : Contains helper adapters and translation utilities.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::entities::{ColonyMessage, ColonyResult};; crate::domain::errors::ColonyError;; or_core::DynState;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-colony/src/infra/implementations.rs
TYPE     : rs
LOC      : 42
STATUS   : 🟢 Complete
PURPOSE  : Contains the main concrete implementations.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::contracts::ColonyAgentTrait;; crate::domain::entities::ColonyMember;; crate::domain::errors::ColonyError;; std::sync::Arc;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-colony/src/infra/mod.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-colony/src/lib.rs
TYPE     : rs
LOC      : 6
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::orchestrators::ColonyOrchestrator;; pub use domain::entities::{ColonyMember, ColonyMessage, ColonyResult};; pub use domain::errors::ColonyError;
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-colony/tests/unit/mod.rs
TYPE     : rs
LOC      : 70
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_colony::domain::contracts::{ColonyAgentTrait, ColonyFuture};; or_colony::{ColonyError, ColonyMember, ColonyMessage, ColonyOrchestrator};; or_core::DynState;
USED BY  : Executed by cargo test for or-colony.
```

```text
PATH     : crates/or-colony/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-colony.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
