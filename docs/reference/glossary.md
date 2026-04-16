# Glossary

- **OrchState**: Core Rust trait that all strongly typed state objects implement; it defines clone, serde, and merge behavior.
- **DynState**: `HashMap<String, serde_json::Value>` used for dynamic state at graph, binding, and agent boundaries.
- **Conduit**: Orchustr term for an LLM provider client such as OpenAI or Anthropic.
- **Beacon**: Prompt templating subsystem used to validate and render `{{variable}}` placeholders.
- **Pipeline**: Ordered sequence of async nodes that each return a state patch merged into running state.
- **Compass**: Predicate router that selects a named route from a state object.
- **Relay**: Concurrent branch executor that merges multiple state patches deterministically.
- **Loom**: Directed state-graph runtime with explicit entry, exit, branching, and pause behavior.
- **Checkpoint Gate**: Named pause point that serializes state and can later restore it from persistence.
- **Forge Tool**: Schema-described async callable registered in `or-forge`.
- **Nexus**: MCP client/server layer implemented by `or-mcp`.
- **Sentinel**: Agent runtime that composes conduit, forge, and loom into think-act loops.
- **Colony**: Multi-agent coordination layer built around member roles and shared transcripts.
- **Prism**: Observability bootstrap crate for tracing subscriber and OTLP export installation.
- **Bridge**: Narrow FFI layer for Python and Node entry points backed by Rust code.

⚠️ Known Gaps & Limitations
- The glossary only covers terms defined directly in the codebase or used consistently across the repository.
- No separate terminology registry was found outside the source tree.
