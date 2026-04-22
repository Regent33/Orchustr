# Orchustr for Dart

Orchustr's Dart package mirrors the current Python and TypeScript binding surface with Dart-first APIs for prompt rendering, graph execution, tool invocation, MCP access, LLM provider calls, and optional Rust-backed crate access.

## What You Get

- `PromptBuilder`
- `GraphBuilder`
- `ForgeRegistry`
- `NexusClient`
- `OpenAiConduit`
- `AnthropicConduit`
- `RustCrateBridge`
- `SearchTools`, `WebTools`, `VectorTools`, `LoaderTools`, `ExecTools`, `FileTools`, `CommsTools`, `ProductivityTools`
- optional native acceleration through `or-bridge`

## Native Bridge

The package works without a native library, but prompt rendering, JSON state normalization, workspace catalog discovery, and Rust-backed crate invocation can use the Rust bridge when it is available.

Build the native bridge from the package root:

```bash
dart run tool/build_native.dart
```

That copies the shared library into `bindings/dart/native/`, where the package can find it automatically during local development.
