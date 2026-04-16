# deny.toml

## File

- Path: `deny.toml`
- Purpose: define `cargo-deny` security and license policy

## Policy Summary

- Advisories with `vulnerability`, `unmaintained`, `unsound`, and `yanked` status are denied.
- Allowed licenses are `MIT`, `Apache-2.0`, `BSD-2-Clause`, `BSD-3-Clause`, and `ISC`.

## Why It Matters

The CI workflow runs `cargo deny check`, so this file is an active policy gate rather than passive documentation. It is the repository's main machine-readable security and license filter for Rust dependencies.

⚠️ Known Gaps & Limitations
- No exception list or advisory allowlist is defined in the current file.
- The policy covers Rust dependencies only, not Python or npm transitive dependencies.
