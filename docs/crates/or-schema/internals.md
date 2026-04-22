# or-schema Internals

## Layering

- `src/lib.rs`: descriptor type definitions and public re-exports
- `src/loader.rs`: `SchemaError` plus JSON/YAML parsing and serialization helpers

## Key Files

- `src/lib.rs`: defines `GraphSpec`, `NodeSpec`, and `EdgeSpec`
- `src/loader.rs`: implements `from_json`, `from_yaml`, `to_json`, and `to_yaml`
- `tests/roundtrip.rs`: covers JSON and YAML roundtrip behavior

## Design Notes

- `or-schema` intentionally stays runtime-agnostic.
- The crate does not depend on `or-loom`, which keeps descriptor parsing reusable and avoids circular dependencies.
