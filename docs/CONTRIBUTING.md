# Contributing

## Development Workflow

1. Read the crate and architecture docs before changing cross-cutting behavior.
2. Keep public API changes explicit and update the matching docs pages in `/docs`.
3. Run the same checks the repository uses in CI where possible:
   - `cargo fmt --all -- --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo deny check`
   - `cargo nextest run --all-features`
   - Python and TypeScript package tests when touching bindings

### Building or-bridge

`or-bridge` is the only workspace crate that does not build with default
features. It refuses to compile without one of `dart`, `node`, or
`python`:

```bash
cargo build -p or-bridge                       # default = dart
cargo build -p or-bridge --features python     # pyo3 extension
cargo build -p or-bridge --features node       # napi-rs add-on
```

Workspace-level `cargo build`/`cargo test` skips this crate
automatically. The `--features python` build needs a Python interpreter
on `PATH`; if `pyo3` cannot find one, set `PYO3_PYTHON=/path/to/python`.

## Code Organization

- `domain/` for contracts, entities, and errors
- `infra/` for concrete implementations and adapters
- `application/` for orchestration entry points and tracing spans

### Adding a new tool surface to the bridge

`or-bridge` exposes Rust crates to Python / TypeScript / Dart through a
JSON dispatch layer in [`crates/or-bridge/src/infra/facades/`](../crates/or-bridge/src/infra/facades/). Each tool surface owns one
short file. To add a new one:

1. Create `crates/or-bridge/src/infra/facades/<crate>.rs` with a
   `pub(crate) fn invoke(operation, payload)` and any private build
   helpers.
2. Add a `mod` line in `facades/mod.rs`.
3. Add a dispatch arm in `facades/mod.rs::invoke`.
4. Add a `CrateBinding` entry in `facades/catalog.rs`.

Each step touches one short file, not the previous monolithic dispatch
file.

## Documentation Expectations

When you add or change behavior, update the matching page in:
- `docs/crates/` for crate-level changes
- `docs/bindings/` for package-level changes
- `docs/architecture/`, `docs/guides/`, or `docs/reference/` for cross-cutting changes

⚠️ Known Gaps & Limitations
- No CODEOWNERS, pull request template, or formal maintainer workflow was found in the repository.
- No conventional commit or release-note automation policy is defined in source.
