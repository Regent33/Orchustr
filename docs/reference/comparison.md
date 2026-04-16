# Comparison

This comparison is limited to what can be grounded in the current Orchustr source tree. It does **not** claim benchmark parity or full ecosystem equivalence with external frameworks.

## Orchustr vs LangChain (Python)

| Area | Orchustr | LangChain (Python reference point) |
|---|---|---|
| Core language | Rust | Python |
| Tooling style | Multi-crate Cargo workspace | Python package ecosystem |
| Tool abstraction | `or-forge` with async JSON-value handlers | Tool abstractions in Python |
| Provider layer | `or-conduit` | Python provider integrations |
| Structured parsing | `or-sieve` with JSON Schema traits | Python output parser ecosystem |
| Current binding completeness | Partial | Native to Python |

## Orchustr vs LangGraph (Python)

| Area | Orchustr | LangGraph (Python reference point) |
|---|---|---|
| Graph runtime | `or-loom` | Python graph runtime |
| State model | `OrchState` / `DynState` | Python-centric typed state patterns |
| Pause/resume | `or-checkpoint` | Checkpointing concepts in Python |
| Agent runtime | `or-sentinel` on top of graph/tool/provider crates | Python graph-agent composition |
| Multi-agent | `or-colony` | Separate Python patterns and integrations |

## Orchustr Rust vs Orchustr Python Binding

| Area | Rust crates | Python package |
|---|---|---|
| Coverage | Full internal runtime surface | Selected concepts only |
| Performance profile | Native Rust | Mixed Python + optional native helpers |
| Provider implementation | Rust `reqwest` adapters | Python stdlib HTTP helpers |
| Agent runtime | Present in `or-sentinel` | Not directly exposed |

## Performance Benchmarks

No benchmark results or benchmark harness were found in the repository.

## When to Use Which Language Interface

- Use **Rust** when you need the full runtime surface and direct control over state, providers, tools, and graphs.
- Use **Python** when scripting convenience matters more than one-to-one access to the Rust workspace.
- Use **TypeScript** when the current JS facade is sufficient for Node-side orchestration helpers.

⚠️ Known Gaps & Limitations
- This page intentionally avoids unsupported claims about external frameworks beyond broad ecosystem context.
- No empirical benchmark data exists in the repository to support speed claims.
