# rust-toolchain.toml

## File

- Path: `rust-toolchain.toml`
- Channel: `1.87.0`
- Profile: `minimal`
- Components: `clippy`, `rustfmt`

## Why It Matters

The toolchain file pins the Rust compiler and tooling version used across local development and CI. That keeps edition and lint behavior consistent across workspace members.

⚠️ Known Gaps & Limitations
- No extra components such as `rust-src` or `llvm-tools-preview` are pinned here.
- Coverage tooling is installed separately in CI rather than through this file.
