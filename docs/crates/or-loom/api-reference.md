# or-loom API Reference

This page documents the main public surface re-exported by `or-loom/src/lib.rs`.

### `NodeResult`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-loom/src/domain/entities.rs |
| **Status** | Complete |

**Description**: Represents how a node advances execution: advance, branch, or pause.

### `GraphBuilder`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-loom/src/infra/implementations.rs |
| **Status** | Complete |

**Description**: Builder for graph nodes, edges, entry, and exit configuration, including multi-exit support.

### `ExecutionGraph`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-loom/src/infra/implementations.rs |
| **Status** | Complete |

**Description**: Executable state graph produced by `GraphBuilder::build`.

### `GraphInspection`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-loom/src/inspection.rs |
| **Status** | Complete |

**Description**: Structural description of a built execution graph used for topology comparison and tests.

### `GraphEdgeInspection`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-loom/src/inspection.rs |
| **Status** | Complete |

**Description**: Edge-level inspection record within a `GraphInspection`.

### `NodeRegistry`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-loom/src/registry.rs |
| **Status** | Complete |

**Description**: Feature-gated registry that resolves named node handlers and conditional predicates from `or-schema::GraphSpec` descriptors.

**Availability**
```rust
#[cfg(feature = "serde")]
pub use registry::NodeRegistry;
```

### `LoomOrchestrator`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-loom/src/application/orchestrators.rs |
| **Status** | Complete |

**Description**: Application helper for executing graphs with tracing.

### `LoomError`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-loom/src/domain/errors.rs |
| **Status** | Complete |

**Description**: Error type for graph validation, execution, and schema-resolution issues.

## Known Gaps & Limitations

- `NodeRegistry` is feature-gated behind `serde`.
- The public API is Rust-first; binding layers expose compatible helpers rather than every graph type directly.
