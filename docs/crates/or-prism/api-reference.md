# or-prism API Reference

This page documents the main public surface re-exported by `or-prism/src/lib.rs`.

### `PrismConfig`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-prism/src/domain/entities.rs |
| **Status** | Partial |

**Description**: Configuration for OTLP endpoint and service name.

### `install_global_subscriber`

| Property | Value |
|---|---|
| **Kind** | fn |
| **Visibility** | pub |
| **File** | crates/or-prism/src/application/orchestrators.rs |
| **Status** | Partial |

**Description**: Installs the global tracing subscriber and OTLP exporter.

**Signature**
```rust
pub fn install_global_subscriber(otlp_endpoint: &str) -> Result<(), PrismError>
```

### `init_with_dashboard`

| Property | Value |
|---|---|
| **Kind** | async fn |
| **Visibility** | pub |
| **File** | crates/or-prism/src/lens_bridge.rs |
| **Status** | Partial |

**Description**: Feature-gated helper that starts the local `or-lens` dashboard and installs the trace mirroring layer.

**Availability**
```rust
#[cfg(feature = "lens")]
pub async fn init_with_dashboard(port: u16) -> Result<LensHandle, PrismError>
```

### `PrismError`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-prism/src/domain/errors.rs |
| **Status** | Partial |

**Description**: Error type for invalid endpoints, exporter failures, dashboard bootstrap failures, and subscriber installation failures.

## Known Gaps & Limitations

- The crate remains tracing-focused.
- `init_with_dashboard` is only available with feature `lens`.
