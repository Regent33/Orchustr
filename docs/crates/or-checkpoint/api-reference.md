# or-checkpoint API Reference

This page documents the main public surface re-exported by `or-checkpoint/src/lib.rs` and the key entry points behind those re-exports. 
### `CheckpointRecord`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-checkpoint/src/domain/entities.rs |
| **Status** | 🟢 |

**Description**: Serializable record stored for a paused execution point.

**Signature**
```rust
pub struct CheckpointRecord<T> { pub graph_id: String, pub checkpoint_id: String, pub resume_from: String, pub state: T }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `CheckpointGate`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-checkpoint/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: Concrete pause/resume component backed by persistence.

**Signature**
```rust
pub struct CheckpointGate<B> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `CheckpointOrchestrator`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-checkpoint/src/application/orchestrators.rs |
| **Status** | 🟢 |

**Description**: Application helper for pause and resume flows.

**Signature**
```rust
pub struct CheckpointOrchestrator;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `CheckpointError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-checkpoint/src/domain/errors.rs |
| **Status** | 🟢 |

**Description**: Error type for storage, serialization, and missing-checkpoint failures.

**Signature**
```rust
pub enum CheckpointError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- Durability depends entirely on the supplied persistence backend.
