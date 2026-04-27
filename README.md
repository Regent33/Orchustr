<div align="center">
  <img src="branding/svg_logo/orchustr.svg" alt="Orchustr Logo" width="120" />
</div>

<br/>

<div align="center">
  <img src="branding/hero_illustration.png" alt="Orchustr Hero" width="800" />
</div>

<br/>

<div align="center">
  <strong>Orchustr</strong> is a Rust-first orchestration framework for building reliable AI agents, stateful workflows, and local-first developer tooling.
</div>

<p align="center">
  <a href="https://github.com/Cether144/Orchustr/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Cether144/Orchustr/ci.yml?branch=main&label=Rust%20CI" alt="Rust CI" /></a>
  <a href="https://github.com/Cether144/Orchustr/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Cether144/Orchustr/ci.yml?branch=main&label=Python%20Bindings" alt="Python Bindings CI" /></a>
  <a href="https://github.com/Cether144/Orchustr/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Cether144/Orchustr/ci.yml?branch=main&label=TypeScript%20Bindings" alt="TypeScript Bindings CI" /></a>
  <a href="https://github.com/Cether144/Orchustr/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Cether144/Orchustr/ci.yml?branch=main&label=Dart%20Bindings" alt="Dart Bindings CI" /></a>
  <img src="https://img.shields.io/badge/Rust-1.87+-5a5a5a?style=flat-square&logo=rust" alt="Rust Version" />
  <img src="https://img.shields.io/badge/Python-3.10+-5a5a5a?style=flat-square&logo=python" alt="Python Supported" />
  <img src="https://img.shields.io/badge/TypeScript-Node_20+-5a5a5a?style=flat-square&logo=typescript" alt="TS Supported" />
  <img src="https://img.shields.io/badge/Dart-3.0+-5a5a5a?style=flat-square&logo=dart" alt="Dart Supported" />
  <img src="https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-4b5563?style=flat-square" alt="License" />
</p>

## What is Orchustr?

Orchustr brings explicit orchestration boundaries to AI systems. Instead of relying on loosely structured runtime state and ad-hoc loops, it gives you graph builders, agent runtimes, prompt tooling, observability hooks, and cross-language bindings that can be tested offline.

The current workspace includes:

- Pluggable Sentinel loops through `LoopTopology` and `SentinelAgentBuilder`, while preserving the legacy `SentinelAgent::new(...)` path unchanged.
- Serializable graph definitions through `or-schema::GraphSpec` plus `or-loom::NodeRegistry` for compiling named handlers into live graphs.
- A local execution dashboard through `or-lens` and `or-prism`'s optional `lens` feature for in-process trace inspection.
- MCP auto-discovery through `or-forge::ImportSummary`, `ForgeRegistry::import_all_from_mcp`, and `or-mcp::MultiMcpClient`.
- A new `orchustr` CLI for project scaffolding, graph linting, local trace bootstrapping, and node/topology stubs.
- Cross-language bindings for Python, TypeScript, and Dart, with Python and TypeScript now exposing additive `DynState`, `NodeResult`, and graph builder helpers.

## Why Teams Reach for It

- Predictable state flow: graph and agent boundaries are explicit instead of implicit.
- Async by default: Tokio-backed runtimes handle tool calls, branching, and orchestration efficiently.
- MCP-native integration strategy: multiple MCP servers can be merged without introducing a crate cycle.
- Binding-friendly: Python and TypeScript can author graphs and workflows without needing to mirror every Rust type directly.
- Local visibility: observability is not limited to third-party telemetry backends anymore.
- Faster starts: `orchustr init` and `orchustr lint` reduce the amount of repo knowledge needed to boot a new agent.

## What Runs in Rust vs the Host Language

Each binding is a mix of native Rust code (loaded as a `.dll`/`.so`/`.dylib`
through the `or-bridge` FFI) and pure host-language code that calls those
Rust entry points. Knowing which surface is which keeps debugging and
performance expectations grounded.

| Surface | Python | TypeScript | Dart |
|---------|--------|------------|------|
| **Prompt rendering** (`render_prompt_json`) | Rust via FFI | Rust via FFI | Rust via FFI |
| **State normalization** (`normalize_state_json`) | Rust via FFI | Rust via FFI | Rust via FFI |
| **Workspace catalog** (`workspace_catalog_json`) | Rust via FFI | Rust via FFI | Rust via FFI |
| **HTTP-backed tool crates** (search/web/vector/loaders/exec/file/comms/productivity) — `invoke_crate_json` | Rust via FFI | Rust via FFI | Rust via FFI |
| `GraphBuilder` / `ExecutionGraph` (graph execution loop) | Pure Python (`orchustr.GraphBuilder` in [graph.py](bindings/python/orchustr/graph.py)) | Pure JS (in [src/index.js](bindings/typescript/src/index.js)) | Pure Dart |
| `ConduitProvider` (LLM provider clients: OpenAI / Anthropic / OpenAI-compat) | Pure Python | Pure JS (with real SSE streaming for OpenAI Responses, Anthropic Messages, and OpenAI Chat Completions) | Pure Dart |
| `ForgeRegistry` (tool registration) | Pure Python (with the pyo3 `PyForgeRegistry` available for callback-style use) | Pure JS | Pure Dart |
| `SentinelOrchestrator`, `ColonyOrchestrator`, `PipelineBuilder`, `RelayBuilder`, `CompassRouterBuilder`, `RecallStore`, `CheckpointGate` | Pure Python ([workflows.py](bindings/python/orchustr/workflows.py)) | Pure JS | Pure Dart |
| Bridge metadata / pyo3 stubs (`PyDynState`, `PyNodeResult`, `PyGraphBuilder`, etc.) | Native Rust types accessible via pyo3 — registered handlers are stored and invocable | n/a (no native types exported beyond the JSON bridge) | n/a |

The graph executor and orchestrator loops live in the host language because
node handlers are themselves host-language callables — running them through
Rust would require a per-language async-callback bridge that doesn't yet
exist. Use `or-bridge::invoke_crate_json` (and its language wrappers) when
you need the native HTTP-backed tool implementations.

## Building From Source

`cargo build --workspace` builds every crate except `or-bridge`, which
gates each FFI surface behind a feature flag and refuses to compile
without one. Pick the binding you actually need:

```bash
# Default (Dart FFI)
cargo build -p or-bridge

# Python (pyo3 extension module)
cargo build -p or-bridge --features python

# Node (napi-rs)
cargo build -p or-bridge --features node
```

Workspace-level builds skip `or-bridge` automatically; you only have to
opt in when you are working on the bindings themselves.

## Quick Links

- [Full Documentation](docs/README.md)
- [Quickstart Guide](docs/QUICKSTART.md)
- [Architectural Overview](docs/ARCHITECTURE.md)
- [Crate Reference Map](docs/reference/crate-index.md)
- [CLI Surface](docs/crates/or-cli/README.md)
- [LangChain / LangGraph Comparison](docs/langchain-comparison.md)
- [Example GraphSpec YAML](docs/examples/simple-react-agent.yaml)

## Contributing

We are building a Rust-first orchestration engine for production AI systems, and contributions are welcome across providers, tools, bindings, runtime crates, and docs.

1. Read the [Contributing Guide](docs/CONTRIBUTING.md).
2. Open an issue or pull request on the [GitHub repository](https://github.com/Cether144/Orchustr).

Let's keep making the workspace clearer, safer, and more useful.
