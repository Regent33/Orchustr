# or-core API Reference

This page documents the main public surface re-exported by `or-core/src/lib.rs` and the key entry points behind those re-exports. 
### `DynState`
| Property | Value |
|---|---|
| **Kind** | type alias |
| **Visibility** | pub |
| **File** | crates/or-core/src/domain/contracts.rs |
| **Status** | 🟢 |

**Description**: Schemaless state map used at graph, agent, and binding boundaries.

**Signature**
```rust
pub type DynState = HashMap<String, serde_json::Value>;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `OrchState`
| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | crates/or-core/src/domain/contracts.rs |
| **Status** | 🟢 |

**Description**: Core state trait with clone, serde, and merge semantics.

**Signature**
```rust
pub trait OrchState: Clone + Send + Sync + serde::Serialize + serde::de::DeserializeOwned + 'static
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `PersistenceBackend`
| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | crates/or-core/src/domain/contracts.rs |
| **Status** | 🟢 |

**Description**: Async key/value persistence contract.

**Signature**
```rust
pub trait PersistenceBackend: Send + Sync + 'static
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `VectorStore`
| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | crates/or-core/src/domain/contracts.rs |
| **Status** | 🟢 |

**Description**: Async vector index contract.

**Signature**
```rust
pub trait VectorStore: Send + Sync + 'static
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `TokenBudget`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-core/src/domain/entities.rs |
| **Status** | 🟢 |

**Description**: Represents context and completion token limits.

**Signature**
```rust
pub struct TokenBudget { pub max_context_tokens: u32, pub max_completion_tokens: u32 }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `RetryPolicy`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-core/src/domain/entities.rs |
| **Status** | 🟢 |

**Description**: Holds retry attempt and delay parameters.

**Signature**
```rust
pub struct RetryPolicy { pub max_attempts: u32, pub base_delay_ms: u64, pub max_delay_ms: u64, pub jitter: bool }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `BackoffStrategy`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-core/src/domain/entities.rs |
| **Status** | 🟢 |

**Description**: Calculates retry delays.

**Signature**
```rust
pub enum BackoffStrategy { Exponential, ExponentialFullJitter, Fixed }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `CoreOrchestrator`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-core/src/application/orchestrators.rs |
| **Status** | 🟢 |

**Description**: Wraps budget checks and retry delay planning with tracing.

**Signature**
```rust
pub struct CoreOrchestrator { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `InMemoryPersistenceBackend`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-core/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: In-memory `PersistenceBackend` implementation.

**Signature**
```rust
pub struct InMemoryPersistenceBackend { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `InMemoryVectorStore`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-core/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: In-memory cosine-similarity vector store.

**Signature**
```rust
pub struct InMemoryVectorStore { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- Persistence and vector storage in this crate are in-memory only.
- Downstream crates add optional durable backends rather than `or-core` itself.
