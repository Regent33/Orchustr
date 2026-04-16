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

## Code Organization

- `domain/` for contracts, entities, and errors
- `infra/` for concrete implementations and adapters
- `application/` for orchestration entry points and tracing spans

## Documentation Expectations

When you add or change behavior, update the matching page in:
- `docs/crates/` for crate-level changes
- `docs/bindings/` for package-level changes
- `docs/architecture/`, `docs/guides/`, or `docs/reference/` for cross-cutting changes

⚠️ Known Gaps & Limitations
- No CODEOWNERS, pull request template, or formal maintainer workflow was found in the repository.
- No conventional commit or release-note automation policy is defined in source.
