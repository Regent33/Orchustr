# or-pipeline API Reference

This page documents the main public surface re-exported by `or-pipeline/src/lib.rs` and the key entry points behind those re-exports. 
### `PipelineBuilder`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-pipeline/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: Builder for ordered async pipeline nodes.

**Signature**
```rust
pub struct PipelineBuilder<T: OrchState> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `Pipeline`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-pipeline/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: Executable sequential pipeline.

**Signature**
```rust
pub struct Pipeline<T: OrchState> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `PipelineOrchestrator`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-pipeline/src/application/orchestrators.rs |
| **Status** | 🟢 |

**Description**: Application helper that wraps pipeline execution with tracing.

**Signature**
```rust
pub struct PipelineOrchestrator;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `PipelineError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-pipeline/src/domain/errors.rs |
| **Status** | 🟢 |

**Description**: Error type for malformed pipelines and execution failures.

**Signature**
```rust
pub enum PipelineError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- Pipelines store executable closures and are therefore not serializable.
