# Binding Architecture

The repository exposes three language packages, and they are not symmetrical yet. Python uses a mixed strategy with an optional native PyO3 module plus pure-Python fallbacks, TypeScript is currently a pure JavaScript facade paired with `.d.ts` declarations, and Dart is a pure Dart facade that can optionally load the Rust bridge with `dart:ffi`.

## Current Binding Stack

- **Rust bridge**: `or-bridge` exports `render_prompt_json` and `normalize_state_json`.
- **Python package**: `bindings/python` can load `orchustr._orchustr` when built through `maturin`, but most package functionality is implemented in Python modules today.
- **TypeScript package**: `bindings/typescript` ships `src/index.js` and `index.d.ts`; it does not currently import a generated native addon.
- **Dart package**: `bindings/dart` ships a pure Dart API and can load a shared library from `or-bridge` when users build it locally.

## Bridge Role

- `or-bridge/src/python.rs` exposes PyO3 functions and a `_orchustr` module.
- `or-bridge/src/node.rs` exposes NAPI functions with the same narrow helper surface.
- `or-bridge/src/dart.rs` exposes C-ABI functions for Dart `dart:ffi` loading.
- The bridge keeps the native API intentionally small: prompt rendering and JSON state normalization only.

## Data Conversion

- Rust `DynState` maps to JSON object strings at the native bridge boundary.
- Python primarily exchanges `dict` objects and falls back to pure-Python rendering if the native module is unavailable.
- TypeScript primarily exchanges plain JavaScript objects and strings in the facade package.
- Dart primarily exchanges `Map<String, Object?>` values and uses the native bridge only for prompt rendering and state normalization when it is available.

⚠️ Known Gaps & Limitations
- The TypeScript package does not yet consume the NAPI exports from `or-bridge`.
- The Python package does not expose the full Rust workspace API through PyO3; most higher-level behavior is implemented separately in Python.
- The Dart package currently targets Dart VM and native Flutter environments rather than browser-only runtimes because it relies on `dart:io` and optional `dart:ffi`.
