# or-beacon Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-beacon/Cargo.toml
TYPE     : toml
LOC      : 14
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-beacon or by its direct callers.
```

```text
PATH     : crates/or-beacon/src/application/mod.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-beacon/src/application/orchestrators.rs
TYPE     : rs
LOC      : 34
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub struct PromptOrchestrator;; pub fn build_template(&self, raw_template: &str) -> Result<PromptTemplate, BeaconError> {; pub fn render_template<T: Serialize>(
IMPORTS  : crate::domain::entities::PromptTemplate;; crate::domain::errors::BeaconError;; crate::infra::implementations::PromptBuilder;; serde::Serialize;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-beacon/src/domain/contracts.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-beacon/src/domain/entities.rs
TYPE     : rs
LOC      : 6
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub struct PromptTemplate {
IMPORTS  : serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-beacon/src/domain/errors.rs
TYPE     : rs
LOC      : 13
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum BeaconError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-beacon/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-beacon/src/infra/adapters.rs
TYPE     : rs
LOC      : 34
STATUS   : 🟢 Complete
PURPOSE  : Contains helper adapters and translation utilities.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::errors::BeaconError;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-beacon/src/infra/implementations.rs
TYPE     : rs
LOC      : 53
STATUS   : 🟢 Complete
PURPOSE  : Contains the main concrete implementations.
EXPORTS  : pub struct PromptBuilder {; pub fn new() -> Self {; pub fn template(mut self, template: impl Into<String>) -> Self {; pub fn build(self) -> Result<PromptTemplate, BeaconError> {; pub fn render<T: Serialize>(&self, context: &T) -> Result<String, BeaconError> {
IMPORTS  : crate::domain::entities::PromptTemplate;; crate::domain::errors::BeaconError;; crate::infra::adapters::{extract_variables, sanitize_text};; serde::Serialize;; serde_json::Value;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-beacon/src/infra/mod.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-beacon/src/lib.rs
TYPE     : rs
LOC      : 7
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::orchestrators::PromptOrchestrator;; pub use domain::entities::PromptTemplate;; pub use domain::errors::BeaconError;; pub use infra::implementations::PromptBuilder;
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-beacon/tests/unit/mod.rs
TYPE     : rs
LOC      : 39
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_beacon::{BeaconError, PromptBuilder, PromptOrchestrator};; serde_json::json;
USED BY  : Executed by cargo test for or-beacon.
```

```text
PATH     : crates/or-beacon/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-beacon.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
