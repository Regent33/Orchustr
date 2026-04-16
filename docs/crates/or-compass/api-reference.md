# or-compass API Reference

This page documents the main public surface re-exported by `or-compass/src/lib.rs` and the key entry points behind those re-exports. 
### `RouteSelection`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-compass/src/domain/entities.rs |
| **Status** | 🟢 |

**Description**: Represents the chosen route name after evaluation.

**Signature**
```rust
pub struct RouteSelection { pub route: String }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `CompassRouterBuilder`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-compass/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: Builder for registering route predicates and a default route.

**Signature**
```rust
pub struct CompassRouterBuilder<T: OrchState> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `CompassRouter`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-compass/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: Concrete runtime that evaluates predicates against state.

**Signature**
```rust
pub struct CompassRouter<T: OrchState> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `CompassOrchestrator`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-compass/src/application/orchestrators.rs |
| **Status** | 🟢 |

**Description**: Thin wrapper around route selection with tracing.

**Signature**
```rust
pub struct CompassOrchestrator;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `CompassError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-compass/src/domain/errors.rs |
| **Status** | 🟢 |

**Description**: Error type for router construction and evaluation failures.

**Signature**
```rust
pub enum CompassError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- Predicates are executable closures, so router instances are not serializable.
