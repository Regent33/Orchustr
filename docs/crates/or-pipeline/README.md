# or-pipeline

**Status**: 🟢 Complete | **Version**: `0.1.3` | **Deps**: serde, thiserror, tracing

Sequential pipeline builder and runtime for state-transforming nodes that merge their output patches into state.

## Position in the Workspace

```mermaid
graph LR
  OR_CORE[or-core] --> THIS[or-pipeline]
  THIS --> CALLERS[Callers]
```

## Implementation Status

| Component | Status | Notes |
|---|---|---|
| Pipeline builder | 🟢 | Node names are validated for duplicates and blanks before build. |
| Execution runtime | 🟢 | Nodes execute sequentially and merge patches through `OrchState::merge`. |
| Application wrapper | 🟢 | `PipelineOrchestrator` wraps execution with tracing. |

## Public Surface

- `PipelineBuilder` (struct): Builder for ordered async pipeline nodes.
- `Pipeline` (struct): Executable sequential pipeline.
- `PipelineOrchestrator` (struct): Application helper that wraps pipeline execution with tracing.
- `PipelineError` (enum): Error type for malformed pipelines and execution failures.

⚠️ Known Gaps & Limitations
- Pipelines store executable closures and are therefore not serializable.
