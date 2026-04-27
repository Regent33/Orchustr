# Changelog

All notable changes to Orchustr should be documented in this file.

## [Unreleased]

## [0.1.3]

### Added

- **Audit fixes batch** ([docs/AUDIT_2026-04-26.md](AUDIT_2026-04-26.md)): 25+ findings landed across the workspace, including:
  - **`LoomError::Paused` carries merged state** so callers can resume without round-tripping through a `PersistenceBackend`. `LoomError` derives `PartialEq` only (no `Eq`) because the `state` field holds `serde_json::Value`.
  - **Typed step context for sentinel** (`SentinelStepContext` in a `tokio::task_local!`) replacing the five `__sentinel_*` keys that were previously stuffed into `DynState`. The user-facing `DynState` now stays clean by construction.
  - **`LoopTopology::bind`** is a real trait method with a no-op default; the previous `Any`-downcast dispatch is gone. Custom topologies that attach handlers in `build()` work unchanged; built-in topologies override `bind` to wire `provider` and `registry`.
  - **`SentinelError::Loom` and `::Core`** wrap the underlying typed error via `#[from]`. `CliError::Lens` wraps `or_lens::LensError` the same way. Pattern-match on the inner error to recover full context.
  - **Bounded `or-lens::SpanCollector`** with per-trace and total-trace caps (defaults 10 000 / 1 024) plus FIFO eviction. `SpanCollector::with_capacity` for tuning.
  - **Bounded top-k in `InMemoryVectorStore::query`** via `BinaryHeap<Reverse<_>>` — O(N log limit) instead of O(N log N), with deterministic tie-breaks and `limit == 0` short-circuit.
  - **Retry classification in sentinel** — only `ForgeError::Invocation` is retried; terminal forge errors short-circuit immediately. Each attempt logs via `tracing::debug!`.
- **Parallel colony fan-out** (`or-colony`): new `ColonyOrchestrator::coordinate_parallel` runs every member concurrently with `futures::try_join_all`, merges replies in deterministic roster order. The existing `coordinate` keeps the cascading hand-off semantics.
- **Real SSE streaming in TypeScript conduits**: `OpenAiConduit`, `AnthropicConduit`, and `OpenAiCompatConduit` (covering OpenRouter, Groq, Together, Fireworks, DeepSeek, Mistral, xAI, NVIDIA, Ollama) now drive `stream: true` requests with a generic SSE parser. Falls back to non-streaming when `response.body` is absent.
- **`or-bridge` decomposed** into per-crate facade modules under `crates/or-bridge/src/infra/facades/` (one short file per tool surface, plus shared helpers/catalog). Adding a new tool surface now touches one short file instead of an 1100-line monolith.
- **Workspace README** ships a "What Runs in Rust vs the Host Language" table that names exactly which classes are FFI-backed vs language-native per binding.

### Changed

- **Python FFI handlers are now retained.** `PyForgeRegistry`, `PyGraphBuilder`/`PyExecutionGraph`, and `PyConduitProvider` keep the Python callables they receive. `PyForgeRegistry.invoke` and `PyConduitProvider.complete_messages` actually call them; `PyExecutionGraph.get_handler` returns the registered callable. (Previously they silently discarded the handler argument.)
- **`orchustr run` shells out** to the language toolchain declared in `orchustr.yaml` (`cargo run` / `python` / `npm start` or `npx tsx` / `dart run`) with inherited stdio and `kill_on_drop`. Previously it was a no-op.
- **`orchustr trace`** keeps the dashboard alive until Ctrl-C and prints the bound port. Previously it spun up the dashboard then immediately shut it down.
- **`orchustr` CLI errors** render via `Display` (`orchustr: <message>`) instead of `Debug`-printing the struct.
- **`SentinelAgentBuilder::build()`** dispatches through `LoopTopology::bind` instead of `Any` downcasting. User-defined topologies finally work end-to-end.
- **`or-bridge::block_on`** branches on `RuntimeFlavor`: `block_in_place` on multi-thread runtimes, typed `BridgeError` (instead of panicking) on current-thread runtimes — covers `pyo3-asyncio` single-thread setups.
- **`or-loom`**: removed redundant handler-local `state.clone()` calls in built-in sentinel topologies (one fewer `DynState` clone per node firing). Executor-side clone remains, tracked as accepted technical debt.

### Fixed

