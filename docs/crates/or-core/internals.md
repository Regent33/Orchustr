# or-core Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-core/Cargo.toml
TYPE     : toml
LOC      : 15
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-core or by its direct callers.
```

```text
PATH     : crates/or-core/src/application/mod.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-core/src/application/orchestrators.rs
TYPE     : rs
LOC      : 71
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub struct CoreOrchestrator {; pub fn new() -> Self {; pub fn enforce_completion_budget(; pub fn next_retry_delay(
IMPORTS  : crate::domain::entities::{BackoffStrategy, RetryPolicy, TokenBudget};; crate::domain::errors::CoreError;; std::time::Duration;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-core/src/domain/contracts.rs
TYPE     : rs
LOC      : 25
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : pub type DynState = HashMap<String, Value>;; pub trait OrchState:; pub trait PersistenceBackend: Send + Sync + 'static {; pub trait VectorStore: Send + Sync + 'static {
IMPORTS  : crate::domain::entities::VectorRecord;; crate::domain::errors::CoreError;; serde_json::Value;; std::collections::HashMap;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-core/src/domain/entities.rs
TYPE     : rs
LOC      : 81
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub struct TokenUsage {; pub struct TokenBudget {; pub fn fits(&self, prompt_tokens: u32, completion_tokens: u32) -> bool {; pub struct RetryPolicy {; pub fn no_retry() -> Self {; pub fn default_llm() -> Self {; pub enum BackoffStrategy {; pub fn delay_ms(&self, policy: &RetryPolicy, attempt: u32) -> u64 {
IMPORTS  : rand::Rng;; serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-core/src/domain/errors.rs
TYPE     : rs
LOC      : 11
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum CoreError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-core/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-core/src/infra/adapters.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Contains helper adapters and translation utilities.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-core/src/infra/implementations.rs
TYPE     : rs
LOC      : 80
STATUS   : 🟢 Complete
PURPOSE  : Contains the main concrete implementations.
EXPORTS  : pub struct InMemoryPersistenceBackend {; pub fn new() -> Self {; pub struct InMemoryVectorStore {; pub fn new() -> Self {
IMPORTS  : crate::domain::contracts::{PersistenceBackend, VectorStore};; crate::domain::entities::VectorRecord;; crate::domain::errors::CoreError;; serde_json::Value;; std::cmp::Ordering;; std::collections::HashMap;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-core/src/infra/mod.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-core/src/lib.rs
TYPE     : rs
LOC      : 8
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::orchestrators::CoreOrchestrator;; pub use domain::contracts::{DynState, OrchState, PersistenceBackend, VectorStore};; pub use domain::entities::{BackoffStrategy, RetryPolicy, TokenBudget, TokenUsage, VectorRecord};; pub use domain::errors::CoreError;; pub use infra::implementations::{InMemoryPersistenceBackend, InMemoryVectorStore};
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-core/tests/unit/mod.rs
TYPE     : rs
LOC      : 73
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_core::{; serde_json::json;
USED BY  : Executed by cargo test for or-core.
```

```text
PATH     : crates/or-core/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-core.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
