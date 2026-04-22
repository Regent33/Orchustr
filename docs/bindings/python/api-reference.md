# Python API Reference

## Exported Top-Level Names

| Name | Kind | Backing implementation | Notes |
|---|---|---|---|
| `PromptBuilder` | class | `orchustr/prompt.py` | Validates `{{variable}}` templates and renders prompts. |
| `GraphBuilder` | class | `orchustr/graph.py` | Builds a simple sequential execution graph. |
| `ForgeRegistry` | class | `orchustr/forge.py` | Registers async tools and can import MCP tools. |
| `NexusClient` | class | `orchustr/mcp.py` | Async HTTP MCP helper. |
| `OpenAiConduit` | class | `orchustr/conduit.py` | Python-side provider wrapper using HTTP requests. |
| `AnthropicConduit` | class | `orchustr/conduit.py` | Python-side provider wrapper using HTTP requests. |
| `RustCrateBridge` | class | `orchustr/bridge.py` | Lists binding-visible crates and invokes Rust-backed operations. |
| `SearchTools` / `WebTools` / `VectorTools` / `LoaderTools` / `ExecTools` / `FileTools` / `CommsTools` / `ProductivityTools` | classes | `orchustr/tools.py` | Friendly wrappers over the Rust `or-tools-*` crates. |
| `CoreOrchestrator`, `CheckpointGate`, `PipelineBuilder`, `RecallStore`, `RelayExecutor`, `SentinelOrchestrator`, and related workflow helpers | classes/functions | `orchustr/workflows.py` | Binding-local helpers for callback-heavy workspace crates. |

## Native Helper Surface

| Name | Signature | Source |
|---|---|---|
| `version` | `() -> str` | `_orchustr.pyi` / `or-bridge` |
| `render_prompt_json` | `(template: str, context_json: str) -> str` | `_orchustr.pyi` / `or-bridge` |
| `normalize_state_json` | `(raw_state: str) -> str` | `_orchustr.pyi` / `or-bridge` |
| `workspace_catalog_json` | `() -> str` | `_orchustr.pyi` / `or-bridge` |
| `invoke_crate_json` | `(crate_name: str, operation: str, payload_json: str) -> str` | `_orchustr.pyi` / `or-bridge` |

⚠️ Known Gaps & Limitations

- The API surface documented here reflects the binding package, not a raw dump of every Rust item.
- Several Python classes intentionally mirror Rust concepts through Python-native ergonomics rather than direct FFI wrappers.
