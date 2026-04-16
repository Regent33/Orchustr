# Environment Variables

## Variables Found in Source

| Variable | Used by | Purpose |
|---|---|---|
| `OPENAI_API_KEY` | `or-conduit`, Python conduit facade | OpenAI authentication |
| `OPENAI_MODEL` | `or-conduit`, Python conduit facade | OpenAI model selection |
| `ANTHROPIC_API_KEY` | `or-conduit`, Python conduit facade | Anthropic authentication |
| `ANTHROPIC_MODEL` | `or-conduit`, Python conduit facade | Anthropic model selection |
| `CARGO_TARGET_DIR` | `scripts/dev/Invoke-CargoExternalTarget.ps1` | Redirects Cargo build artifacts outside the repo |

## MCP/HTTP Headers Observed

- `Mcp-Session-Id` is used by the MCP HTTP transport.
- `protocolVersion` is exchanged during MCP initialize flows.

⚠️ Known Gaps & Limitations
- No `.env` file or central runtime config loader exists in the repository.
- Additional environment variables may be used indirectly by third-party libraries, but only the variables explicitly referenced in source are listed here.
