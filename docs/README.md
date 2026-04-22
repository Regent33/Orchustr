<div align="center">
  <img src="../branding/svg_logo/orchustr.svg" alt="Orchustr Logo" width="200" />
  <h1>Orchustr Documentation</h1>
</div>

<p align="center">
  <a href="https://github.com/Cether144/Orchustr/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Cether144/Orchustr/ci.yml?branch=main&label=Rust%20CI" alt="Rust CI" /></a>
  <a href="https://github.com/Cether144/Orchustr/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Cether144/Orchustr/ci.yml?branch=main&label=Python%20Bindings" alt="Python Bindings CI" /></a>
  <a href="https://github.com/Cether144/Orchustr/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Cether144/Orchustr/ci.yml?branch=main&label=TypeScript%20Bindings" alt="TypeScript Bindings CI" /></a>
  <a href="https://github.com/Cether144/Orchustr/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Cether144/Orchustr/ci.yml?branch=main&label=Dart%20Bindings" alt="Dart Bindings CI" /></a>
  <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-4b5563" alt="License" />
</p>

Orchustr is a Rust-based AI orchestration workspace organized around focused crates for state, graphs, agents, prompts, tools, observability, and multi-language bindings.

## Workspace Shape

- Rust: 29 workspace crates plus the integration test suite
- Python: package `orchustr` with optional native bridge classes and Python-first workflow helpers
- TypeScript: package `@orchustr/core` with ESM helpers, declaration files, and an optional native addon path
- Dart: package `orchustr` with Dart workflow helpers and optional `dart:ffi` bridge access

## Complete Crate Map

- `or-core`: shared state, retry, token budgets, and in-memory persistence/vector primitives
- `or-anchor`: chunking and in-memory retrieval pipeline
- `or-beacon`: prompt templating and validation
- `or-bridge`: native binding gateway for Python, Node, and Dart
- `or-checkpoint`: checkpoint pause/resume support
- `or-colony`: multi-agent coordination and aggregation
- `or-compass`: predicate-based routing
- `or-conduit`: LLM provider abstraction and adapters
- `or-forge`: async tool registry and MCP import path
- `or-loom`: directed graph execution runtime
- `or-schema`: serializable graph descriptors and JSON/YAML loader helpers
- `or-mcp`: MCP client, server, and transports
- `or-pipeline`: sequential pipeline runtime
- `or-prism`: observability bootstrap and OTLP export setup
- `or-lens`: optional local execution dashboard and in-process trace collection
- `or-cli`: project scaffolding, graph linting, and trace bootstrapping CLI
- `or-recall`: memory stores
- `or-relay`: parallel branch execution
- `or-sentinel`: agent runtime and configurable loop topology support
- `or-sieve`: structured-output and text parsing
- `or-tools-core`: shared tool traits, registry, metadata, dispatcher, and tool errors
- `or-tools-search`: feature-gated web search providers and fallback search orchestration
- `or-tools-web`: browser fetch and scraping backends with URL validation
- `or-tools-vector`: feature-gated vector store clients and RAG-oriented operations
- `or-tools-loaders`: document loaders for text, markdown, JSON, CSV, HTML, and PDF
- `or-tools-exec`: local and remote code execution backends
- `or-tools-file`: local file operations plus JSON, Drive, ArXiv, and financial data backends
- `or-tools-comms`: outbound messaging backends for SMS and chat platforms
- `or-tools-productivity`: productivity clients for email, calendar, tracking, knowledge, and messaging

## Build From Source

- Rust: `cargo check --all-features`
- Python: `cd bindings/python && maturin develop`
- TypeScript: `cd bindings/typescript && npm ci && npm run build:native && npm run typecheck && npm test`
- Dart: `cd bindings/dart && dart pub get && dart run tool/build_native.dart && dart run test/bindings_test.dart`

## Install the Bindings

- Python local install: `pip install -e bindings/python`
- TypeScript local install: `npm install ./bindings/typescript`
- Dart local use: `cd bindings/dart && dart pub get`

## Compatibility

