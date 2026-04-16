# or-sentinel Internals  ## Layering  - `domain/`: data contracts, entities, and crate-local error types. - `infra/`: concrete implementations and helper adapters. - `application/`: orchestration entry points and tracing spans.  ## File Registry 
```text
PATH     : crates/or-sentinel/Cargo.toml
TYPE     : toml
LOC      : 21
STATUS   : 🟢 Complete
PURPOSE  : Defines the package manifest, feature flags, and dependencies.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed within or-sentinel or by its direct callers.
```

```text
PATH     : crates/or-sentinel/src/application/mod.rs
TYPE     : rs
LOC      : 1
STATUS   : 🟢 Complete
PURPOSE  : Wires the application module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-sentinel/src/application/orchestrators.rs
TYPE     : rs
LOC      : 40
STATUS   : 🟢 Complete
PURPOSE  : Implements the application entry points and tracing-oriented orchestration logic.
EXPORTS  : pub struct SentinelOrchestrator;; pub async fn run_agent<A: SentinelAgentTrait>(; pub async fn run_planned_agent<A: PlanExecuteAgentTrait>(
IMPORTS  : crate::domain::contracts::{PlanExecuteAgentTrait, SentinelAgentTrait};; crate::domain::entities::{SentinelConfig, StepOutcome};; crate::domain::errors::SentinelError;; or_core::DynState;
USED BY  : Called from the crate public API and runtime entry points.
```

```text
PATH     : crates/or-sentinel/src/domain/contracts.rs
TYPE     : rs
LOC      : 22
STATUS   : 🟢 Complete
PURPOSE  : Defines traits and abstraction contracts.
EXPORTS  : pub trait SentinelAgentTrait: Send + Sync + 'static {; pub trait PlanExecuteAgentTrait: Send + Sync + 'static {
IMPORTS  : crate::domain::entities::{PlanStep, SentinelConfig, StepOutcome};; crate::domain::errors::SentinelError;; or_core::DynState;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-sentinel/src/domain/entities.rs
TYPE     : rs
LOC      : 29
STATUS   : 🟢 Complete
PURPOSE  : Defines serializable domain entities.
EXPORTS  : pub enum StepOutcome {; pub struct SentinelConfig {; pub struct PlanStep {
IMPORTS  : or_core::{DynState, RetryPolicy, TokenBudget};; serde::{Deserialize, Serialize};
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-sentinel/src/domain/errors.rs
TYPE     : rs
LOC      : 21
STATUS   : 🟢 Complete
PURPOSE  : Defines crate-specific error types.
EXPORTS  : pub enum SentinelError {
IMPORTS  : serde::{Deserialize, Serialize};; thiserror::Error;
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-sentinel/src/domain/mod.rs
TYPE     : rs
LOC      : 3
STATUS   : 🟢 Complete
PURPOSE  : Wires the domain module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application and infra layers inside the same crate.
```

