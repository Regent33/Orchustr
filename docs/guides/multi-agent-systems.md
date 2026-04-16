# Multi-Agent Systems

`or-colony` is the current multi-agent coordination crate. It manages a roster of named members, seeds an initial task message, collects responses, and aggregates the result into state.

## Core Pattern

1. Create a `ColonyOrchestrator`.
2. Add members with names, roles, and implementations of `ColonyAgentTrait`.
3. Put a `task` field into the initial `DynState`.
4. Call `coordinate(initial_state)`.

## What the Result Contains

- Final `state` with recorded member outputs.
- A colony message transcript.
- A summary string.

⚠️ Known Gaps & Limitations
- Execution is sequential today.
- There is no built-in planner, scheduler, or conflict-resolution layer beyond sequential aggregation.
