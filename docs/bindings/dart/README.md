# Dart Bindings

The Dart package lives under `bindings/dart` and exposes the same high-level concepts as the current Python and TypeScript packages: prompt rendering, graph execution, tool registration, MCP access, and HTTP-backed conduit helpers.

## Binding Technology

- Pure Dart runtime surface for most behavior
- Optional `dart:ffi` bridge loading for `or-bridge`
- No external Dart package dependencies are required by the current implementation

## Quickstart

```bash
cd bindings/dart
dart pub get
dart run tool/build_native.dart
dart run test/bindings_test.dart
```

```dart
import "package:orchustr/orchustr.dart";

Future<void> main() async {
  final template = PromptBuilder().template("Hello {{name}}").build();
  print(template.render(<String, Object?>{"name": "Ralph"}));
}
```

## Public Surface

- `PromptBuilder`
- `GraphBuilder`
- `ForgeRegistry`
- `NexusClient`
- `OpenAiConduit`
- `AnthropicConduit`
- `configureNativeBridge`
- `nativeBridgeAvailable`

⚠️ Known Gaps & Limitations
- The package currently targets Dart VM and native Flutter-style environments rather than browser-only builds.
- Only prompt rendering and state normalization use the native Rust bridge today.