- Minimum Rust version: `1.87.0`
- Python package metadata: `>=3.10`
- TypeScript package target: Node `20+`
- Dart package metadata: `>=3.0.0 <4.0.0`

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
- [or-schema README](./crates/or-schema/README.md), [api-reference](./crates/or-schema/api-reference.md), [internals](./crates/or-schema/internals.md)
- [or-mcp README](./crates/or-mcp/README.md), [api-reference](./crates/or-mcp/api-reference.md), [internals](./crates/or-mcp/internals.md)
- [or-pipeline README](./crates/or-pipeline/README.md), [api-reference](./crates/or-pipeline/api-reference.md), [internals](./crates/or-pipeline/internals.md)
- [or-prism README](./crates/or-prism/README.md), [api-reference](./crates/or-prism/api-reference.md), [internals](./crates/or-prism/internals.md)
- [or-lens README](./crates/or-lens/README.md), [api-reference](./crates/or-lens/api-reference.md), [internals](./crates/or-lens/internals.md)
- [or-cli README](./crates/or-cli/README.md), [api-reference](./crates/or-cli/api-reference.md), [internals](./crates/or-cli/internals.md)
- [or-recall README](./crates/or-recall/README.md), [api-reference](./crates/or-recall/api-reference.md), [internals](./crates/or-recall/internals.md)
- [or-relay README](./crates/or-relay/README.md), [api-reference](./crates/or-relay/api-reference.md), [internals](./crates/or-relay/internals.md)
- [or-sentinel README](./crates/or-sentinel/README.md), [api-reference](./crates/or-sentinel/api-reference.md), [internals](./crates/or-sentinel/internals.md)
- [or-sieve README](./crates/or-sieve/README.md), [api-reference](./crates/or-sieve/api-reference.md), [internals](./crates/or-sieve/internals.md)
- [or-tools-core README](./crates/or-tools-core/README.md), [api-reference](./crates/or-tools-core/api-reference.md), [internals](./crates/or-tools-core/internals.md)
- [or-tools-search README](./crates/or-tools-search/README.md), [api-reference](./crates/or-tools-search/api-reference.md), [internals](./crates/or-tools-search/internals.md)
- [or-tools-web README](./crates/or-tools-web/README.md), [api-reference](./crates/or-tools-web/api-reference.md), [internals](./crates/or-tools-web/internals.md)
- [or-tools-vector README](./crates/or-tools-vector/README.md), [api-reference](./crates/or-tools-vector/api-reference.md), [internals](./crates/or-tools-vector/internals.md)
- [or-tools-loaders README](./crates/or-tools-loaders/README.md), [api-reference](./crates/or-tools-loaders/api-reference.md), [internals](./crates/or-tools-loaders/internals.md)
- [or-tools-exec README](./crates/or-tools-exec/README.md), [api-reference](./crates/or-tools-exec/api-reference.md), [internals](./crates/or-tools-exec/internals.md)
- [or-tools-file README](./crates/or-tools-file/README.md), [api-reference](./crates/or-tools-file/api-reference.md), [internals](./crates/or-tools-file/internals.md)
- [or-tools-comms README](./crates/or-tools-comms/README.md), [api-reference](./crates/or-tools-comms/api-reference.md), [internals](./crates/or-tools-comms/internals.md)
- [or-tools-productivity README](./crates/or-tools-productivity/README.md), [api-reference](./crates/or-tools-productivity/api-reference.md), [internals](./crates/or-tools-productivity/internals.md)

### Bindings

- [bindings overview](./bindings/overview.md)
- Python: [README](./bindings/python/README.md), [installation](./bindings/python/installation.md), [api-reference](./bindings/python/api-reference.md), [examples](./bindings/python/examples.md), [internals](./bindings/python/internals.md)
- TypeScript: [README](./bindings/typescript/README.md), [installation](./bindings/typescript/installation.md), [api-reference](./bindings/typescript/api-reference.md), [examples](./bindings/typescript/examples.md), [internals](./bindings/typescript/internals.md)
- Dart: [README](./bindings/dart/README.md), [installation](./bindings/dart/installation.md), [api-reference](./bindings/dart/api-reference.md), [examples](./bindings/dart/examples.md), [internals](./bindings/dart/internals.md)

### Guides

- [building-your-first-agent](./guides/building-your-first-agent.md)
- [building-your-first-agent-python](./guides/building-your-first-agent-python.md)
- [building-your-first-agent-ts](./guides/building-your-first-agent-ts.md)
- [building-your-first-agent-dart](./guides/building-your-first-agent-dart.md)
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

### Reference

- [crate-index](./reference/crate-index.md)
- [error-codes](./reference/error-codes.md)
- [glossary](./reference/glossary.md)
- [faq](./reference/faq.md)
- [comparison](./langchain-comparison.md)
- [simple-react-agent example](./examples/simple-react-agent.yaml)

## Known Gaps & Limitations

- The badges above all reflect the shared repository workflow because Rust, Python, TypeScript, and Dart are currently validated by one `ci.yml`.
- Binding parity and backend maturity still differ across crates and languages.
