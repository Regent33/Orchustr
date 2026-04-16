# Performance Tuning

## Current Levers in the Codebase

- Keep `DynState` compact so cloning and serialization are cheaper.
- Use `TokenBudget` to cap provider request size before sending requests.
- Use `or-relay` when work can safely happen in parallel and merged deterministically.
- Keep bridge payloads small because `or-bridge` currently exchanges JSON strings rather than zero-copy objects.

## Practical Suggestions

- Prefer typed state with custom `merge` behavior if `DynState` replacement becomes too coarse.
- Avoid large prompt contexts when a retrieval step or memory store can narrow the input first.
- Keep tool schemas focused so validation and invocation stay predictable.

⚠️ Known Gaps & Limitations
- No benchmark harness or performance report exists in the repository.
- The TypeScript and Python packages currently add facade overhead instead of exposing the full Rust runtime directly.
