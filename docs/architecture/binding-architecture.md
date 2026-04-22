# Binding Architecture

The repository exposes three language packages, and they now share the same core pattern even though the ergonomics differ by language. Python uses PyO3 plus Python helpers, TypeScript uses an ESM package plus an optional Node native addon, and Dart uses `dart:ffi` plus Dart helpers.

## Current Binding Stack

- **Rust bridge**: `or-bridge` exports prompt/state helpers plus `workspace_catalog_json` and `invoke_crate_json`.
- **Python package**: `bindings/python` loads `orchustr._orchustr` when built through `maturin` and layers Python workflow helpers on top.
- **TypeScript package**: `bindings/typescript` ships `src/index.js`, `index.d.ts`, and an optional `src/native.js` loader backed by `npm run build:native`.
- **Dart package**: `bindings/dart` ships a Dart API and can load a shared library from `or-bridge` when users build it locally.

## Bridge Role

- `or-bridge/src/python.rs` exposes PyO3 functions and a `_orchustr` module.
- `or-bridge/src/node.rs` exposes NAPI functions used by the optional TypeScript native path.
- `or-bridge/src/dart.rs` exposes C-ABI functions for Dart `dart:ffi` loading.
- `or-bridge/src/infra/facades.rs` maintains the workspace catalog and the JSON invocation facade for Rust-backed crate operations.

## Data Conversion

- Rust `DynState` maps to JSON object strings at the native bridge boundary.
- Python primarily exchanges `dict` objects and falls back to Python workflow helpers when the native module is unavailable.
- TypeScript primarily exchanges plain JavaScript objects and strings, with optional native calls through `RustCrateBridge`.
- Dart primarily exchanges `Map<String, Object?>` values and can use the native bridge for prompt/state helpers plus crate catalog and invocation flows.

## Why the Surface Is Hybrid

- JSON-friendly operations such as search, fetch, execute, load, and vector queries map well onto a native bridge.
- Callback-heavy and long-lived constructs such as graphs, pipelines, MCP clients, and agent loops stay easier to reason about when they are expressed in the host language.
- This keeps the bridge narrower and easier to audit while still making every crate reachable from the bindings.

⚠️ Known Gaps & Limitations

- The binding surface is intentionally not a raw 1:1 export of every Rust item.
- TypeScript native loading is optional and requires a local build artifact.
- The Dart package currently targets Dart VM and native Flutter environments rather than browser-only runtimes because it relies on `dart:io` and optional `dart:ffi`.
