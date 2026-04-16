# or-bridge Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-bridge/Cargo.toml
TYPE     : toml
LOC      : 24
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-bridge or by its direct callers.
```

```text
PATH     : crates/or-bridge/src/application/mod.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-bridge/src/application/orchestrators.rs
TYPE     : rs
LOC      : 24
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub fn render_prompt_json(template: &str, context_json: &str) -> Result<String, BridgeError> {; pub fn normalize_state_json(raw_state: &str) -> Result<String, BridgeError> {
IMPORTS  : crate::domain::errors::BridgeError;; crate::infra::implementations::{normalize, render};
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-bridge/src/domain/contracts.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-bridge/src/domain/entities.rs
TYPE     : rs
LOC      : 5
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub struct BridgeState {
IMPORTS  : serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-bridge/src/domain/errors.rs
TYPE     : rs
LOC      : 11
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum BridgeError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-bridge/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-bridge/src/infra/adapters.rs
TYPE     : rs
LOC      : 14
STATUS   : 🟢 Complete
PURPOSE  : Contains helper adapters and translation utilities.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::errors::BridgeError;; or_core::DynState;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-bridge/src/infra/implementations.rs
TYPE     : rs
LOC      : 15
STATUS   : 🟢 Complete
PURPOSE  : Contains the main concrete implementations.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::errors::BridgeError;; crate::infra::adapters::{dyn_state_from_json, dyn_state_to_json};; or_beacon::PromptBuilder;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-bridge/src/infra/mod.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-bridge/src/lib.rs
TYPE     : rs
LOC      : 9
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::orchestrators::{normalize_state_json, render_prompt_json};; pub use domain::errors::BridgeError;
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-bridge/src/node.rs
TYPE     : rs
LOC      : 16
STATUS   : 🟢 Complete
PURPOSE  : Implements a focused part of the crate.
EXPORTS  : pub fn version() -> String {; pub fn render_prompt_json(template: String, context_json: String) -> napi::Result<String> {; pub fn normalize_state_json(raw_state: String) -> napi::Result<String> {
IMPORTS  : napi_derive::napi;
USED BY  : Consumed within or-bridge or by its direct callers.
```

```text
PATH     : crates/or-bridge/src/python.rs
TYPE     : rs
LOC      : 23
STATUS   : 🟢 Complete
PURPOSE  : Implements a focused part of the crate.
EXPORTS  : `(none)`
IMPORTS  : pyo3::exceptions::PyValueError;; pyo3::prelude::*;; pyo3::wrap_pyfunction;
USED BY  : Consumed within or-bridge or by its direct callers.
```

```text
PATH     : crates/or-bridge/tests/unit/mod.rs
TYPE     : rs
LOC      : 21
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_bridge::{BridgeError, normalize_state_json, render_prompt_json};
USED BY  : Executed by cargo test for or-bridge.
```

```text
PATH     : crates/or-bridge/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-bridge.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
