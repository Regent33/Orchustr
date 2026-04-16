# or-recall Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-recall/Cargo.toml
TYPE     : toml
LOC      : 19
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-recall or by its direct callers.
```

```text
PATH     : crates/or-recall/src/application/mod.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-recall/src/application/orchestrators.rs
TYPE     : rs
LOC      : 37
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub struct RecallOrchestrator;; pub async fn remember<S: RecallStore>(; pub async fn recall<S: RecallStore>(
IMPORTS  : crate::domain::contracts::RecallStore;; crate::domain::entities::{MemoryKind, RecallEntry};; crate::domain::errors::RecallError;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-recall/src/domain/contracts.rs
TYPE     : rs
LOC      : 8
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : pub trait RecallStore: Send + Sync + 'static {
IMPORTS  : crate::domain::entities::{MemoryKind, RecallEntry};; crate::domain::errors::RecallError;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-recall/src/domain/entities.rs
TYPE     : rs
LOC      : 15
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub enum MemoryKind {; pub struct RecallEntry {
IMPORTS  : serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-recall/src/domain/errors.rs
TYPE     : rs
LOC      : 9
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum RecallError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-recall/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-recall/src/infra/adapters.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Contains helper adapters and translation utilities.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-recall/src/infra/implementations.rs
TYPE     : rs
LOC      : 36
STATUS   : 🟢 Complete
PURPOSE  : Contains the main concrete implementations.
EXPORTS  : pub struct InMemoryRecallStore {; pub fn new() -> Self {
IMPORTS  : crate::domain::contracts::RecallStore;; crate::domain::entities::{MemoryKind, RecallEntry};; crate::domain::errors::RecallError;; std::collections::HashMap;; std::sync::Arc;; tokio::sync::RwLock;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-recall/src/infra/mod.rs
TYPE     : rs
LOC      : 6
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-recall/src/infra/sqlite.rs
TYPE     : rs
LOC      : 95
STATUS   : 🟢 Complete
PURPOSE  : Contains the optional SQLite-backed implementation.
EXPORTS  : pub struct SqliteRecallStore {; pub async fn connect(database_url: &str) -> Result<Self, RecallError> {
IMPORTS  : crate::domain::contracts::RecallStore;; crate::domain::entities::{MemoryKind, RecallEntry};; crate::domain::errors::RecallError;; sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};; sqlx::{Pool, Row, Sqlite};; std::str::FromStr;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-recall/src/lib.rs
TYPE     : rs
LOC      : 9
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::orchestrators::RecallOrchestrator;; pub use domain::entities::{MemoryKind, RecallEntry};; pub use domain::errors::RecallError;; pub use infra::implementations::InMemoryRecallStore;; pub use infra::sqlite::SqliteRecallStore;
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-recall/tests/unit/mod.rs
TYPE     : rs
LOC      : 32
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_recall::{InMemoryRecallStore, MemoryKind, RecallEntry, RecallOrchestrator};
USED BY  : Executed by cargo test for or-recall.
```

```text
PATH     : crates/or-recall/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-recall.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
