# Bindings Overview

Orchustr ships three language packages: Python under `bindings/python`, TypeScript under `bindings/typescript`, and Dart under `bindings/dart`. The packages use a hybrid model: a native Rust bridge for binding-safe crate operations, plus language-local helpers for callback-heavy workflows and ergonomics that are better owned in Python, JavaScript, or Dart.

- **Python**: PyO3-backed native module plus Python graph, state, prompt, conduit, MCP, tool, and workflow helpers.
- **TypeScript**: ESM package with JavaScript graph, state, prompt, conduit, MCP, and workflow helpers plus an optional native addon path.
- **Dart**: Dart-first package with graph, prompt, conduit, MCP, tool, and workflow helpers plus optional `dart:ffi` access to the native bridge.
- **Rust bridge**: `or-bridge` is the shared native target for prompt rendering, state normalization, workspace catalog discovery, and selected crate invocation.

## Mapping Strategy

- Rust `snake_case` stays `snake_case` in Python.
- TypeScript surfaces use JavaScript-friendly `camelCase` methods, while keeping some snake_case aliases where the helper layer already exposes them.
- Dart surfaces use `camelCase` and JSON-like `Map<String, Object?>` data.
- Dynamic state crosses language boundaries as JSON-like object maps.

## Availability Model

- **Native bridge path**: `RustCrateBridge` can list the workspace catalog and invoke Rust-backed operations for crates such as the `or-tools-*` family. All HTTP-backed providers (search, web, vector, loaders, exec, file, comms, productivity) live on this path.
- **Binding-local path**: graph execution, conduit clients (LLM providers), forge registries, sentinel/colony/pipeline/relay/recall/checkpoint orchestrators, and other callback-heavy workflows are implemented directly in the binding language. This avoids needing an async-callback FFI bridge for host-language node handlers.
- **Mixed path**: prompt rendering, state normalization, observability helpers, and MCP clients have both bridge-backed and binding-local entry points depending on the package.

For the per-class breakdown of which surface is which in each package,
see the workspace [README](../../README.md#what-runs-in-rust-vs-the-host-language).

## Current High-Level Surfaces

- **Python** exports `DynState`, `NodeResult`, `GraphBuilder`, `PromptBuilder`, `NexusClient`, tool helpers, workflow helpers, and optional native `Py*` wrapper classes.
- **TypeScript** exports `DynState`, `NodeResult`, `GraphBuilder`, `PromptBuilder`, `ConduitProvider`, `ForgeRegistry`, `NexusClient`, and workflow helpers from `src/index.js`.
- **Dart** exports graph, prompt, conduit, forge, MCP, tools, and workflow helpers from `lib/orchustr.dart`, plus optional native bridge configuration helpers.

## Known Gaps & Limitations

- The bindings do not expose every Rust type or trait as a literal 1:1 projection.
- TypeScript native loading is optional and requires a local `npm run build:native` step.
- Dart native loading is optional and currently geared toward Dart VM or native Flutter-style environments rather than browser-only builds.