```text
PATH     : crates/or-sentinel/src/infra/adapters.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Contains helper adapters and translation utilities.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-sentinel/src/infra/adapters/parsing.rs
TYPE     : rs
LOC      : 70
STATUS   : 🟢 Complete
PURPOSE  : Implements a focused part of the crate.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::entities::{PlanStep, SentinelConfig};; crate::domain::errors::SentinelError;; or_conduit::{CompletionMessage, ContentPart};; serde::{Deserialize, Serialize};
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-sentinel/src/infra/adapters/state.rs
TYPE     : rs
LOC      : 126
STATUS   : 🟢 Complete
PURPOSE  : Implements a focused part of the crate.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::entities::SentinelConfig;; crate::domain::errors::SentinelError;; crate::infra::adapters::parsing::config_to_value;; or_conduit::{CompletionMessage, MessageRole};; or_core::DynState;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-sentinel/src/infra/implementations.rs
TYPE     : rs
LOC      : 5
STATUS   : 🟢 Complete
PURPOSE  : Contains the main concrete implementations.
EXPORTS  : pub use plan_execute::PlanExecuteAgent;; pub use sentinel_agent::SentinelAgent;
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-sentinel/src/infra/implementations/plan_execute.rs
TYPE     : rs
LOC      : 89
STATUS   : 🟢 Complete
PURPOSE  : Implements a focused part of the crate.
EXPORTS  : pub struct PlanExecuteAgent<P> {; pub fn new(planner: P, registry: ForgeRegistry) -> Result<Self, SentinelError> {
IMPORTS  : crate::domain::contracts::PlanExecuteAgentTrait;; crate::domain::entities::{PlanStep, SentinelConfig, StepOutcome};; crate::domain::errors::SentinelError;; crate::infra::adapters::parsing::parse_plan;; crate::infra::adapters::state::{messages_from_state, write_messages};; crate::infra::implementations::sentinel_agent::SentinelAgent;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-sentinel/src/infra/implementations/sentinel_agent.rs
TYPE     : rs
LOC      : 174
STATUS   : 🟢 Complete
PURPOSE  : Implements a focused part of the crate.
EXPORTS  : pub struct SentinelAgent<P> {; pub fn new(provider: P, registry: ForgeRegistry) -> Result<Self, SentinelError> {
IMPORTS  : crate::domain::contracts::SentinelAgentTrait;; crate::domain::entities::{SentinelConfig, StepOutcome};; crate::domain::errors::SentinelError;; crate::infra::adapters::parsing::{ModelDecision, approx_prompt_tokens, parse_decision};; crate::infra::adapters::state::{; crate::infra::implementations::support::{invoke_with_retry, node_error};
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-sentinel/src/infra/implementations/support.rs
TYPE     : rs
LOC      : 34
STATUS   : 🟢 Complete
PURPOSE  : Implements a focused part of the crate.
EXPORTS  : `(none)`
IMPORTS  : crate::domain::entities::SentinelConfig;; crate::domain::errors::SentinelError;; or_core::CoreOrchestrator;; or_forge::ForgeRegistry;; or_loom::LoomError;
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-sentinel/src/infra/mod.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Wires the infra module into the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Consumed by the application layer and re-exported public surface where applicable.
```

```text
PATH     : crates/or-sentinel/src/lib.rs
TYPE     : rs
LOC      : 7
STATUS   : 🟢 Complete
PURPOSE  : Provides the public crate surface through re-exports.
EXPORTS  : pub use application::orchestrators::SentinelOrchestrator;; pub use domain::entities::{PlanStep, SentinelConfig, StepOutcome};; pub use domain::errors::SentinelError;; pub use infra::implementations::{PlanExecuteAgent, SentinelAgent};
IMPORTS  : `(none)`
USED BY  : Downstream crates and bindings import the crate through this file.
```

```text
PATH     : crates/or-sentinel/tests/unit/mod.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Groups the primary unit tests for the crate. Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-sentinel.
```

```text
PATH     : crates/or-sentinel/tests/unit/orchestrators.rs
TYPE     : rs
LOC      : 110
STATUS   : 🟢 Complete
PURPOSE  : Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_core::DynState;; or_sentinel::domain::contracts::{PlanExecuteAgentTrait, SentinelAgentTrait};; or_sentinel::{SentinelConfig, SentinelError, SentinelOrchestrator, StepOutcome};
USED BY  : Executed by cargo test for or-sentinel.
```

```text
PATH     : crates/or-sentinel/tests/unit/runtime.rs
TYPE     : rs
LOC      : 151
STATUS   : 🟢 Complete
PURPOSE  : Contains focused test scenarios for the crate.
EXPORTS  : `(none)`
IMPORTS  : or_conduit::{; or_core::{RetryPolicy, TokenBudget, TokenUsage};; or_forge::{ForgeRegistry, ForgeTool};; or_sentinel::domain::contracts::{PlanExecuteAgentTrait, SentinelAgentTrait};; or_sentinel::{PlanExecuteAgent, SentinelAgent, SentinelConfig, StepOutcome};; schemars::schema::RootSchema;
USED BY  : Executed by cargo test for or-sentinel.
```

```text
PATH     : crates/or-sentinel/tests/unit_suite.rs
TYPE     : rs
LOC      : 2
STATUS   : 🟢 Complete
PURPOSE  : Acts as the Cargo test entry point for the crate test tree.
EXPORTS  : `(none)`
IMPORTS  : `(none)`
USED BY  : Executed by cargo test for or-sentinel.
```

## Test Shape

- `tests/unit_suite.rs` wires the crate test tree into Cargo.
- Additional unit coverage lives under `tests/unit/`.

⚠️ Known Gaps & Limitations
- This page inventories the current file tree only; it does not infer future module plans beyond what the source already contains.
- Generated artifacts outside `crates/` are intentionally excluded from these crate internals pages.
