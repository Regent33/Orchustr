# Glossary

- **Anchor**: Retrieval-oriented system for indexing and chunking embeddings to supply RAG (Retrieval-Augmented Generation) memory.
- **Beacon**: Prompt templating subsystem used to validate and render `{{variable}}` placeholders.
- **Bridge**: Narrow FFI layer for Python and Node entry points backed by Rust code.
- **Checkpoint Gate**: Named pause point that serializes state and can later restore it from persistence.
- **Colony**: Multi-agent coordination layer built around member roles and shared transcripts.
- **Compass**: Predicate router that selects a named route from a state object.
- **Conduit**: Orchustr term for an LLM provider client such as OpenAI or Anthropic.
- **DynState**: `HashMap<String, serde_json::Value>` used for dynamic state at graph, binding, and agent boundaries.
- **Forge Tool**: Schema-described async callable registered in `or-forge`.
- **Loom**: Directed state-graph runtime with explicit entry, exit, branching, and pause behavior.
- **Nexus**: MCP client/server layer implemented by `or-mcp`.
- **OrchState**: Core Rust trait that all strongly typed state objects implement; it defines clone, serde, and merge behavior.
- **Pipeline**: Ordered sequence of async nodes that each return a state patch merged into running state.
- **Prism**: Observability bootstrap crate for tracing subscriber and OTLP export installation.
- **Recall**: Long-term conversational memory store backed by a persistence layer (e.g., PostgreSQL/SQLite) to retain continuous dialogue.
- **Relay**: Concurrent branch executor that merges multiple state patches deterministically.
- **Sentinel**: Agent runtime that composes conduit, forge, and loom into think-act loops.
- **Sieve**: Data parsing constraint system designed to validate LLM outputs against strict JSON schemas before returning.

## Known Gaps & Limitations

- The glossary only covers terms defined directly in the codebase or used consistently across the repository.
- No separate terminology registry was found outside the source tree.
