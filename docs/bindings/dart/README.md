# Dart Bindings

The Dart package lives under `bindings/dart` and exposes the same high-level concepts as the current Python and TypeScript packages: prompt rendering, graph execution, tool registration, MCP access, HTTP-backed conduit helpers, and optional Rust-backed crate access through `dart:ffi`.

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

## Hybrid Surface

- Use `PromptBuilder`, `GraphBuilder`, `ForgeRegistry`, and `NexusClient` when you want Dart-first ergonomics.
- Use `RustCrateBridge` and the `*Tools` helper classes when you want Dart code to reach the Rust `or-tools-*` crates.

## Public Surface

- `PromptBuilder`
- `GraphBuilder`
- `ForgeRegistry`
- `NexusClient`
- `OpenAiConduit`
- `AnthropicConduit`
- `RustCrateBridge`
- `SearchTools`, `WebTools`, `VectorTools`, `LoaderTools`, `ExecTools`, `FileTools`, `CommsTools`, `ProductivityTools`
- `configureNativeBridge`
- `nativeBridgeAvailable`

⚠️ Known Gaps & Limitations

- The package currently targets Dart VM and native Flutter-style environments rather than browser-only builds.
- Native bridge usage is optional and local-build oriented today.
