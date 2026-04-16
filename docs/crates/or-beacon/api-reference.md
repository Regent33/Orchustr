# or-beacon API Reference

This page documents the main public surface re-exported by `or-beacon/src/lib.rs` and the key entry points behind those re-exports. 
### `PromptTemplate`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-beacon/src/domain/entities.rs |
| **Status** | 🟢 |

**Description**: Compiled prompt template with tracked variable names.

**Signature**
```rust
pub struct PromptTemplate { pub template: String, pub variables: Vec<String> }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `PromptBuilder`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-beacon/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: Builder for validating and constructing prompt templates.

**Signature**
```rust
pub struct PromptBuilder { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `PromptOrchestrator`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-beacon/src/application/orchestrators.rs |
| **Status** | 🟢 |

**Description**: Application helper for build and render operations.

**Signature**
```rust
pub struct PromptOrchestrator;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `BeaconError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-beacon/src/domain/errors.rs |
| **Status** | 🟢 |

**Description**: Error type for malformed templates and invalid context objects.

**Signature**
```rust
pub enum BeaconError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- Only `{{variable}}` placeholder substitution is implemented.
- There is no file-backed prompt registry or template version store in this crate.
