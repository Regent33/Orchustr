# or-checkpoint

**Status**: 🟢 Complete | **Version**: `0.1.3` | **Deps**: serde, serde_json, thiserror, tracing

Checkpointing crate that serializes state at named gates and restores it through an abstract persistence backend.

## Position in the Workspace

```mermaid
graph LR
  OR_CORE[or-core] --> THIS[or-checkpoint]
  THIS --> CALLERS[Callers]
```

## Implementation Status

| Component | Status | Notes |
|---|---|---|
| Checkpoint records | 🟢 | `CheckpointRecord<T>` stores graph and resume metadata plus state. |
| Gate implementation | 🟢 | `CheckpointGate` saves and restores records through `PersistenceBackend`. |
| Application wrapper | 🟢 | `CheckpointOrchestrator` adds tracing around pause and resume calls. |

## Public Surface

- `CheckpointRecord` (struct): Serializable record stored for a paused execution point.
- `CheckpointGate` (struct): Concrete pause/resume component backed by persistence.
- `CheckpointOrchestrator` (struct): Application helper for pause and resume flows.
- `CheckpointError` (enum): Error type for storage, serialization, and missing-checkpoint failures.

## Dependencies

- Internal crates: or-core
- External crates: serde, serde_json, thiserror, tracing

⚠️ Known Gaps & Limitations
- Durability depends entirely on the supplied persistence backend.
