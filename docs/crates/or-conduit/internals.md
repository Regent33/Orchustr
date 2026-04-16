# or-conduit Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-conduit/Cargo.toml
TYPE     : toml
LOC      : 17
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-conduit or by its direct callers.
```

```text
PATH     : crates/or-conduit/src/application/mod.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-conduit/src/application/orchestrators.rs
TYPE     : rs
LOC      : 47
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub struct ConduitOrchestrator;; pub fn prepare_text_request(; pub async fn execute_completion<P: ConduitProvider>(
IMPORTS  : crate::domain::contracts::ConduitProvider;; crate::domain::entities::{CompletionMessage, CompletionResponse, MessageRole};; crate::domain::errors::ConduitError;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-conduit/src/domain/contracts.rs
TYPE     : rs
LOC      : 37
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : pub type TextStream = Pin<Box<dyn Stream<Item = Result<String, ConduitError>> + Send>>;; pub trait ConduitProvider: Send + Sync + 'static {
IMPORTS  : crate::domain::entities::{CompletionMessage, CompletionResponse, MessageRole};; crate::domain::errors::ConduitError;; futures::{Stream, stream};; std::pin::Pin;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-conduit/src/domain/entities.rs
TYPE     : rs
LOC      : 52
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub enum ContentPart {; pub enum ImageDetail {; pub struct CompletionMessage {; pub fn single_text(role: MessageRole, text: impl Into<String>) -> Self {; pub enum MessageRole {; pub struct CompletionResponse {; pub enum FinishReason {
IMPORTS  : or_core::TokenUsage;; serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-conduit/src/domain/errors.rs
TYPE     : rs
LOC      : 26
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum ConduitError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-conduit/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-conduit/src/infra/adapters.rs
TYPE     : rs
LOC      : 173
STATUS   : 🟢 Complete
PURPOSE  : Contains helper adapters and translation utilities.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::entities::{; crate::domain::errors::ConduitError;; or_core::TokenUsage;; serde_json::{Value, json};
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-conduit/src/infra/http.rs
TYPE     : rs
LOC      : 108
STATUS   : 🟢 Complete
PURPOSE  : Contains shared HTTP transport logic.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::entities::{CompletionMessage, CompletionResponse};; crate::domain::errors::ConduitError;; crate::infra::adapters::estimate_prompt_tokens;; or_core::{CoreOrchestrator, RetryPolicy, TokenBudget};; reqwest::Client;; reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-conduit/src/infra/implementations.rs
TYPE     : rs
LOC      : 155
STATUS   : 🟢 Complete
PURPOSE  : Contains the main concrete implementations.
EXPORTS  : pub struct OpenAiConduit {; pub struct AnthropicConduit {; pub fn with_retry(mut self, retry_policy: RetryPolicy) -> Self {; pub fn with_budget(mut self, token_budget: TokenBudget) -> Self {; pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {; pub fn from_env() -> Result<Self, ConduitError> {; pub fn with_retry(mut self, retry_policy: RetryPolicy) -> Self {; pub fn with_budget(mut self, token_budget: TokenBudget) -> Self {
IMPORTS  : crate::domain::contracts::ConduitProvider;; crate::domain::entities::{CompletionMessage, CompletionResponse};; crate::domain::errors::ConduitError;; crate::infra::adapters::{; crate::infra::http::{HttpConduit, anthropic_headers, openai_headers, required_env};; or_core::{RetryPolicy, TokenBudget};
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-conduit/src/infra/mod.rs
TYPE     : rs
LOC      : 6
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-conduit/src/lib.rs
TYPE     : rs
LOC      : 10
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::orchestrators::ConduitOrchestrator;; pub use domain::contracts::{ConduitProvider, TextStream};; pub use domain::entities::{; pub use domain::errors::ConduitError;; pub use infra::implementations::{AnthropicConduit, OpenAiConduit};
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-conduit/tests/unit/mod.rs
TYPE     : rs
LOC      : 139
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : futures::StreamExt;; or_conduit::{; or_core::TokenUsage;
USED BY  : Executed by cargo test for or-conduit.
```

```text
PATH     : crates/or-conduit/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-conduit.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
