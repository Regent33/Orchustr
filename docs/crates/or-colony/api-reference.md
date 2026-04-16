# or-colony API Reference

This page documents the main public surface re-exported by `or-colony/src/lib.rs` and the key entry points behind those re-exports. 
### `ColonyAgentTrait`
| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | crates/or-colony/src/domain/contracts.rs |
| **Status** | 🟡 |

**Description**: Async interface implemented by a colony member runtime.

**Signature**
```rust
pub trait ColonyAgentTrait: Send + Sync + 'static
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `ColonyMember`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-colony/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Metadata describing a named colony participant and role.

**Signature**
```rust
pub struct ColonyMember { pub name: String, pub role: String }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `ColonyMessage`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-colony/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Represents a message in the multi-agent transcript.

**Signature**
```rust
pub struct ColonyMessage { pub sender: String, pub role: String, pub content: String }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `ColonyResult`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-colony/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Aggregated outcome containing final state, messages, and summary.

**Signature**
```rust
pub struct ColonyResult { pub state: DynState, pub messages: Vec<ColonyMessage>, pub summary: String }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `ColonyOrchestrator`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-colony/src/application/orchestrators.rs |
| **Status** | 🟡 |

**Description**: Application entry point for member registration and coordination.

**Signature**
```rust
pub struct ColonyOrchestrator { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `ColonyError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-colony/src/domain/errors.rs |
| **Status** | 🟡 |

**Description**: Error type for duplicate members, empty rosters, and malformed state.

**Signature**
```rust
pub enum ColonyError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- Execution is sequential rather than concurrent.
- The initial state contract currently requires a `task` field.
