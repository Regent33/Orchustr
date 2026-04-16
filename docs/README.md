<div align="center">
  <img src="../branding/svg_logo/orchustr.svg" alt="Orchustr Logo" width="200" />
  <h1>Orchustr Documentation</h1>
</div>

<p align="center">
  <img src="https://img.shields.io/badge/CI-GitHub_Actions%20configured-blue" alt="CI configured" />
  <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue" alt="License" />
</p>


Orchustr is a Rust-based AI agent orchestration framework organized as a multi-crate Cargo workspace. It implements explicit state passing, provider and tool abstractions, graph and agent runtimes, and cross-language packages for Python, TypeScript, and Dart. The repository is inspired by LangChain- and LangGraph-style patterns, but the codebase is Rust-native and organized around Orchustr's own crate names and boundaries.

## Why Rust

Rust gives the workspace strong ownership semantics for state flow, explicit concurrency boundaries for runtime code, and a natural fit for a layered systems-style architecture. That is especially visible in the graph runtime, retry and budget logic, provider transports, and crate-level API boundaries.

## Multi-Language Support

- **Rust**: full runtime surface across 17 crates.
- **Python**: package `orchustr` with optional PyO3 helpers and pure-Python facades.
- **TypeScript**: package `@orchustr/core` with a pure JS runtime facade and declaration file.
- **Dart**: package `orchustr` with a pure Dart facade and optional `dart:ffi` bridge helpers.

## Complete Crate Map

- `or-core`: shared state, retry, token budgets, and in-memory persistence/vector primitives.
- `or-anchor`: chunking and in-memory retrieval pipeline.
- `or-beacon`: prompt templating and validation.
- `or-bridge`: native prompt/state helper bridge for Python, Node, and Dart.
- `or-checkpoint`: checkpoint pause/resume support.
- `or-colony`: multi-agent coordination and aggregation.
- `or-compass`: predicate-based routing.
- `or-conduit`: LLM provider abstraction and adapters.
- `or-forge`: async tool registry and MCP import path.
- `or-loom`: directed graph execution runtime.
- `or-mcp`: MCP client, server, and transports.
- `or-pipeline`: sequential pipeline runtime.
- `or-prism`: observability bootstrap and OTLP export setup.
- `or-recall`: memory stores.
- `or-relay`: parallel branch execution.
- `or-sentinel`: agent runtime and plan/execute loop.
- `or-sieve`: structured-output and text parsing.

## Build From Source

- Rust: `cargo check --all-features`
- Python: `cd bindings/python && maturin develop`
- TypeScript: `cd bindings/typescript && npm ci && npm run typecheck && npm test`
- Dart: `cd bindings/dart && dart pub get && dart analyze && dart run test/bindings_test.dart`

## Install the Bindings

- Python local install: `pip install -e bindings/python` or `cd bindings/python && maturin develop`
- TypeScript local install: `npm install ./bindings/typescript`
- Dart local use: `cd bindings/dart && dart pub get`

## Compatibility

- Minimum Rust version: `1.87.0` from `rust-toolchain.toml`
- Python package metadata: `>=3.10`; CI validates `3.14.4`
- TypeScript dev tooling: `typescript@6.0.2`; CI validates on Node `20`
- Dart package metadata: `>=3.0.0 <4.0.0`; CI validates on the latest stable Dart SDK

## Bindings Overview

See [Bindings Overview](./bindings/overview.md), [Python README](./bindings/python/README.md), [TypeScript README](./bindings/typescript/README.md), and [Dart README](./bindings/dart/README.md).

## Documentation Index

### Top-level

- [README](./README.md)
- [ARCHITECTURE](./ARCHITECTURE.md)
- [QUICKSTART](./QUICKSTART.md)
- [CONTRIBUTING](./CONTRIBUTING.md)
- [CHANGELOG](./CHANGELOG.md)
- [SECURITY](./SECURITY.md)

### Architecture

- [overview](./architecture/overview.md)
- [crate-dependency-graph](./architecture/crate-dependency-graph.md)
- [data-flow](./architecture/data-flow.md)
- [execution-model](./architecture/execution-model.md)
- [binding-architecture](./architecture/binding-architecture.md)
- [design-decisions](./architecture/design-decisions.md)
- [multi-language-strategy](./architecture/multi-language-strategy.md)

### Crates

