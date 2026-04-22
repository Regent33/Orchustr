# FAQ

## Is Orchustr a direct Rust port of LangChain or LangGraph?

No. The repository is clearly inspired by those ecosystems, but the codebase defines its own crate names, contracts, and layering.

## Which language surface is the most complete today?

Rust is still the most direct surface because it owns the original runtime types and implementations. Python, TypeScript, and Dart now cover the full workspace at the crate level, but they do so through a hybrid model rather than a raw 1:1 export of every Rust item.

## Is MCP fully production-ready here?

`or-mcp` implements client/server types, request handling, and transports, but it does not yet include a standalone HTTP hosting layer of its own.

## Are the bindings native?

Partly. All three bindings now have a native bridge path through `or-bridge`, but each package still keeps some workflow behavior in the host language where closures, async control flow, and callback-heavy APIs are easier to express.

## Does the framework ship persistent memory or vector databases by default?

No. The default path is in-memory. SQLite is feature-gated in `or-recall`, and `or-anchor` uses an in-memory vector store.

## Is there a benchmark suite?

No benchmark harness or benchmark report was found in the repository.

⚠️ Known Gaps & Limitations

- This FAQ is based on the current source tree and can go stale if the package surfaces expand.
- No dedicated FAQ source file existed before this generated docs set.
