# Cross-Language Usage

## Naming Conventions

| Concept | Rust | Python | TypeScript | Dart |
|---|---|---|---|---|
| Prompt builder | `PromptBuilder` | `PromptBuilder` | `PromptBuilder` | `PromptBuilder` |
| Dynamic state | `DynState` | `dict` | `Record<string, unknown>` | `Map<String, Object?>` |
| Graph builder | `GraphBuilder` | `GraphBuilder` | `GraphBuilder` | `GraphBuilder` |

## Current Capability Differences

- **Rust**: richest and most direct access to the runtime crates.
- **Python**: mixed PyO3/Python package with Rust-backed tool access and Python workflow helpers.
- **Dart/Flutter**: Dart-first package with optional FFI-backed crate access plus Dart workflow helpers.
- **TypeScript**: JS-first package with optional native bridge loading for Rust-backed crate access.

## When to Choose Which

- Choose **Rust** for the full runtime surface and best control over state, graph, provider, and transport layers.
- Choose **Python** when scripting convenience matters and you still want access to the Rust tool crates.
- Choose **Dart/Flutter** when you want Dart-native ergonomics with optional FFI-backed crate access.
- Choose **TypeScript** when you want a Node-facing package that can stay lightweight by default and opt into native bridge calls locally.

⚠️ Known Gaps & Limitations

- Cross-language feature parity exists at the crate level, but the exact API shape still differs by language.
- Package semantics are intentionally similar, but runtime implementation depth differs significantly by language.
