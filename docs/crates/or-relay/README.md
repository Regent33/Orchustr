# or-relay

**Status**: 🟢 Complete | **Version**: `0.1.1` | **Deps**: futures, serde, thiserror, tracing

Parallel branch execution crate that runs named state transformations concurrently and merges their patches deterministically.

## Position in the Workspace

```mermaid
graph LR
  OR_CORE[or-core] --> THIS[or-relay]
  THIS --> CALLERS[Callers]
```

## Implementation Status

| Component | Status | Notes |
|---|---|---|
| Branch planning | 🟢 | `RelayBuilder` validates branch names and produces a relay plan. |
| Concurrent execution | 🟢 | `RelayExecutor` runs branches through `FuturesUnordered`. |
| Deterministic merging | 🟢 | Branch results are sorted by branch name before state merge. |

## Public Surface

- `RelayBuilder` (struct): Builder for named parallel branches.
- `RelayPlan` (struct): Executable plan containing branch handlers.
- `RelayExecutor` (struct): Runtime that executes all relay branches concurrently.
- `RelayOrchestrator` (struct): Application helper for relay execution with tracing.
- `RelayError` (enum): Error type for malformed plans and branch execution failures.

⚠️ Known Gaps & Limitations
- Parallelism is intra-process and memory-local; no distributed execution layer exists in this crate.
