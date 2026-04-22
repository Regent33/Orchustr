# TypeScript API Reference

## Exported Runtime Surface

| Name | Kind | Source | Notes |
|---|---|---|---|
| `PromptBuilder` | class | `src/index.js` | Validates and renders `{{variable}}` templates. |
| `GraphBuilder` | class | `src/index.js` | Builds and executes a simple state graph. |
| `ForgeRegistry` | class | `src/index.js` | Registers async tools and imports MCP tools. |
| `NexusClient` | class | `src/index.js` | HTTP MCP helper using `fetch`. |
| `OpenAiConduit` | class | `src/index.js` | JavaScript provider helper. |
| `AnthropicConduit` | class | `src/index.js` | JavaScript provider helper. |
| `RustCrateBridge` | class | `src/bridge.js` | Lists binding-visible crates and invokes Rust-backed operations. |
| `SearchTools` / `WebTools` / `VectorTools` / `LoaderTools` / `ExecTools` / `FileTools` / `CommsTools` / `ProductivityTools` | classes | `src/tools.js` | Friendly wrappers over the Rust `or-tools-*` crates. |
| `CoreOrchestrator`, `CheckpointGate`, `PipelineBuilder`, `RecallStore`, `RelayExecutor`, `SentinelOrchestrator`, and related workflow helpers | classes/functions | `src/workflows.js` | Binding-local helpers for callback-heavy workspace crates. |

## Type Surface

`index.d.ts` declares the package surface and is used by `tests/typecheck.ts` during `npm run typecheck`.

⚠️ Known Gaps & Limitations

- The declaration file describes the binding package, not a raw dump of every Rust item.
- Native crate calls require building the optional addon first.
