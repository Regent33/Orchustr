# or-relay API Reference

This page documents the main public surface re-exported by `or-relay/src/lib.rs` and the key entry points behind those re-exports. 
### `RelayBuilder`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-relay/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: Builder for named parallel branches.

**Signature**
```rust
pub struct RelayBuilder<T: OrchState> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `RelayPlan`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-relay/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: Executable plan containing branch handlers.

**Signature**
```rust
pub struct RelayPlan<T: OrchState> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `RelayExecutor`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-relay/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: Runtime that executes all relay branches concurrently.

**Signature**
```rust
pub struct RelayExecutor;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `RelayOrchestrator`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-relay/src/application/orchestrators.rs |
| **Status** | 🟢 |

**Description**: Application helper for relay execution with tracing.

**Signature**
```rust
pub struct RelayOrchestrator;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `RelayError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-relay/src/domain/errors.rs |
| **Status** | 🟢 |

**Description**: Error type for malformed plans and branch execution failures.

**Signature**
```rust
pub enum RelayError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- Parallelism is intra-process and memory-local; no distributed execution layer exists in this crate.
