# Multi-Language Strategy

## Why Rust for the Core

Rust is the implementation language for the main runtime because the repository emphasizes explicit state ownership, concurrency control, and a multi-crate architecture with shared contracts. That shows up in state passing, retry policies, graph execution, and transport layers.

## Why Python Exists

The Python package makes the framework easier to script from data and AI workflows. The repository mixes a PyO3 bridge for Rust-backed crate operations with Python-native helpers so users can keep Pythonic control over callback-heavy workflows.

## Why TypeScript Exists

The TypeScript package targets Node-oriented consumers and mirrors the same high-level concepts as the Python package. It ships an ESM-first facade with type declarations, while also supporting an optional native addon build for Rust-backed crate calls.

## Why Dart Exists

The Dart package targets Flutter and Dart VM users who want the same high-level Orchustr concepts without leaving the Dart ecosystem. It follows the same hybrid pattern: Dart ergonomics first, with optional native acceleration where the Rust bridge already owns the behavior.

## `or-bridge` as the Universal Adapter

`or-bridge` is the only Rust crate that directly depends on both PyO3 and NAPI feature paths and also exposes C-ABI functions for Dart. It is the shared native adapter layer for prompt rendering, state normalization, workspace catalog discovery, and Rust-backed crate invocation.

## Performance Characteristics

- **Rust crates**: lowest overhead and full access to workspace internals.
- **Python package**: Rust-backed crate calls can go through PyO3, while workflow composition remains Python-native.
- **TypeScript package**: the default package remains lightweight JS, but native-bridge performance is available when the addon is built locally.
- **Dart package**: prompt/state helpers and Rust-backed crate calls can use the native bridge, while workflow composition remains Dart-native.

## Adding Another Binding

1. Keep the Rust-facing native surface explicit and JSON-oriented inside `or-bridge` or a sibling bridge crate.
2. Map binding-specific data structures onto `DynState`-like JSON objects where possible.
3. Expose higher-level ergonomics in the target language without renaming the underlying concepts beyond language conventions.

## Versioning Strategy

The current repository aligns Rust crates, the Python package, the TypeScript package, and the Dart package at `0.1.3`. No independent cross-language release cadence or publish automation exists in the repository yet.

⚠️ Known Gaps & Limitations

- Binding parity now exists at the crate level, but not as a raw 1:1 export of every Rust item.
- The Python package metadata allows `>=3.10`, while CI currently validates the package on Python `3.14.4`.
- The Dart package currently assumes Dart VM or native Flutter-style environments rather than web-only deployments.
