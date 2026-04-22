# Changelog

All notable changes to Orchustr should be documented in this file.

## [Unreleased]

### Added

- **19 LLM Provider Support** (`or-conduit`):
  - Direct providers: OpenAI, Anthropic, Gemini, Cohere, AI21, Hugging Face, Replicate, DeepSeek, Mistral, xAI, Nvidia, Ollama.
  - Aggregators/routers: OpenRouter, Together AI, Groq, Fireworks AI, AWS Bedrock, Azure OpenAI, Google Vertex AI.
  - `OpenAiCompatConduit` generic struct with named factory constructors for 10 OpenAI-compatible providers.
  - Dedicated conduit types for unique APIs: `GeminiConduit`, `CohereConduit`, `AI21Conduit`, `HuggingFaceConduit`, `ReplicateConduit`, `AzureConduit`, `BedrockConduit`, `VertexConduit`.
- **New error variants** (`or-conduit`): `ConduitError::Timeout`, `ConduitError::AuthenticationFailed`.
- **New error variant** (`or-loom`): `LoomError::NodeExecution` for labeled node failures.
- **Documentation suite** refreshed under `/docs` to cover all 19 providers, plus an exhaustive cross-language `api-matrix.md`.
- **45+ new tests** across 7 crates covering provider construction, key redaction, payload guards, prompt injection, schema validation, graph execution, and routing edge cases.
- **Live Integration Tests**: Added real end-to-end multi-turn memory and ReAct agent tests against live OpenRouter APIs for Rust, Python, TypeScript, and Dart.

### Changed

- **`or-conduit` architecture**: Refactored from flat `adapters.rs`/`implementations.rs` into `infra/adapters/` and `infra/implementations/` directories (feature-based clean architecture).
- **`or-conduit` HTTP layer**: `bearer_headers()` now returns `Result` instead of silently constructing headers with empty keys.
- **`or-prism`**: Updated to `opentelemetry_sdk` 0.31 API (`SdkTracerProvider`, argument-less `with_batch_exporter`).
- **Workspace**: Downgraded `schemars` from `1.x` to `0.8.22` to restore compatibility with `or-sieve` schema module.
- **Crates.io Publishing**: Workspace path dependencies globally upgraded to strictly map versions/descriptions recursively for package publishing.
- **Python Bindings**: Migrated inner conduits from blocking `urllib` to full async natively using `aiohttp`.
- **TypeScript Bindings**: Replaced local-bridge fallback conduits with native asynchronous `fetch` calls.

### Fixed

- **Security**: API keys are redacted in all `Debug` implementations (`[REDACTED]`), preventing accidental logging of secrets.
- **Security**: Added 64KB input size guard to `or-sentinel` `parse_decision` to prevent OOM from oversized LLM responses.
- **Security**: Added 1MB argument payload size guard to `or-forge` `invoke_tool` to prevent injection/OOM.
- **Reliability**: Added SQLite `WAL` mode and `busy_timeout(5000)` to `or-recall` to resolve concurrent access errors.
- **Correctness**: Route names are now trimmed at insertion time in `or-compass`, preventing whitespace-based validation mismatches.
- **Correctness**: `or-sentinel` support module now uses `LoomError::NodeExecution` instead of undefined error variants.
- **Correctness**: Fixed Python `forge.py` closure capture using secure default keyword-args to prevent loop bleeding.
- **Performance**: Upgraded Dart FFI memory allocations using `strlen` instead of slow byte-by-byte copies.
- **Streaming**: Fixed `or-conduit` fallback pipeline breaking stream arrays by passing explicit continuous chunks backwards instead of splitting string whitespace.
- **State Merging**: Replaced `OrchState::merge` silent no-ops with key-level deep merging for `DynState`.

## [0.1.2]

### Added

- Added a cross-language binding bridge for the workspace through `or-bridge`, including `workspace_catalog_json` and `invoke_crate_json` entry points for Python, Node, and Dart.
- Added first-class binding helpers for every `or-tools-*` crate:
  - Python: `RustCrateBridge`, `SearchTools`, `WebTools`, `VectorTools`, `LoaderTools`, `ExecTools`, `FileTools`, `CommsTools`, and `ProductivityTools`
  - TypeScript: `RustCrateBridge` plus matching `*Tools` classes and an optional `npm run build:native` flow
  - Dart: `RustCrateBridge` plus matching `*Tools` classes over `dart:ffi`
- Added binding-local workflow helpers for the crates whose main ergonomics are callback-heavy or host-language-centric, including checkpointing, colony coordination, routing, pipeline execution, recall, relay, sentinel, and sieve-style parsing helpers.

### Changed

- Aligned the Rust workspace, internal path dependency versions, language binding package versions, and docs references on `0.1.2`.
- Expanded the `or-tools-*` documentation set so each tool crate explains its purpose, responsibilities, and boundaries in warmer plain language before diving into API and internals detail.
- Fixed the root workspace manifest version line so `Cargo.toml` remains valid TOML while carrying the new version.
- Clarified the binding story in docs and release notes: all workspace crates are now available in bindings through a hybrid model that combines a native JSON bridge for Rust-backed operations with binding-local helper layers where direct FFI would be a worse fit.

## [0.1.1]

### Added

- Initial multi-crate Rust workspace for orchestration, providers, tools, MCP, retrieval, memory, and bindings.
