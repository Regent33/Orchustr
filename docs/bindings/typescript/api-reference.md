# TypeScript API Reference

## Exported Runtime Surface

| Name | Kind | Source | Notes |
|---|---|---|---|
| `DynState` | class | `src/index.js` | Mutable graph state helper for JS and TS graph authoring. |
| `NodeResult` | class | `src/index.js` | Encodes `advance`, `exit`, `branch`, and `pause` outcomes. |
| `GraphBuilder` | class | `src/index.js` | Builds async execution graphs with explicit entry and exit nodes. |
| `PromptBuilder` | class | `src/index.js` | Validates and renders `{{variable}}` templates. |
| `ConduitProvider` | class | `src/index.js` | Base class for conduit implementations. |
| `ForgeRegistry` | class | `src/index.js` | Registers async tools and imports MCP tools. |
| `NexusClient` | class | `src/index.js` | HTTP MCP helper using `fetch`. |
| `PipelineBuilder`, `RelayBuilder`, `ColonyBuilder`, `SentinelOrchestrator`, and related workflow helpers | classes/functions | `src/workflows.js` | Binding-local helpers for callback-heavy workspace crates. |
| `RustCrateBridge` | class | `src/bridge.js` | Lists binding-visible crates and invokes Rust-backed operations. |

## Type Surface

`index.d.ts` declares the package surface and includes typings for:

- `DynState`
- `NodeResult`
- `GraphBuilder`
- `ConduitProvider`
- `PipelineBuilder`
- `RelayBuilder`
- `ColonyBuilder`
- `installGlobalSubscriber`

## Known Gaps & Limitations

- The declaration file describes the TypeScript package, not a raw dump of every Rust item.
- Native crate calls still require building the optional addon first.
