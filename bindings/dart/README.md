# Orchustr for Dart

Orchustr's Dart package mirrors the current Python and TypeScript binding surface with Dart-first APIs for prompt rendering, graph execution, tool invocation, MCP access, and LLM provider calls.

## What You Get

- `PromptBuilder`
- `GraphBuilder`
- `ForgeRegistry`
- `NexusClient`
- `OpenAiConduit`
- `AnthropicConduit`
- optional native acceleration through `or-bridge`

## Native Bridge

The package works without a native library, but prompt rendering and JSON state normalization can use the Rust bridge when it is available.

Build the native bridge from the package root:

```bash
dart run tool/build_native.dart
```

That copies the shared library into `bindings/dart/native/`, where the package can find it automatically during local development.
