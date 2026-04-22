# Data Flow

This page describes how data moves through the current runtime paths that exist in the source tree.

## Rust Request Lifecycle

```mermaid
sequenceDiagram
  participant Caller
  participant Pipeline as or-pipeline/or-loom
  participant Conduit as or-conduit
  participant Forge as or-forge
  participant Sentinel as or-sentinel
  Caller->>Pipeline: provide DynState or typed OrchState
  Pipeline->>Pipeline: execute nodes / merge patches
  Pipeline-->>Caller: updated state
  Caller->>Sentinel: run(initial_state, config)
  Sentinel->>Conduit: complete_messages(messages)
  Conduit-->>Sentinel: CompletionResponse
  Sentinel->>Forge: invoke(tool_name, args)
  Forge-->>Sentinel: tool result JSON
  Sentinel->>Sentinel: append MessageRole::Tool observation
  Sentinel-->>Caller: StepOutcome::FinalAnswer or StepLimitReached
```

## Binding Entry Paths

```mermaid
sequenceDiagram
  participant Py as Python caller
  participant Ts as TypeScript caller
  participant Dart as Dart caller
  participant Bridge as or-bridge
  participant Beacon as or-beacon
  participant Tools as Rust-backed crates
  Py->>Bridge: invoke_crate_json(...) or render_prompt_json(...)
  Bridge->>Beacon: PromptBuilder::build + render
  Beacon-->>Bridge: rendered prompt
  Bridge-->>Py: JSON or rendered string
  Ts->>Bridge: optional native invoke via RustCrateBridge
  Dart->>Bridge: optional FFI invoke via RustCrateBridge
  Bridge->>Tools: dispatch supported crate operation
  Tools-->>Bridge: JSON result
  Bridge-->>Ts: JSON
  Bridge-->>Dart: JSON
```

## Data Shapes

- **State**: `DynState` is a JSON-like object map used widely at orchestration and binding boundaries.
- **Messages**: `or-conduit` and `or-sentinel` pass `Vec<CompletionMessage>` with structured content parts.
- **Tool calls**: `or-forge` and `or-mcp` exchange JSON values plus JSON Schema-backed metadata.

⚠️ Known Gaps & Limitations

- Native event streaming is not implemented for provider adapters; `stream_text` falls back to locally chunked final text.
- Some binding flows deliberately stay in the host language rather than going through FFI when the API is callback-heavy or long-lived.
