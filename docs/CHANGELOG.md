# Changelog

All notable changes to Orchustr should be documented in this file.

## [Unreleased]

### Added

- **22 LLM Provider Support** (`or-conduit`):
  - Direct providers: OpenAI, Anthropic, Gemini, Cohere, AI21, Hugging Face, Replicate, DeepSeek, Mistral, xAI, Nvidia, Ollama.
  - Aggregators/routers: OpenRouter, Together AI, Groq, Fireworks AI, AWS Bedrock, Azure OpenAI, Google Vertex AI.
  - `OpenAiCompatConduit` generic struct with named factory constructors for 10 OpenAI-compatible providers.
  - Dedicated conduit types for unique APIs: `GeminiConduit`, `CohereConduit`, `AI21Conduit`, `HuggingFaceConduit`, `ReplicateConduit`, `AzureConduit`, `BedrockConduit`, `VertexConduit`.
- **New error variants** (`or-conduit`): `ConduitError::Timeout`, `ConduitError::AuthenticationFailed`.
- **New error variant** (`or-loom`): `LoomError::NodeExecution` for labeled node failures.
- **Documentation suite** refreshed under `/docs` to cover all 22 providers.
- **45+ new tests** across 7 crates covering provider construction, key redaction, payload guards, prompt injection, schema validation, graph execution, and routing edge cases.

### Changed

- **`or-conduit` architecture**: Refactored from flat `adapters.rs`/`implementations.rs` into `infra/adapters/` and `infra/implementations/` directories (feature-based clean architecture).
- **`or-conduit` HTTP layer**: `bearer_headers()` now returns `Result` instead of silently constructing headers with empty keys.
- **`or-prism`**: Updated to `opentelemetry_sdk` 0.31 API (`SdkTracerProvider`, argument-less `with_batch_exporter`).
- **Workspace**: Downgraded `schemars` from `1.x` to `0.8.22` to restore compatibility with `or-sieve` schema module.

### Fixed

- **Security**: API keys are redacted in all `Debug` implementations (`[REDACTED]`), preventing accidental logging of secrets.
- **Security**: Added 64KB input size guard to `or-sentinel` `parse_decision` to prevent OOM from oversized LLM responses.
- **Security**: Added 1MB argument payload size guard to `or-forge` `invoke_tool` to prevent injection/OOM.
- **Reliability**: Added SQLite `WAL` mode and `busy_timeout(5000)` to `or-recall` to resolve concurrent access errors.
- **Correctness**: Route names are now trimmed at insertion time in `or-compass`, preventing whitespace-based validation mismatches.
- **Correctness**: `or-sentinel` support module now uses `LoomError::NodeExecution` instead of undefined error variants.

## [0.1.0]

### Added

- Initial multi-crate Rust workspace for orchestration, providers, tools, MCP, retrieval, memory, and bindings.
