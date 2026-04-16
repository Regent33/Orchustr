# Dart Binding Internals

## Package Structure

- `lib/orchustr.dart`: public exports
- `lib/src/native_bridge.dart`: optional `dart:ffi` loader and bridge calls
- `lib/src/prompt.dart`: prompt builder and render fallback
- `lib/src/graph.dart`: graph builder and execution loop
- `lib/src/forge.dart`: tool registry
- `lib/src/mcp.dart`: HTTP JSON-RPC client
- `lib/src/conduit.dart`: HTTP-backed provider helpers
- `tool/build_native.dart`: local native bridge build-and-copy workflow

## Native Bridge Flow

1. Dart checks for a configured library path, `ORCHUSTR_DART_LIBRARY`, or common local library locations.
2. If a library is found, Dart loads `or-bridge` with `dart:ffi`.
3. Prompt rendering and state normalization call the Rust bridge.
4. Returned strings are released through `orchustr_bridge_free_string`.

## Pure Dart Fallbacks

- Prompt rendering falls back to local variable replacement and sanitization.
- Graph execution runs entirely in Dart.
- Forge, MCP, and conduit helpers are implemented entirely in Dart today.

⚠️ Known Gaps & Limitations
- The package does not yet ship an automated native asset pipeline for `pub.dev` consumers.
- Only a narrow Rust helper surface is bridged into Dart today.
