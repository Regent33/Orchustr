# or-prism API Reference

This page documents the main public surface re-exported by `or-prism/src/lib.rs` and the key entry points behind those re-exports. 
### `PrismConfig`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-prism/src/domain/entities.rs |
| **Status** | 🟡 |

**Description**: Configuration for OTLP endpoint and service name.

**Signature**
```rust
pub struct PrismConfig { pub otlp_endpoint: String, pub service_name: String }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `install_global_subscriber`
| Property | Value |
|---|---|
| **Kind** | fn |
| **Visibility** | pub |
| **File** | crates/or-prism/src/application/orchestrators.rs |
| **Status** | 🟡 |

**Description**: Installs the global tracing subscriber and OTLP exporter.

**Signature**
```rust
pub fn install_global_subscriber(otlp_endpoint: &str) -> Result<(), PrismError>
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `PrismError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-prism/src/domain/errors.rs |
| **Status** | 🟡 |

**Description**: Error type for invalid endpoints, exporter failures, and subscriber installation failures.

**Signature**
```rust
pub enum PrismError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- The crate focuses on tracing bootstrap and does not yet expose metrics-specific orchestration APIs.
- It is unrelated to `or-sieve`; the similar short name reflects observability rather than output parsing.
