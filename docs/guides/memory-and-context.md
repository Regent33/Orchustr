# Memory and Context

Orchustr currently separates short-lived execution state from reusable memory. Dynamic execution state usually lives in `DynState`, while persistent memory-like records live in `or-recall`.

## Current Building Blocks

- `DynState` in `or-core` for per-run or per-step state.
- `RecallEntry` and `RecallStore` in `or-recall` for named memory records.
- `messages` arrays in `or-sentinel` state for conversational context.

## Practical Pattern

- Keep request-local facts in `DynState`.
- Promote reusable facts into `RecallStore`.
- Append tool observations back into `messages` so the agent can see them in the next step.

⚠️ Known Gaps & Limitations
- No automatic memory compaction or summarization system exists in the current code.
- Recall and agent context are connected conceptually, not through a built-in unified memory manager.
