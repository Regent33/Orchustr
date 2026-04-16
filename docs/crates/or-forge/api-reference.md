# or-forge API Reference

This page documents the main public surface re-exported by `or-forge/src/lib.rs` and the key entry points behind those re-exports. 
### `ForgeTool`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-forge/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Named tool definition with description and input schema.

**Signature**
```rust
pub struct ForgeTool { pub name: String, pub description: String, pub input_schema: RootSchema }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `ForgeRegistry`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-forge/src/application/orchestrators.rs |
| **Status** | 🟡 |

**Description**: Registry for local async handlers and imported MCP tool proxies.

**Signature**
```rust
pub struct ForgeRegistry { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `ForgeError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-forge/src/domain/errors.rs |
| **Status** | 🟡 |

**Description**: Error type for duplicate tools, invalid arguments, and invocation failures.

**Signature**
```rust
pub enum ForgeError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- Schema validation is intentionally lightweight rather than exhaustive JSON Schema support.
- There is no derive macro or declarative registration layer in the current implementation.
