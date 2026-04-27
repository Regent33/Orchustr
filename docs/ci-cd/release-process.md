# Release Process

No automated release workflow is defined in the repository today. Based on the checked-in files, the current process is best described as **manual and policy-driven**.

## What Exists

- Version numbers are aligned through `workspace.package.version = "0.1.3"` in the root Cargo manifest.
- Python package metadata is versioned at `0.1.3` in `bindings/python/pyproject.toml`.
- TypeScript package metadata is versioned at `0.1.3` in `bindings/typescript/package.json`.
- Dart package metadata is versioned at `0.1.3` in `bindings/dart/pubspec.yaml`.

## Practical Manual Release Checklist

1. Update Rust workspace, Python package, and TypeScript package versions consistently.
2. Run the same validation steps as CI: format, clippy, deny, tests, coverage, Python tests, TypeScript typecheck/tests.
3. Build the Python package with `maturin` and verify the TypeScript package contents.
4. Tag and publish through whatever hosting system the project uses outside this repository.

⚠️ Known Gaps & Limitations
- No release tagging, changelog generation, or package publishing automation exists in the repository.
- No crates.io, PyPI, or npm publication metadata was found beyond package names and versions.
