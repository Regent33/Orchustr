# or-bridge API Reference

This page documents the main public surface re-exported by `or-bridge/src/lib.rs` and the key entry points behind those re-exports.

### `render_prompt_json`

| Property | Value |
|---|---|
| **Kind** | fn |
| **Visibility** | pub |
| **File** | crates/or-bridge/src/application/orchestrators.rs |
| **Status** | Partial |

**Description**: Renders a Beacon template using JSON object context.

**Signature**
```rust
pub fn render_prompt_json(template: &str, context_json: &str) -> Result<String, BridgeError>
```

### `normalize_state_json`

| Property | Value |
|---|---|
| **Kind** | fn |
| **Visibility** | pub |
| **File** | crates/or-bridge/src/application/orchestrators.rs |
| **Status** | Partial |

**Description**: Validates and normalizes a JSON object string for state exchange.

**Signature**
```rust
pub fn normalize_state_json(raw_state: &str) -> Result<String, BridgeError>
```

### `workspace_catalog_json`

| Property | Value |
|---|---|
| **Kind** | fn |
| **Visibility** | pub |
| **File** | crates/or-bridge/src/application/orchestrators.rs |
| **Status** | Partial |

**Description**: Returns a JSON catalog describing which workspace crates are available through the binding layer and whether they are native, mixed, or language-runtime surfaces.

**Signature**
```rust
pub fn workspace_catalog_json() -> Result<String, BridgeError>
```

### `invoke_crate_json`

| Property | Value |
|---|---|
| **Kind** | fn |
| **Visibility** | pub |
| **File** | crates/or-bridge/src/application/orchestrators.rs |
| **Status** | Partial |

**Description**: Invokes a supported workspace crate operation with JSON input and returns JSON output.

**Signature**
```rust
pub fn invoke_crate_json(
    crate_name: &str,
    operation: &str,
    payload_json: &str,
) -> Result<String, BridgeError>
```

### `BridgeError`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-bridge/src/domain/errors.rs |
| **Status** | Partial |

**Description**: Error type for invalid JSON, invalid input, unsupported crates or operations, and invocation failures.

**Signature**
```rust
pub enum BridgeError { ... }
```

### `BridgeState`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-bridge/src/domain/entities.rs |
| **Status** | Partial |

**Description**: Wrapper for JSON payloads crossing the binding boundary.

**Signature**
```rust
pub struct BridgeState { pub json: String }
```

⚠️ Known Gaps & Limitations

- The bridge exposes supported crate operations rather than a raw 1:1 export of every Rust item.
- Some higher-level runtime constructs remain binding-local by design.
