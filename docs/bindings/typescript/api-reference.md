# TypeScript API Reference

## Exported Runtime Surface

| Name | Kind | Source | Notes |
|---|---|---|---|
| `PromptBuilder` | class | `src/index.js` | Validates and renders `{{variable}}` templates. |
| `GraphBuilder` | class | `src/index.js` | Builds and executes a simple state graph. |
| `ForgeRegistry` | class | `src/index.js` | Registers async tools and imports MCP tools. |
| `NexusClient` | class | `src/index.js` | HTTP MCP helper using `fetch`. |
| `OpenAiConduit` | class | `src/index.js` | Currently a local JS facade rather than a Rust-backed client. |
| `AnthropicConduit` | class | `src/index.js` | Shares the same facade style as `OpenAiConduit`. |

## Type Surface

`index.d.ts` declares the package surface and is used by `tests/typecheck.ts` during `npm run typecheck`.

⚠️ Known Gaps & Limitations
- The declaration file describes the JavaScript facade package, not a direct binding over every Rust crate.
- Some provider behavior is intentionally simplified in the JS package compared to the Rust crate design.
