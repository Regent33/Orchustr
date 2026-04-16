# Multi-Language Strategy

## Why Rust for the Core

Rust is the implementation language for the main runtime because the repository emphasizes explicit state ownership, concurrency control, and a multi-crate architecture with shared contracts. That shows up in state passing, retry policies, graph execution, and transport layers.

## Why Python Exists

The Python package makes the framework easier to script from data and AI workflows. The repository currently mixes optional PyO3 helpers with pure-Python implementations so users can work with the package even when a native build is not present.

## Why TypeScript Exists

The TypeScript package targets Node-oriented consumers and mirrors the same high-level concepts as the Python package. Right now it ships a pure JS facade with type declarations, which keeps adoption simple but leaves native acceleration for later.

## Why Dart Exists

The Dart package targets Flutter and Dart VM users who want the same high-level Orchustr concepts without leaving the Dart ecosystem. It follows the same hybrid pattern as Python: pure language ergonomics first, with optional native acceleration where the Rust bridge already owns the behavior.

## `or-bridge` as the Universal Adapter

`or-bridge` is the only Rust crate that directly depends on both PyO3 and NAPI feature paths. It is therefore the intended long-term native adapter layer even though only a narrow helper API is exposed today.

## Performance Characteristics

- **Rust crates**: lowest overhead and full access to workspace internals.
- **Python package**: some operations can use the native bridge, but much of the package is currently pure Python logic.
- **TypeScript package**: currently behaves like a pure JS wrapper layer, so native-bridge performance benefits are not yet realized.
- **Dart package**: graph, tool, MCP, and conduit behavior currently run in Dart, while prompt rendering and state normalization can use the native bridge when present.

## Adding Another Binding

1. Keep the Rust-facing native surface narrow inside `or-bridge` or a sibling bridge crate.
2. Map binding-specific data structures onto `DynState`-like JSON objects where possible.
3. Expose higher-level ergonomics in the target language without renaming the underlying concepts beyond language conventions.

## Versioning Strategy

The current repository aligns Rust crates, the Python package, the TypeScript package, and the Dart package at `0.1.0`. No independent cross-language release cadence or publish automation exists in the repository yet.

⚠️ Known Gaps & Limitations
- The TypeScript package does not currently use `or-bridge` at runtime.
- The Python package metadata allows `>=3.10`, while CI currently validates the package on Python `3.14.4`.
- The Dart package currently assumes Dart VM or native Flutter-style environments rather than web-only deployments.
