# FAQ

## Is Orchustr a direct Rust port of LangChain or LangGraph?

No. The repository is clearly inspired by those ecosystems, but the codebase defines its own crate names, contracts, and layering.

## Which language surface is the most complete today?

Rust. The Python and TypeScript packages mirror selected concepts but do not expose the full Rust crate graph directly.

## Is MCP fully production-ready here?

`or-mcp` implements client/server types, request handling, and transports, but it does not yet include a standalone HTTP hosting layer of its own.

## Are the bindings native?

Partly. Python can use a native PyO3 helper module, but most package behavior is still Python code. The TypeScript package is currently a pure JS facade.

## Does the framework ship persistent memory or vector databases by default?

No. The default path is in-memory. SQLite is feature-gated in `or-recall`, and `or-anchor` uses an in-memory vector store.

## Is there a benchmark suite?

No benchmark harness or benchmark report was found in the repository.

⚠️ Known Gaps & Limitations
- This FAQ is based on the current source tree and can go stale if the package surfaces expand.
- No dedicated FAQ source file existed before this generated docs set.
