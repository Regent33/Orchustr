# or-anchor API Reference

This page documents the main public surface re-exported by `or-anchor/src/lib.rs` and the key entry points behind those re-exports. 
### `AnchorChunk`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-anchor/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Represents a chunked document segment.

**Signature**
```rust
pub struct AnchorChunk { pub document_id: String, pub chunk_id: String, pub text: String }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `RetrievedChunk`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-anchor/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Represents a retrieved chunk with a score.

**Signature**
```rust
pub struct RetrievedChunk { pub chunk: AnchorChunk, pub score: f32 }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `AnchorPipeline`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-anchor/src/infra/implementations.rs |
| **Status** | 🟡 |

**Description**: Concrete indexing and retrieval implementation built on `InMemoryVectorStore`.

**Signature**
```rust
pub struct AnchorPipeline { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `AnchorOrchestrator`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-anchor/src/application/orchestrators.rs |
| **Status** | 🟡 |

**Description**: Application wrapper around indexing and retrieval.

**Signature**
```rust
pub struct AnchorOrchestrator;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `AnchorError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-anchor/src/domain/errors.rs |
| **Status** | 🟡 |

**Description**: Error type for indexing and retrieval failures.

**Signature**
```rust
pub enum AnchorError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- Embeddings are simple hashed vectors, not model-backed embeddings.
- No external vector database integration exists in this crate.
