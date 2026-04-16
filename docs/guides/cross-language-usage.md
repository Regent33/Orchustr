# Cross-Language Usage

## Naming Conventions

| Concept | Rust | Python | TypeScript |
|---|---|---|---|
| Prompt builder | `PromptBuilder` | `PromptBuilder` | `PromptBuilder` |
| Dynamic state | `DynState` | `dict` | `Record<string, unknown>` |
| Graph builder | `GraphBuilder` | `GraphBuilder` | `GraphBuilder` |

## Current Capability Differences

- **Rust**: richest and most direct access to the runtime crates.
- **Python**: mixed native/pure-Python package with graph, prompt, provider, forge, and MCP facades.
- **TypeScript**: pure JS facade with declarations and tests, but no native bridge loading yet.

## When to Choose Which

- Choose **Rust** for the full runtime surface and best control over state, graph, provider, and transport layers.
- Choose **Python** when scripting convenience matters more than direct access to every Rust crate.
- Choose **TypeScript** when you want a lightweight Node-facing package and current JS ergonomics are enough.

⚠️ Known Gaps & Limitations
- Cross-language feature parity is incomplete today.
- Package semantics are intentionally similar, but runtime implementation depth differs significantly by language.
