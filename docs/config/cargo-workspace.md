# Cargo Workspace

## File

- Path: `Cargo.toml`
- Edition: `2024`
- Rust version: `1.87.0`
- License: `MIT OR Apache-2.0`
- Workspace version: `0.1.3`

## What It Defines

- The full list of 17 workspace member crates.
- Shared package metadata used by member manifests through `workspace = true`.
- Shared dependency versions for crates such as `tokio`, `serde`, `reqwest`, `schemars`, and observability libraries.

## Why It Matters

The root manifest is the single source of truth for workspace composition and shared dependency pinning. Changes here affect build graph shape and version alignment across all crates.

⚠️ Known Gaps & Limitations
- No package descriptions or repository metadata fields were found in the root manifest.
- Release automation metadata is not defined here.
