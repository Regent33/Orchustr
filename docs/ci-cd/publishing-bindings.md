# Publishing Bindings

## Python

The repository contains `bindings/python/pyproject.toml` configured for `maturin`, which is enough to build the package locally. There is no checked-in workflow that publishes wheels or source distributions to PyPI.

## TypeScript

The repository contains `bindings/typescript/package.json` and `index.d.ts`, which are enough to install and test the package locally. There is no checked-in workflow that publishes the package to npm.

## What a Manual Publish Would Need

- Version bump in package metadata.
- Local build/test verification.
- Credentials and publish commands executed outside the checked-in workflow.

⚠️ Known Gaps & Limitations
- No automated binding publishing pipeline exists in the repository.
- The TypeScript package is currently a pure JS facade and may need native packaging decisions before a broader release strategy.
