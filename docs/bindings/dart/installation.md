# Dart Installation

## Local Repository Use

```bash
cd bindings/dart
dart pub get
```

## Optional Native Bridge

```bash
cd bindings/dart
dart run tool/build_native.dart
```

That builds `or-bridge` with the `dart` Cargo feature and copies the shared library into `bindings/dart/native/`.

## CI Shape

The repository CI runs:

```bash
dart pub get
dart format --output=none --set-exit-if-changed .
dart analyze
dart run tool/build_native.dart
dart run test/bindings_test.dart
```

⚠️ Known Gaps & Limitations

- The repository does not currently publish the Dart package to `pub.dev`.
- Native bridge loading is local-build oriented today.
