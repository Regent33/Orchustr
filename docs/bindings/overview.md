# Bindings Overview

Orchustr ships three language packages: Python under `bindings/python`, TypeScript under `bindings/typescript`, and Dart under `bindings/dart`. All three now expose the full workspace through a hybrid model: a native Rust bridge for JSON-friendly crate operations, plus language-local helpers for callback-heavy workflows that are more natural to own in Python, JavaScript, or Dart.

- **Python**: mixed model with PyO3-backed bridge calls plus Python workflow helpers.
- **TypeScript**: ESM package with `.d.ts` declarations, an optional Node native addon loader, and JS workflow helpers.
- **Dart**: Dart-first package with optional `dart:ffi` loading for the native bridge and Dart workflow helpers.
- **Rust bridge**: `or-bridge` is the shared native target for prompt rendering, state normalization, workspace catalog discovery, and crate invocation.

## Mapping Strategy

- Rust `snake_case` stays `snake_case` in Python.
- Rust concepts map to `camelCase` method names or JavaScript idioms in TypeScript only where the package author chose them.
- Rust concepts map to Dart `camelCase` methods and JSON-like `Map<String, Object?>` state.
- Dynamic state crosses language boundaries as JSON-like object maps.

## Availability Model

- **Native bridge path**: `RustCrateBridge` can list the workspace catalog and invoke Rust-backed operations for crates such as `or-tools-search`, `or-tools-web`, `or-tools-vector`, `or-tools-loaders`, `or-tools-exec`, `or-tools-file`, `or-tools-comms`, and `or-tools-productivity`.
- **Binding-local path**: crates such as `or-checkpoint`, `or-colony`, `or-compass`, `or-forge`, `or-loom`, `or-mcp`, `or-pipeline`, `or-recall`, `or-relay`, and `or-sentinel` are exposed through language-local helper classes instead of raw FFI calls.
- **Mixed path**: crates like `or-beacon`, `or-conduit`, `or-prism`, and `or-sieve` are available both through native bridge calls and higher-level binding helpers.

⚠️ Known Gaps & Limitations

- The bindings expose every workspace crate, but not always as a literal 1:1 projection of every Rust type or trait.
- TypeScript native loading is optional and requires a local `npm run build:native` step.
- Dart native loading is optional and currently geared toward Dart VM or native Flutter-style environments rather than browser-only builds.