- **`ShellExecutor` security gate**: refuses to run unless `ORCHUSTR_ALLOW_UNSANDBOXED_SHELL=1`. Returns `ExecError::ExecutorNotFound` with guidance toward `E2BExecutor` / `DaytonaExecutor` / `BearlyExecutor`.
- **`ShellExecutor` child reaping**: `kill_on_drop(true)` on the spawned `Command` so a fired timeout reaps the process instead of orphaning it.
- **`into_raw_string` (Dart bridge)** returns `Result<*mut c_char, BridgeError>`, surfacing interior NUL bytes as a typed error rather than the previous null-on-failure that was indistinguishable from "empty output".
- **`or-bridge` global runtime documented** as intentional — `OnceLock<Runtime>` lives for process lifetime; pyo3 / napi-rs / dart:ffi all rely on the OS for teardown at exit.

### Documentation

- New **"Accepted Technical Debt"** section in [docs/AUDIT_2026-04-26.md](AUDIT_2026-04-26.md) documenting why audit items #1 (orchestrator shim collapse), #11 (executor-side clone), and #23 (parallel binding executors) are deferred to `0.2.0`.
- Updated `docs/SECURITY.md` (shell sandbox gate, OWASP LLM07/LLM08 rows), `docs/CONTRIBUTING.md` (or-bridge build steps, "adding a new tool surface" checklist), `docs/QUICKSTART.md` (per-language run matrix), `docs/ARCHITECTURE.md` (cross-cutting layer list), and `docs/reference/error-codes.md` (schema notes for `LoomError::Paused`, typed `SentinelError` chains, `CliError::Lens`, shell `ExecError::ExecutorNotFound`).
- Per-crate READMEs refreshed for `or-cli`, `or-loom`, `or-sentinel`, `or-colony`, `or-bridge`, `or-lens`, `or-tools-exec`, and `or-core`.
- Per-binding READMEs (Python / TypeScript / Dart) refreshed: Python's two-`GraphBuilder` clarification, TypeScript's SSE streaming examples, Dart's FFI allocation contract.
- `cargo clippy --workspace --all-targets --all-features` is now warning-clean.

## [0.1.2-historical]

(originally `[Unreleased]` — content preserved verbatim below)

### Added

- **Pluggable sentinel topologies** (`or-sentinel`): Added `LoopTopology`, `SentinelAgentBuilder`, and built-in `ReActTopology`, `PlanExecuteTopology`, and `ReflectionTopology` for additive loop customization without changing `SentinelAgent::new`.
- **Graph late binding and inspection** (`or-loom`): Added placeholder-node support, handler rebinding, `ExecutionGraph::inspect()`, `GraphInspection`, and `LoomError::UnboundNode` so higher-level crates can compose graph structure before wiring runtime handlers.
- **Binding parity helpers** (`or-bridge`, Python, TypeScript): Added additive graph/state/result/conduit/workflow builder surfaces so Python and TypeScript can assemble offline Orchustr graphs with `DynState`, `NodeResult`, `GraphBuilder`, `PipelineBuilder`, `RelayBuilder`, and `ColonyBuilder` without requiring live provider keys.
- **Serializable graph descriptors** (`or-schema`, `or-loom`): Added the new `or-schema` crate with JSON/YAML `GraphSpec` loading plus `or-loom::NodeRegistry` for resolving named handlers and conditional edges into live execution graphs.
- **Local dashboard runtime** (`or-lens`, `or-prism`): Added the new `or-lens` crate with an embedded HTML dashboard, in-memory span collector, execution snapshots, and additive `or-prism::init_with_dashboard` support behind the `lens` feature.
- **MCP ecosystem bridge** (`or-mcp`, `or-forge`): Added `MultiMcpClient`, `MultiMcpSession`, `McpServerConfig`, curated `known_servers::*` presets, `ForgeRegistry::import_all_from_mcp`, `ForgeRegistry::import_all_from_multi_mcp`, and `ImportSummary` for additive multi-server MCP discovery.
- **CLI scaffolding crate** (`or-cli`): Added the new `orchustr` binary with `init`, `lint`, `run`, `trace`, `new node`, and `new topology` flows plus embedded Rust, Python, TypeScript, and Dart templates.
- **Unified error-code registry and CI expansion** (`docs`, `.github`): Added stable `ORC-*` error code documentation, repository workflow badges, CLI graph linting in CI, explicit binding test steps, and an `or-lens` dashboard smoke step.
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
- **`or-lens` dashboard UI**: Refined the embedded HTML dashboard toward a denser grayscale presentation with a stronger execution map, timeline, and node inspector layout while keeping the API contract unchanged.
- **Docs**: Synced the crate index, API matrix, observability guide, and per-crate docs to the additive Phase 5 and Phase 6 surfaces.

### Fixed

- **Observability** (`or-prism`): OTLP HTTP exporter installation now provides a compatible HTTP client explicitly, restoring `install_global_subscriber` and `lens` feature tests under the current OpenTelemetry dependency set.
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
