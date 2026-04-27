# or-tools-core

**Status**: Implemented | **Version**: `0.1.3` | **Feature flags**: `(none)` | **Deps**: async-trait, serde, serde_json, thiserror, tokio, tracing

Shared tool abstractions for the Orchustr tool ecosystem. Every `or-tools-*` crate builds on this crate for tool metadata, invocation envelopes, registry contracts, dispatcher behavior, and the canonical `ToolError` type.

## In Plain Language

If the other tool crates are the specialists, this crate is the shared language they all agree to speak. It defines what a tool is, what goes into a tool call, what comes out, how tools describe themselves, and how callers can register or dispatch them without caring which concrete provider sits underneath.

For non-Rust readers, the easiest way to think about `or-tools-core` is as the contract layer. It does not search the web, browse pages, store vectors, or send messages by itself. Instead, it makes sure all of those crates behave consistently enough to plug into the same Orchustr runtime.

## Responsibilities

- Define the common `Tool` and `ToolRegistry` contracts used by every `or-tools-*` crate.
- Define shared metadata and invocation types like `ToolMeta`, `ToolInput`, and `ToolOutput`.
- Provide the canonical `ToolError` type so callers see one consistent error shape.
- Provide a basic in-memory registry and a dispatcher for in-process tool execution.
- Leave provider-specific behavior to downstream crates such as search, web, vector, file, comms, and productivity.

## Position in the Workspace

```mermaid
graph LR
  THIS[or-tools-core] --> DOWN[other or-tools-* crates]
```

## Implementation Status

| Component | Status | Notes |
|---|---|---|
| Tool contracts | Implemented | `Tool` and `ToolRegistry` live in `domain/contracts.rs`. |
| Invocation entities | Implemented | `ToolCapability`, `ToolMeta`, `ToolInput`, and `ToolOutput` are serializable and re-exported. |
| Error model | Implemented | `ToolError` covers lookup, input, transport, upstream, credential, timeout, and serialization failures. |
| Dispatcher | Implemented | `ToolDispatcher` resolves tools through a registry, invokes them, and records elapsed time. |
| Registry | Implemented | `InMemoryToolRegistry` stores `Arc<dyn Tool>` values and rejects duplicate registrations. |
| Unit tests | Implemented | `tests/unit_suite.rs` covers registration, lookup, listing, dispatch, and failure propagation. |

## Public Surface

- `Tool` (trait): contract implemented by every concrete tool crate.
- `ToolRegistry` (trait): async registry abstraction for registration, retrieval, and listing.
- `ToolCapability` (enum): declarative capability tags such as `Network`, `Filesystem`, `Subprocess`, and `Vector`.
- `ToolMeta` (struct): discovery metadata including name, description, declared capabilities, and optional schemas.
- `ToolInput` (struct): invocation envelope containing `tool` and JSON `payload`.
- `ToolOutput` (struct): result envelope containing `tool`, JSON `payload`, and `duration_ms`.
- `ToolError` (enum): canonical error type used across the tool ecosystem.
- `ToolDispatcher` (struct): dispatch adapter that resolves tools through a registry and records timing.
- `InMemoryToolRegistry` (struct): default in-memory registry implementation.

## Dependencies

- Internal crates: `(none)`
- External crates: async-trait, serde, serde_json, thiserror, tokio, tracing

## Known Gaps & Limitations

- The built-in registry implementation in this crate is in-memory only.
- This crate defines abstractions and shared runtime behavior; provider-specific tools live in downstream `or-tools-*` crates.
