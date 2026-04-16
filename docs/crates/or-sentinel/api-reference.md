# or-sentinel API Reference

This page documents the main public surface re-exported by `or-sentinel/src/lib.rs` and the key entry points behind those re-exports. 
### `StepOutcome`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-sentinel/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Outcome of a single agent step or full run.

**Signature**
```rust
pub enum StepOutcome { ToolCall { ... }, FinalAnswer { ... }, StepLimitReached { ... } }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `SentinelConfig`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-sentinel/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Configuration for max steps, token budget, and tool retry policy.

**Signature**
```rust
pub struct SentinelConfig { pub max_steps: u32, pub step_budget: TokenBudget, pub tool_retry: RetryPolicy }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `PlanStep`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-sentinel/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Single step in the plan-and-execute flow.

**Signature**
```rust
pub struct PlanStep { pub step_index: u32, pub description: String }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `SentinelAgentTrait`
| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | crates/or-sentinel/src/domain/contracts.rs |
| **Status** | 🟡 |

**Description**: Async contract for running or stepping a sentinel agent.

**Signature**
```rust
pub trait SentinelAgentTrait: Send + Sync + 'static
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `PlanExecuteAgentTrait`
| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | crates/or-sentinel/src/domain/contracts.rs |
| **Status** | 🟡 |

**Description**: Async contract for plan creation and plan execution.

**Signature**
```rust
pub trait PlanExecuteAgentTrait: Send + Sync + 'static
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `SentinelAgent`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-sentinel/src/infra/implementations/sentinel_agent.rs |
| **Status** | 🟡 |

**Description**: Graph-backed think-act agent runtime over a provider and forge registry.

**Signature**
```rust
pub struct SentinelAgent<P> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `PlanExecuteAgent`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-sentinel/src/infra/implementations/plan_execute.rs |
| **Status** | 🟡 |

**Description**: Higher-level planner that delegates individual steps to a sentinel worker.

**Signature**
```rust
pub struct PlanExecuteAgent<P> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `SentinelOrchestrator`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-sentinel/src/application/orchestrators.rs |
| **Status** | 🟡 |

**Description**: Application helper for agent setup and top-level execution.

**Signature**
```rust
pub struct SentinelOrchestrator;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `SentinelError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-sentinel/src/domain/errors.rs |
| **Status** | 🟡 |

**Description**: Error type for malformed state, provider/tool failures, and serialization issues.

**Signature**
```rust
pub enum SentinelError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- Decision and plan parsing depend on expected JSON-ish model output rather than provider-enforced structured mode.
- The internal graph always uses the fixed think/act/exit topology in the current implementation.
