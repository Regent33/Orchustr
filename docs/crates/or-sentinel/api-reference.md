# or-sentinel API Reference

This page documents the main public surface re-exported by `or-sentinel/src/lib.rs`.

### `StepOutcome`
- **Kind**: enum
- **File**: `crates/or-sentinel/src/domain/entities.rs`
- **Description**: Outcome of a single agent step or full run.

### `SentinelConfig`
- **Kind**: struct
- **File**: `crates/or-sentinel/src/domain/entities.rs`
- **Description**: Configuration for max steps, token budget, and tool retry policy.

### `PlanStep`
- **Kind**: struct
- **File**: `crates/or-sentinel/src/domain/entities.rs`
- **Description**: Single step in the plan-and-execute flow.

### `LoopTopology`
- **Kind**: trait
- **File**: `crates/or-sentinel/src/topology.rs`
- **Description**: Additive extension point for custom loop shapes such as ReAct, reflection, or plan-execute variants.

### `SentinelAgentBuilder`
- **Kind**: struct
- **File**: `crates/or-sentinel/src/builder.rs`
- **Description**: Builder path for constructing sentinel agents from built-in or custom loop topologies.

### `ReActTopology`
- **Kind**: struct
- **File**: `crates/or-sentinel/src/topologies/react.rs`
- **Description**: Built-in topology equivalent to the legacy `SentinelAgent::new(...)` graph shape.

### `PlanExecuteTopology`
- **Kind**: struct
- **File**: `crates/or-sentinel/src/topologies/plan_execute.rs`
- **Description**: Built-in topology for plan, execute-step, and completion-check loops.

### `ReflectionTopology`
- **Kind**: struct
- **File**: `crates/or-sentinel/src/topologies/reflection.rs`
- **Description**: Built-in topology for draft, critique, and bounded revision loops.

### `SentinelAgent`
- **Kind**: struct
- **File**: `crates/or-sentinel/src/infra/implementations/sentinel_agent.rs`
- **Description**: Graph-backed think/act agent runtime over a provider and forge registry.

### `PlanExecuteAgent`
- **Kind**: struct
- **File**: `crates/or-sentinel/src/infra/implementations/plan_execute.rs`
- **Description**: Higher-level planner that delegates individual steps to a sentinel worker.

### `SentinelOrchestrator`
- **Kind**: struct
- **File**: `crates/or-sentinel/src/application/orchestrators.rs`
- **Description**: Application helper for agent setup and top-level execution.

### `SentinelError`
- **Kind**: enum
- **File**: `crates/or-sentinel/src/domain/errors.rs`
- **Description**: Error type for malformed state, provider/tool failures, and serialization issues.

## Known Gaps & Limitations

- The legacy `SentinelAgent::new(...)` path intentionally keeps the fixed ReAct topology for backward compatibility.
- Custom loop topologies are Rust-first today; bindings do not expose them yet.
