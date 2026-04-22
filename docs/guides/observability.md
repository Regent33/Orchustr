# Observability

`or-prism` is the workspace entry point for tracing setup. It installs a tracing subscriber with OTLP export and JSON log formatting, while runtime crates emit `tracing` spans from their application layers.

For local development, `or-prism` can also start the optional `or-lens` dashboard behind the `lens` feature.

## OTLP Export Flow

1. Call `install_global_subscriber(otlp_endpoint)`.
2. Runtime crates emit spans from orchestrators and service boundaries.
3. Spans are exported through OTLP and formatted locally as JSON logs.

## Local Dashboard Flow

1. Enable `or-prism` with feature `lens`.
2. Call `init_with_dashboard(port)`.
3. `or-prism` starts an `or-lens` Axum server and installs a `LensLayer`.
4. Completed spans are mirrored into the in-process `SpanCollector`.
5. Open `http://127.0.0.1:<port>/` to inspect recent traces.

The current dashboard endpoints are:

- `GET /` for the embedded HTML dashboard
- `GET /api/traces` for recent trace summaries
- `GET /api/traces/{id}` for an execution snapshot of a single trace

## CLI Shortcut

If a project has an `orchustr.yaml`, the CLI can validate the config and boot the local dashboard entry point:

```bash
cargo run -p or-cli -- trace path/to/project
```

This currently confirms the project config and dashboard port wiring, then starts and shuts down the local server once the trace bootstrap succeeds.

## Example

```rust
use or_prism::init_with_dashboard;

# async fn boot() -> Result<(), Box<dyn std::error::Error>> {
let handle = init_with_dashboard(7700).await?;
println!("dashboard on {}", handle.port());
# Ok(())
# }
```

## Good Places to Instrument

- Workflow boundaries in application orchestrators
- External calls in `or-conduit` and `or-mcp`
- Tool invocation and step boundaries in `or-forge` and `or-sentinel`

## Known Gaps & Limitations

- Metrics-specific abstractions are not implemented in `or-prism`.
- The current `or-lens` path is in-process and layer-driven; it is not yet a standalone OTLP receiver service.
