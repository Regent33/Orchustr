# Publishing Bindings

## Python

The repository contains `bindings/python/pyproject.toml` configured for `maturin`, which is enough to build the package locally. There is no checked-in workflow that publishes wheels or source distributions to PyPI.

## TypeScript

The repository contains `bindings/typescript/package.json`, `index.d.ts`, and a local `build:native` script. That is enough to install and test the package locally. There is no checked-in workflow that publishes the package to npm.

## Dart

The repository contains `bindings/dart/pubspec.yaml` and `tool/build_native.dart`, which are enough to develop the package locally. There is no checked-in workflow that publishes the package to `pub.dev`.

## What a Manual Publish Would Need

- Version bump in package metadata.
- Local build/test verification.
- Credentials and publish commands executed outside the checked-in workflow.

⚠️ Known Gaps & Limitations

- No automated binding publishing pipeline exists in the repository.
- The TypeScript package now supports local native builds, but broader native packaging and distribution decisions are still open.
