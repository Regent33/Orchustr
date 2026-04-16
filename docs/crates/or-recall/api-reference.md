# or-recall API Reference

This page documents the main public surface re-exported by `or-recall/src/lib.rs` and the key entry points behind those re-exports. 
### `MemoryKind`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-recall/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Categorizes recall entries as short-term, long-term, or episodic.

**Signature**
```rust
pub enum MemoryKind { ShortTerm, LongTerm, Episodic }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `RecallEntry`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-recall/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Serializable memory record with metadata.

**Signature**
```rust
pub struct RecallEntry { pub id: String, pub kind: MemoryKind, pub content: String, pub metadata: HashMap<String, String> }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `RecallStore`
| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | crates/or-recall/src/domain/contracts.rs |
| **Status** | 🟡 |

**Description**: Async storage and listing contract for memory backends.

**Signature**
```rust
pub trait RecallStore: Send + Sync + 'static
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `InMemoryRecallStore`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-recall/src/infra/implementations.rs |
| **Status** | 🟡 |

**Description**: Synchronized in-memory memory store.

**Signature**
```rust
pub struct InMemoryRecallStore { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `SqliteRecallStore`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-recall/src/infra/sqlite.rs |
| **Status** | 🟡 |

**Description**: Feature-gated SQLite-backed memory store.

**Signature**
```rust
pub struct SqliteRecallStore { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `RecallOrchestrator`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-recall/src/application/orchestrators.rs |
| **Status** | 🟡 |

**Description**: Application helper for remember and recall operations.

**Signature**
```rust
pub struct RecallOrchestrator;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `RecallError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-recall/src/domain/errors.rs |
| **Status** | 🟡 |

**Description**: Error type for storage and serialization failures.

**Signature**
```rust
pub enum RecallError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- SQLite support is feature-gated and not the default runtime path.
- There is no vector-memory or retrieval scoring layer in this crate today.
