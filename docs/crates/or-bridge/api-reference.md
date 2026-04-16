# or-bridge API Reference

This page documents the main public surface re-exported by `or-bridge/src/lib.rs` and the key entry points behind those re-exports. 
### `render_prompt_json`
| Property | Value |
|---|---|
| **Kind** | fn |
| **Visibility** | pub |
| **File** | crates/or-bridge/src/application/orchestrators.rs |
| **Status** | 🟡 |

**Description**: Renders a Beacon template using JSON object context.

**Signature**
```rust
pub fn render_prompt_json(template: &str, context_json: &str) -> Result<String, BridgeError>
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `normalize_state_json`
| Property | Value |
|---|---|
| **Kind** | fn |
| **Visibility** | pub |
| **File** | crates/or-bridge/src/application/orchestrators.rs |
| **Status** | 🟡 |

**Description**: Validates and normalizes a JSON object string for state exchange.

**Signature**
```rust
pub fn normalize_state_json(raw_state: &str) -> Result<String, BridgeError>
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `BridgeError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-bridge/src/domain/errors.rs |
| **Status** | 🟡 |

**Description**: Error type for invalid state JSON and prompt rendering failures.

**Signature**
```rust
pub enum BridgeError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `BridgeState`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-bridge/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Wrapper for JSON payloads crossing the binding boundary.

**Signature**
```rust
pub struct BridgeState { pub json: String }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- The bridge exposes only prompt rendering and state normalization today.
- No unsafe blocks are present, but the Node package does not yet load the NAPI build.
