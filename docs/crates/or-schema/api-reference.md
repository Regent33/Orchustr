# or-schema API Reference

This page documents the main public surface re-exported by `or-schema/src/lib.rs`.

### `GraphSpec`
- **Kind**: struct
- **File**: `crates/or-schema/src/lib.rs`
- **Description**: Serializable graph definition with nodes, edges, entry, and exits.

### `NodeSpec`
- **Kind**: struct
- **File**: `crates/or-schema/src/lib.rs`
- **Description**: Serializable node descriptor with an identifier, registered handler name, and metadata payload.

### `EdgeSpec`
- **Kind**: struct
- **File**: `crates/or-schema/src/lib.rs`
- **Description**: Serializable edge descriptor with an optional condition name.

### `SchemaError`
- **Kind**: enum
- **File**: `crates/or-schema/src/loader.rs`
- **Description**: Error type for JSON parsing, YAML parsing, and missing YAML feature support.

## Known Gaps & Limitations

- Runtime compilation is handled by downstream crates such as `or-loom`.
- YAML helpers are feature-gated.
