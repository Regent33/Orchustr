# Bindings Overview

Orchustr ships three language packages: Python under `bindings/python`, TypeScript under `bindings/typescript`, and Dart under `bindings/dart`. All three packages mirror the Rust concepts at a high level, but they are implemented differently today.

- **Python**: mixed model with optional PyO3-native helpers plus pure-Python implementations.
- **TypeScript**: pure JavaScript runtime facade with `.d.ts` declarations and no active native addon loading.
- **Dart**: pure Dart runtime facade with optional `dart:ffi` loading for the native bridge helpers.
- **Rust bridge**: `or-bridge` is the shared native target for prompt rendering and JSON state normalization.

## Mapping Strategy

- Rust `snake_case` stays `snake_case` in Python.
- Rust concepts map to `camelCase` method names or JavaScript idioms in TypeScript only where the package author chose them; the current package is fairly direct in naming.
- Rust concepts map to Dart `camelCase` methods and JSON-like `Map<String, Object?>` state.
- Dynamic state crosses language boundaries as JSON-like object maps.

⚠️ Known Gaps & Limitations
- The current bindings do not expose every Rust crate surface directly.
- The TypeScript package does not yet consume the `or-bridge` NAPI exports.
- The Dart package uses the Rust bridge for a narrow helper surface only; graph, tool, MCP, and conduit behavior is still implemented in Dart.
