# or-loom API Reference

This page documents the main public surface re-exported by `or-loom/src/lib.rs` and the key entry points behind those re-exports. 
### `NodeResult`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-loom/src/domain/entities.rs |
| **Status** | 🟢 |

**Description**: Represents how a node advances execution: advance, branch, or pause.

**Signature**
```rust
pub enum NodeResult<T> { Advance(T), Branch { state: T, next: String }, Pause { checkpoint_id: String, state: T } }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `GraphBuilder`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-loom/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: Builder for graph nodes, edges, entry, and exit configuration.

**Signature**
```rust
pub struct GraphBuilder<T: OrchState> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `ExecutionGraph`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-loom/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: Executable state graph produced by `GraphBuilder::build`.

**Signature**
```rust
pub struct ExecutionGraph<T: OrchState> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `LoomOrchestrator`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-loom/src/application/orchestrators.rs |
| **Status** | 🟢 |

**Description**: Application helper for executing graphs with tracing.

**Signature**
```rust
pub struct LoomOrchestrator;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `LoomError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-loom/src/domain/errors.rs |
| **Status** | 🟢 |

**Description**: Error type for graph validation, execution, and pause/branch issues.

**Signature**
```rust
pub enum LoomError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- Node futures are intentionally not required to be `Send`, which keeps sequential execution flexible but limits some multi-thread assumptions.
