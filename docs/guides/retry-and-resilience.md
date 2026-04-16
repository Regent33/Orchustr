# Retry and Resilience

Retry behavior is centralized in `or-core` and consumed primarily by provider and tool execution layers.

## Current Building Blocks

- `RetryPolicy` in `or-core` defines attempts, base delay, max delay, and jitter.
- `BackoffStrategy` calculates retry delays.
- `or-conduit` uses retry policy during HTTP execution.
- `or-sentinel` applies tool retry policy through `invoke_with_retry`.

## Practical Advice

- Use `RetryPolicy::no_retry()` when failures should surface immediately.
- Use `RetryPolicy::default_llm()` when network/provider retries are acceptable.
- Keep max attempts bounded so agent loops do not hide repeated failures.

⚠️ Known Gaps & Limitations
- There is no circuit breaker implementation in the current codebase.
- Retry behavior is local to execution paths that explicitly opt into it.
