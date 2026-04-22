# Design Decisions

## Rust-First Core

The workspace keeps core orchestration logic in Rust crates rather than treating Rust as a hidden implementation detail. That choice shows up in `or-core`, the graph and pipeline runtimes, and the MCP and provider crates.

## Narrower-Than-Raw FFI Surface

`or-bridge` does not attempt to export the entire Rust workspace as a raw ABI. Instead, it exposes explicit JSON-oriented entry points for prompt rendering, state normalization, workspace catalog discovery, and supported crate operations. Higher-level workflows that are callback-heavy or long-lived stay in the host language. This keeps the bridge auditable and reduces cross-language memory and ABI risk.

## Explicit State Everywhere

Execution crates accept owned state and return replacement or patch state. The code consistently avoids shared mutable state as the main execution model.

## In-Memory First Backends

`or-core`, `or-anchor`, and the default path in `or-recall` all use in-memory implementations first. Optional durable storage exists only where the code already implements it, such as SQLite in `or-recall`.

## Transport-Driven MCP

`or-mcp` centers around message types, transports, and request handlers instead of shipping a full standalone HTTP server. That keeps the MCP runtime composable but leaves some hosting responsibilities to callers.

⚠️ Known Gaps & Limitations

- Some design choices clearly point toward future expansion, but this page documents only what exists in the current source.
- No architecture decision record directory was found outside the codebase itself.
