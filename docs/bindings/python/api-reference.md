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

## Native Helper Surface

| Name | Signature | Source |
|---|---|---|
| `version` | `() -> str` | `_orchustr.pyi` / `or-bridge` |
| `render_prompt_json` | `(template: str, context_json: str) -> str` | `_orchustr.pyi` / `or-bridge` |
| `normalize_state_json` | `(raw_state: str) -> str` | `_orchustr.pyi` / `or-bridge` |

⚠️ Known Gaps & Limitations
- The API surface documented here matches the current Python package, not the full Rust workspace.
- Several Python classes mirror Rust concepts but are implemented independently in Python.
