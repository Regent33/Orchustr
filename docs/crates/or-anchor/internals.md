# or-anchor Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-anchor/Cargo.toml
TYPE     : toml
LOC      : 14
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-anchor or by its direct callers.
```

```text
PATH     : crates/or-anchor/src/application/mod.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-anchor/src/application/orchestrators.rs
TYPE     : rs
LOC      : 39
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub struct AnchorOrchestrator;; pub async fn index_document(; pub async fn retrieve(
IMPORTS  : crate::domain::entities::{AnchorChunk, RetrievedChunk};; crate::domain::errors::AnchorError;; crate::infra::implementations::AnchorPipeline;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-anchor/src/domain/contracts.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-anchor/src/domain/entities.rs
TYPE     : rs
LOC      : 12
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub struct AnchorChunk {; pub struct RetrievedChunk {
IMPORTS  : serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-anchor/src/domain/errors.rs
TYPE     : rs
LOC      : 7
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum AnchorError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-anchor/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-anchor/src/infra/adapters.rs
TYPE     : rs
LOC      : 33
STATUS   : 🟢 Complete
PURPOSE  : Contains helper adapters and translation utilities.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::entities::AnchorChunk;; std::collections::hash_map::DefaultHasher;; std::hash::{Hash, Hasher};
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-anchor/src/infra/implementations.rs
TYPE     : rs
LOC      : 74
STATUS   : 🟢 Complete
PURPOSE  : Contains the main concrete implementations.
EXPORTS  : pub struct AnchorPipeline {; pub fn new() -> Self {; pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {; pub async fn index_document(; pub async fn retrieve(
IMPORTS  : crate::domain::entities::{AnchorChunk, RetrievedChunk};; crate::domain::errors::AnchorError;; crate::infra::adapters::{chunk_text, embed};; or_core::{InMemoryVectorStore, VectorStore};
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-anchor/src/infra/mod.rs
TYPE     : rs
LOC      : 4
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-anchor/src/lib.rs
TYPE     : rs
LOC      : 7
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::orchestrators::AnchorOrchestrator;; pub use domain::entities::{AnchorChunk, RetrievedChunk};; pub use domain::errors::AnchorError;; pub use infra::implementations::AnchorPipeline;
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-anchor/tests/unit/mod.rs
TYPE     : rs
LOC      : 28
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_anchor::{AnchorOrchestrator, AnchorPipeline};
USED BY  : Executed by cargo test for or-anchor.
```

```text
PATH     : crates/or-anchor/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-anchor.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
