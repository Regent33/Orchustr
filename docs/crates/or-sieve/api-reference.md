# or-sieve API Reference

This page documents the main public surface re-exported by `or-sieve/src/lib.rs` and the key entry points behind those re-exports. 
### `JsonSchemaOutput`
| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | crates/or-sieve/src/domain/contracts.rs |
| **Status** | 🟢 |

**Description**: Trait for types that can declare their own JSON Schema.

**Signature**
```rust
pub trait JsonSchemaOutput: serde::de::DeserializeOwned + schemars::JsonSchema
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `StructuredParser`
| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | crates/or-sieve/src/domain/contracts.rs |
| **Status** | 🟢 |

**Description**: Trait implemented by parsers that produce typed structured output.

**Signature**
```rust
pub trait StructuredParser<T: JsonSchemaOutput>: Send + Sync
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `JsonParser`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-sieve/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: Structured parser that validates raw JSON against a schema then deserializes it.

**Signature**
```rust
pub struct JsonParser<T: JsonSchemaOutput> { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `TextParser`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-sieve/src/infra/implementations.rs |
| **Status** | 🟢 |

**Description**: Parser that trims and validates plain text responses.

**Signature**
```rust
pub struct TextParser;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `PlainText`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-sieve/src/domain/entities.rs |
| **Status** | 🟢 |

**Description**: Simple wrapper for plain text output.

**Signature**
```rust
pub struct PlainText { pub text: String }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `SieveOrchestrator`
| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | crates/or-sieve/src/application/orchestrators.rs |
| **Status** | 🟢 |

**Description**: Application helper for structured and plain-text parsing.

**Signature**
```rust
pub struct SieveOrchestrator;
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

### `SieveError`
| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | crates/or-sieve/src/domain/errors.rs |
| **Status** | 🟢 |

**Description**: Error type for invalid JSON, schema violations, deserialization failures, and empty text.

**Signature**
```rust
pub enum SieveError { ... }
```
**Panics**: No explicit panics were found in the exported path during source review.
**Thread Safety**: Follow the type's trait bounds and any synchronization used by its implementation.

⚠️ Known Gaps & Limitations
- Validation is centered on the schema shapes currently produced in this repository rather than every JSON Schema feature.
