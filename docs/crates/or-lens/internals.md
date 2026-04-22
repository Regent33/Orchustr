# or-lens Internals

## Layering

- `application/`: dashboard startup orchestration and snapshot assembly
- `domain/`: trace entities, repository contract, and crate-local errors
- `infra/`: Axum HTTP handlers, in-memory repositories, and tracing integration
- compatibility shims: `collector.rs`, `server.rs`, `snapshot.rs`, and `tracing_layer.rs` re-export the refactored internals so the additive public surface remains stable

## Key Files

- `src/application/orchestrators.rs`: starts the server and builds snapshots from collected spans
- `src/domain/entities.rs`: defines `LensSpan`, `TraceSummary`, `ExecutionSnapshot`, and related types
- `src/domain/contracts.rs`: defines the trace repository contract
- `src/domain/errors.rs`: defines `LensError`
- `src/infra/http.rs`: serves the dashboard HTML and JSON endpoints
- `src/infra/repositories.rs`: implements `SpanCollector`
- `src/infra/tracing.rs`: implements `LensLayer`
- `assets/dashboard.html`: self-contained embedded dashboard UI

## Test Shape

- `tests/dashboard.rs` covers collector storage, snapshot ordering, and the HTTP traces endpoint

## Known Gaps & Limitations

- The crate is intentionally local-first and lightweight.
- The current trace ingestion path depends on `LensLayer` rather than a standalone OTLP service.
