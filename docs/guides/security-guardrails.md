# Security Guardrails

The repository already includes several secure-by-default patterns in code: prompt sanitization in `or-beacon`, schema validation in `or-forge` and `or-mcp`, and explicit provider timeouts in `or-conduit`.

## Current Guardrails

- Prompt template values are sanitized for control characters before render.
- Forge and MCP tool inputs are validated against JSON-schema-like structure checks before invocation.
- Provider HTTP calls use `reqwest` with explicit timeouts.
- Secrets are expected through environment variables instead of checked-in files.

## Practical Advice

- Keep untrusted tool descriptions and input payloads outside shell or SQL execution unless you validate them again at the boundary.
- Avoid storing sensitive values inside `DynState` unless you also control downstream logging and serialization.
- Install `or-prism` carefully and avoid logging prompt or secret contents in spans.

⚠️ Known Gaps & Limitations
- There is no full policy engine or sandboxing layer inside the framework itself.
- The current TypeScript and Python packages do not add extra secret-management abstractions.
