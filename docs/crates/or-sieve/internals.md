# or-sieve Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-sieve/Cargo.toml
TYPE     : toml
LOC      : 15
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-sieve or by its direct callers.
```

```text
PATH     : crates/or-sieve/src/application/mod.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-sieve/src/application/orchestrators.rs
TYPE     : rs
LOC      : 34
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub struct SieveOrchestrator;; pub fn parse_structured<T: JsonSchemaOutput, P: StructuredParser<T>>(; pub fn parse_text(&self, parser: &TextParser, raw: &str) -> Result<PlainText, SieveError> {
IMPORTS  : crate::domain::contracts::{JsonSchemaOutput, StructuredParser};; crate::domain::entities::PlainText;; crate::domain::errors::SieveError;; crate::infra::implementations::TextParser;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-sieve/src/domain/contracts.rs
TYPE     : rs
LOC      : 19
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : pub trait JsonSchemaOutput:; pub trait StructuredParser<T: JsonSchemaOutput>: Send + Sync {
IMPORTS  : crate::domain::errors::SieveError;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-sieve/src/domain/entities.rs
TYPE     : rs
LOC      : 5
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub struct PlainText {
IMPORTS  : serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-sieve/src/domain/errors.rs
TYPE     : rs
LOC      : 13
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum SieveError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-sieve/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-sieve/src/infra/adapters.rs
TYPE     : rs
LOC      : 104
STATUS   : 🟢 Complete
PURPOSE  : Contains helper adapters and translation utilities.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::contracts::JsonSchemaOutput;; crate::domain::errors::SieveError;; schemars::schema::{InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec};; serde_json::Value;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-sieve/src/infra/implementations.rs
TYPE     : rs
LOC      : 39
STATUS   : 🟢 Complete
PURPOSE  : Contains the main concrete implementations.
EXPORTS  : pub struct JsonParser<T: JsonSchemaOutput> {; pub fn new() -> Self {; pub struct TextParser;; pub fn parse(&self, raw: &str) -> Result<PlainText, SieveError> {
IMPORTS  : crate::domain::contracts::{JsonSchemaOutput, StructuredParser};; crate::domain::entities::PlainText;; crate::domain::errors::SieveError;; crate::infra::adapters::validate_against_schema;; std::marker::PhantomData;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-sieve/src/infra/mod.rs
TYPE     : rs
LOC      : 4
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-sieve/src/lib.rs
TYPE     : rs
LOC      : 8
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::orchestrators::SieveOrchestrator;; pub use domain::contracts::{JsonSchemaOutput, StructuredParser};; pub use domain::entities::PlainText;; pub use domain::errors::SieveError;; pub use infra::implementations::{JsonParser, TextParser};
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-sieve/tests/unit/mod.rs
TYPE     : rs
LOC      : 44
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_sieve::{JsonParser, SieveError, SieveOrchestrator, TextParser};; schemars::JsonSchema;; serde::{Deserialize, Serialize};
USED BY  : Executed by cargo test for or-sieve.
```

```text
PATH     : crates/or-sieve/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-sieve.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
