# Python API Reference

## Exported Top-Level Names

| Name | Kind | Backing implementation | Notes |
|---|---|---|---|
| `DynState` | class | `orchustr/state.py` | Mutable graph state helper used by the Python graph surface. |
| `NodeResult` | class | `orchustr/result.py` | Encodes `advance`, `exit`, `branch`, and `pause` outcomes. |
| `GraphBuilder` | class | `orchustr/graph.py` | Builds async execution graphs with explicit entry and exit nodes. |
| `PromptBuilder` | class | `orchustr/prompt.py` | Validates and renders `{{variable}}` templates. |
| `ConduitProvider` | class | `orchustr/conduit.py` | Base class for Python conduit implementations. |
| `ForgeRegistry` | class | `orchustr/forge.py` | Registers async tools and imports MCP tools. |
| `NexusClient` | class | `orchustr/mcp.py` | Async HTTP MCP helper. |
| `PipelineBuilder`, `RelayBuilder`, `ColonyBuilder`, `SentinelOrchestrator`, and related workflow helpers | classes/functions | `orchustr/workflows.py` | Binding-local helpers for callback-heavy workspace crates. |
| `RustCrateBridge` | class | `orchustr/bridge.py` | Lists binding-visible crates and invokes Rust-backed operations. |

## Optional Native Wrapper Surface

When the PyO3 extension is available, `_runtime.py` also re-exports:

- `PyGraphBuilder`
- `PyExecutionGraph`
- `PyDynState`
- `PyNodeResult`
- `PyPromptBuilder`
- `PyPromptTemplate`
- `PyPipelineBuilder`
- `PyPipeline`
- `PyConduitProvider`
- `PyForgeRegistry`
- `PyColonyBuilder`
- `PyRelayBuilder`
- `PyRelayPlan`

## Known Gaps & Limitations

- The Python package intentionally blends Python-native helpers with optional native wrappers instead of projecting every Rust item one-for-one.
- Native bridge availability depends on building the extension successfully in the local environment.
