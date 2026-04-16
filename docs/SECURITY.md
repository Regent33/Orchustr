# Security

## Current Security Posture in Source

The repository implements the following concrete safeguards:

### Input Validation
- **Prompt sanitization** in `or-beacon`: Null bytes and control characters are stripped from template variables before rendering, preventing injection through template values.
- **Schema validation** in `or-forge` and `or-mcp`: Tool input schemas are validated against JSON Schema definitions before invocation.
- **Input size guards**: `or-sentinel` enforces a 64KB limit on `parse_decision` input; `or-forge` enforces a 1MB limit on tool invocation arguments. Both prevent OOM and denial-of-service from oversized payloads.

### Authentication & Secrets
- **Key redaction**: All provider conduit `Debug` implementations print `[REDACTED]` instead of raw API keys, preventing accidental logging of secrets.
- **Auth guard**: `bearer_headers()` in `or-conduit` returns `Err(AuthenticationFailed)` on empty or missing API keys, failing fast instead of silently sending unauthenticated requests.
- **Environment variables**: All provider keys are read from environment variables with explicit error messages when missing.

### Transport Security
- **Configurable timeouts**: All HTTP provider requests have a configurable timeout (default 30s) with a dedicated `ConduitError::Timeout` variant.
- **Retry with backoff**: Provider requests support configurable retry policies with exponential backoff.
- **Token budget enforcement**: Estimated prompt token counts are checked against budgets before dispatch, preventing accidental overspend.

### Data Integrity
- **SQLite WAL mode**: `or-recall` uses WAL journal mode and `busy_timeout(5000)` to prevent database lock contention under concurrent access.
- **cargo-deny**: `deny.toml` policy enforces license allowlists and bans known-vulnerable dependencies.

## OWASP LLM Top 10 Coverage

| # | Risk | Mitigation |
|---|---|---|
| LLM01 | Prompt Injection | Template variable values are sanitized (null bytes stripped) and not re-expanded. |
| LLM02 | Insecure Output Handling | `or-sieve` validates structured outputs against JSON Schema before downstream use. |
| LLM03 | Training Data Poisoning | Out of scope (framework does not train models). |
| LLM04 | Model Denial of Service | Token budgets and argument size guards prevent oversized requests. |
| LLM05 | Supply Chain Vulnerabilities | `cargo-deny` enforces license and vulnerability audits. |
| LLM06 | Sensitive Information Disclosure | API keys redacted in Debug; environment-only key storage. |
| LLM07 | Insecure Plugin Design | Tool schemas validated before invocation; 1MB argument guard. |
| LLM08 | Excessive Agency | `or-sentinel` enforces step limits in the plan/execute loop. |
| LLM09 | Overreliance | Out of scope (application concern, not framework). |
| LLM10 | Model Theft | Bring-your-own-token design; framework never stores model weights. |

## Reporting Security Issues

A dedicated security reporting address or policy file was **not** found elsewhere in the repository. Until one is added, avoid posting exploit details publicly in issues or documentation. Coordinate with the project owners through a private channel when possible before disclosure.

## Safe Contribution Guidance

- Do not commit secrets or test credentials.
- Keep provider keys in environment variables.
- Treat tool descriptions and inputs as untrusted data.
- Avoid logging prompt bodies or sensitive state into tracing spans.
- All new provider conduits must implement key redaction in Debug.
