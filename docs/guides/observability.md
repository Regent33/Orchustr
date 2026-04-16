# Observability

`or-prism` is the workspace entry point for observability bootstrap. It installs a tracing subscriber with OTLP export and JSON log formatting, while runtime crates emit tracing spans from their application layers.

## Current Flow

1. Call `install_global_subscriber(otlp_endpoint)`.
2. Runtime crates emit `tracing` spans from application orchestrators.
3. Spans are exported through OTLP and formatted locally as JSON logs.

## Good Places to Instrument

- Workflow boundaries in application orchestrators.
- External calls in `or-conduit` and `or-mcp`.
- Tool invocation and step boundaries in `or-forge` and `or-sentinel`.

⚠️ Known Gaps & Limitations
- Metrics-specific abstractions are not implemented in `or-prism`.
- The repository does not include a local telemetry stack or replay UI.
