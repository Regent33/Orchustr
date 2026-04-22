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
  <img src="https://img.shields.io/badge/Rust-1.87+-blue?style=flat-square&logo=rust" alt="Rust Version" />
  <img src="https://img.shields.io/badge/Python-3.10+-blue?style=flat-square&logo=python" alt="Python Supported" />
  <img src="https://img.shields.io/badge/TypeScript-Node_20+-blue?style=flat-square&logo=typescript" alt="TS Supported" />
  <img src="https://img.shields.io/badge/Dart-3.0+-blue?style=flat-square&logo=dart" alt="Dart Supported" />
  <img src="https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-green?style=flat-square" alt="License" />
</p>

## What is Orchustr?

Orchustr brings explicit orchestration boundaries to AI systems. Instead of relying on loosely structured runtime state and ad-hoc loops, it gives you graph builders, agent runtimes, prompt tooling, observability hooks, and cross-language bindings that can be tested offline.

The current workspace includes:

- Pluggable Sentinel loops through `LoopTopology` and `SentinelAgentBuilder`, while preserving the legacy `SentinelAgent::new(...)` path unchanged.
- Serializable graph definitions through `or-schema::GraphSpec` plus `or-loom::NodeRegistry` for compiling named handlers into live graphs.
- A local execution dashboard through `or-lens` and `or-prism`'s optional `lens` feature for in-process trace inspection.
- Cross-language bindings for Python, TypeScript, and Dart, with Python and TypeScript now exposing additive `DynState`, `NodeResult`, and graph builder helpers.

## Why Teams Reach for It

- Predictable state flow: graph and agent boundaries are explicit instead of implicit.
- Async by default: Tokio-backed runtimes handle tool calls, branching, and orchestration efficiently.
- Binding-friendly: Python and TypeScript can author graphs and workflows without needing to mirror every Rust type directly.
- Local visibility: observability is not limited to third-party telemetry backends anymore.

## Quick Links

- [Full Documentation](docs/README.md)
- [Quickstart Guide](docs/QUICKSTART.md)
- [Architectural Overview](docs/ARCHITECTURE.md)
- [Crate Reference Map](docs/reference/crate-index.md)
- [LangChain / LangGraph Comparison](docs/langchain-comparison.md)
- [Example GraphSpec YAML](docs/examples/simple-react-agent.yaml)

## Contributing

We are building a Rust-first orchestration engine for production AI systems, and contributions are welcome across providers, tools, bindings, runtime crates, and docs.

1. Read the [Contributing Guide](docs/CONTRIBUTING.md).
2. Open an issue or pull request on the [GitHub repository](https://github.com/Cether144/Orchustr).

Let's keep making the workspace clearer, safer, and more useful.