- [or-anchor README](./crates/or-anchor/README.md), [api-reference](./crates/or-anchor/api-reference.md), [internals](./crates/or-anchor/internals.md)
- [or-beacon README](./crates/or-beacon/README.md), [api-reference](./crates/or-beacon/api-reference.md), [internals](./crates/or-beacon/internals.md)
- [or-bridge README](./crates/or-bridge/README.md), [api-reference](./crates/or-bridge/api-reference.md), [internals](./crates/or-bridge/internals.md)
- [or-checkpoint README](./crates/or-checkpoint/README.md), [api-reference](./crates/or-checkpoint/api-reference.md), [internals](./crates/or-checkpoint/internals.md)
- [or-colony README](./crates/or-colony/README.md), [api-reference](./crates/or-colony/api-reference.md), [internals](./crates/or-colony/internals.md)
- [or-compass README](./crates/or-compass/README.md), [api-reference](./crates/or-compass/api-reference.md), [internals](./crates/or-compass/internals.md)
- [or-conduit README](./crates/or-conduit/README.md), [api-reference](./crates/or-conduit/api-reference.md), [internals](./crates/or-conduit/internals.md)
- [or-core README](./crates/or-core/README.md), [api-reference](./crates/or-core/api-reference.md), [internals](./crates/or-core/internals.md)
- [or-forge README](./crates/or-forge/README.md), [api-reference](./crates/or-forge/api-reference.md), [internals](./crates/or-forge/internals.md)
- [or-loom README](./crates/or-loom/README.md), [api-reference](./crates/or-loom/api-reference.md), [internals](./crates/or-loom/internals.md)
- [or-mcp README](./crates/or-mcp/README.md), [api-reference](./crates/or-mcp/api-reference.md), [internals](./crates/or-mcp/internals.md)
- [or-pipeline README](./crates/or-pipeline/README.md), [api-reference](./crates/or-pipeline/api-reference.md), [internals](./crates/or-pipeline/internals.md)
- [or-prism README](./crates/or-prism/README.md), [api-reference](./crates/or-prism/api-reference.md), [internals](./crates/or-prism/internals.md)
- [or-recall README](./crates/or-recall/README.md), [api-reference](./crates/or-recall/api-reference.md), [internals](./crates/or-recall/internals.md)
- [or-relay README](./crates/or-relay/README.md), [api-reference](./crates/or-relay/api-reference.md), [internals](./crates/or-relay/internals.md)
- [or-sentinel README](./crates/or-sentinel/README.md), [api-reference](./crates/or-sentinel/api-reference.md), [internals](./crates/or-sentinel/internals.md)
- [or-sieve README](./crates/or-sieve/README.md), [api-reference](./crates/or-sieve/api-reference.md), [internals](./crates/or-sieve/internals.md)

### Bindings

- [bindings overview](./bindings/overview.md)
- Python: [README](./bindings/python/README.md), [installation](./bindings/python/installation.md), [api-reference](./bindings/python/api-reference.md), [examples](./bindings/python/examples.md), [internals](./bindings/python/internals.md)
- TypeScript: [README](./bindings/typescript/README.md), [installation](./bindings/typescript/installation.md), [api-reference](./bindings/typescript/api-reference.md), [examples](./bindings/typescript/examples.md), [internals](./bindings/typescript/internals.md)
- Dart: [README](./bindings/dart/README.md), [installation](./bindings/dart/installation.md), [api-reference](./bindings/dart/api-reference.md), [examples](./bindings/dart/examples.md), [internals](./bindings/dart/internals.md)

### Guides

- [building-your-first-agent](./guides/building-your-first-agent.md)
- [building-your-first-agent-python](./guides/building-your-first-agent-python.md)
- [building-your-first-agent-ts](./guides/building-your-first-agent-ts.md)
- [building-rag-pipelines](./guides/building-rag-pipelines.md)
- [multi-agent-systems](./guides/multi-agent-systems.md)
- [memory-and-context](./guides/memory-and-context.md)
- [tool-integration](./guides/tool-integration.md)
- [llm-providers](./guides/llm-providers.md)
- [error-handling](./guides/error-handling.md)
- [retry-and-resilience](./guides/retry-and-resilience.md)
- [security-guardrails](./guides/security-guardrails.md)
- [observability](./guides/observability.md)
- [performance-tuning](./guides/performance-tuning.md)
- [cross-language-usage](./guides/cross-language-usage.md)

### CI/CD and Configuration

- CI/CD: [overview](./ci-cd/overview.md), [github-actions](./ci-cd/github-actions.md), [release-process](./ci-cd/release-process.md), [publishing-bindings](./ci-cd/publishing-bindings.md)
- Config: [cargo-workspace](./config/cargo-workspace.md), [cargo-lock](./config/cargo-lock.md), [deny-toml](./config/deny-toml.md), [rust-toolchain](./config/rust-toolchain.md), [rustc-info](./config/rustc-info.md), [gitignore](./config/gitignore.md), [environment-variables](./config/environment-variables.md)
- Scripts: [dev-scripts](./scripts/dev-scripts.md)

### Reference

- [crate-index](./reference/crate-index.md)
- [error-codes](./reference/error-codes.md)
- [glossary](./reference/glossary.md)
- [faq](./reference/faq.md)
- [comparison](./langchain-comparison.md)

## License

The Rust workspace uses `MIT OR Apache-2.0` in `Cargo.toml`, and `deny.toml` allows MIT-, Apache-, BSD-, and ISC-family licenses for dependencies.

⚠️ Known Gaps & Limitations
- The CI badge above indicates that a GitHub Actions workflow is configured; it is not a repository-specific pass/fail badge because no public repository URL is encoded in the source tree.
- Binding parity and backend maturity still differ across crates and languages.
