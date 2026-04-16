<div align="center">
  <img src="branding/svg_logo/orchustr.svg" alt="Orchustr Logo" width="120" />
</div>

<br/>

<div align="center">
  <img src="branding/hero_illustration.png" alt="Orchustr Hero" width="800" />
</div>

<br/>

<div align="center">
  <strong>Orchustr</strong> is a secure, blazing-fast, Rust-based orchestration framework for building autonomous AI agents and massive LLM workflows.
</div>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.87+-blue?style=flat-square&logo=rust" alt="Rust Version" />
  <img src="https://img.shields.io/badge/Python-3.10+-blue?style=flat-square&logo=python" alt="Python Supported" />
  <img src="https://img.shields.io/badge/TypeScript-Node_20+-blue?style=flat-square&logo=typescript" alt="TS Supported" />
  <img src="https://img.shields.io/badge/Dart-3.0+-blue?style=flat-square&logo=dart" alt="Dart Supported" />
  <img src="https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-green?style=flat-square" alt="License" />
</p>

## What is Orchustr?

Orchustr brings systems-level reliability to AI. Unlike dynamic frameworks (like LangChain or LangGraph) where complex agent loops often fail at runtime due to dictionary errors, Orchustr combines **strict compile-time type safety** with massive **async concurrency** to build production-grade agent pipelines. 

With 22 native LLM providers packed in, built-in graph state machines, deterministic multi-agent routing, and cross-language support for Python, TypeScript, and Dart, Orchustr is built to never hang or crash in production.

- 🛡️ **Predictable State:** Pure Rust trait boundaries ensure state transitions are verified at compile time.
- 🚀 **Blazing Fast:** Tokio-based runtime handles parallel agent routing, network requests, and heavy branch fan-outs with near-zero scheduling overhead.
- 🔌 **Universal Compatibility:** Deep integration with the **Model Context Protocol (MCP)** ensures instant plug-and-play capability with thousands of external tools.

## Quick Links

- [📖 Full Documentation](docs/README.md)
- [⚡ Quickstart Guide](docs/QUICKSTART.md)
- [🏗️ Architectural Overview](docs/ARCHITECTURE.md)
- [🛠️ Crate Reference Map](docs/reference/crate-index.md)
- [📊 Orchustr vs LangChain / LangGraph](docs/langchain-comparison.md)

## Contributing

We are building the most robust AI orchestration engine in the world, and **we need your help!** 
Whether you're developing new conduit providers, adding memory tiers, building out the Python/TS/Dart bridges, or improving our documentation, all contributions are warmly welcomed.

1. Check out our [Contributing Guide](docs/CONTRIBUTING.md) to get started.
2. Join the discussion by opening an Issue or submitting a Pull Request on our [GitHub Repo](https://github.com/Cether144/Orchustr).

Let's build secure, production-grade agents together. 🦀🤝🤖
