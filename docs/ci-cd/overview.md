# CI/CD Overview

The repository defines a single GitHub Actions workflow at `.github/workflows/ci.yml`. That workflow validates Rust formatting, linting, dependency policy, tests, coverage, and both language binding packages.

## Current Strategy

- **Rust job**: runs on Ubuntu, macOS, and Windows with Rust `1.87.0`.
- **Python bindings job**: builds and tests the Python package on Ubuntu with Python `3.14.4`.
- **TypeScript bindings job**: installs, type-checks, and tests the TypeScript package on Ubuntu with Node `20`.

## What Is Not Present

- No publish job for crates.io, PyPI, or npm.
- No scheduled workflow.
- No artifact upload or release packaging stage beyond coverage upload to Codecov.

⚠️ Known Gaps & Limitations
- The workflow validates quality gates but does not automate releases.
- Python and TypeScript jobs are Linux-only in the current workflow.
