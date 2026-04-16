# Orchustr vs LangChain / LangGraph — Crate Comparison

> This document maps every Orchustr crate to the closest conceptual equivalent
> in the Python LangChain + LangGraph ecosystem.  
> It is **not** a feature-for-feature mapping — Orchustr is written in Rust
> with strict compile-time safety guarantees; the equivalents listed are the
> closest analogues in spirit and responsibility.

---

## Comparison Table

| Orchustr Crate | LangChain / LangGraph Equivalent | Description |
|---|---|---|
| **or-core** | `langchain_core` (base package) | Foundational shared contracts. Defines `OrchState`, `DynState`, `PersistenceBackend`, `VectorStore`, `RetryPolicy`, and `TokenBudget`. The whole workspace depends on this crate — nothing else does. Equivalent to `langchain_core`, the package that holds all base classes, interfaces, and schemas that every other LC component depends on. |
| **or-anchor** | `langchain.text_splitter` + `langchain.retrievers` | RAG pipeline: splits raw text into chunks, embeds them into an in-memory vector store, and retrieves the top-K most relevant chunks by cosine similarity. Maps to `RecursiveCharacterTextSplitter` for chunking and `VectorStoreRetriever` for retrieval. |
| **or-beacon** | `langchain.prompts` — `PromptTemplate`, `ChatPromptTemplate` | Prompt engineering layer. Compiles named-variable templates, sanitises inputs (null-byte stripping, re-expansion guard), and renders final prompt strings. Equivalent to `PromptTemplate` / `ChatPromptTemplate` with built-in input validation. |
| **or-bridge** | LangChain `Tools` + `RunnableLambda` (cross-language boundary) | Native FFI bridge exposing Orchustr's prompt renderer and state normaliser to Python and Node.js over JSON. Analogous to wrapping a LangChain `RunnableLambda` so a non-Python host can call it via a serialised boundary. |
| **or-checkpoint** | LangGraph `MemorySaver` / `SqliteSaver` (checkpointers) | Serialises and persists graph state at a named node so an interrupted run can be resumed exactly where it stopped. Direct equivalent of LangGraph's `MemorySaver` or `SqliteSaver` checkpointer, including the pause/resume contract. |
| **or-colony** | LangGraph multi-agent `supervisor` pattern — `Command`, `Send` routing | Coordinates a roster of specialised agents: each agent produces a `ColonyMessage`, results are aggregated into a `ColonyResult`. Mirrors the LangGraph supervisor pattern (`create_supervisor`) where a parent node fans out tasks then collects responses. |
| **or-compass** | LangGraph conditional edges — `add_conditional_edges` | Predicate-based runtime routing. A `CompassRouter` maps string route names to boolean predicates and selects the next node. Direct equivalent of LangGraph `add_conditional_edges` / `conditional_edge` functions. |
| **or-conduit** | `langchain_openai`, `langchain_anthropic`, `langchain_google_genai`, `langchain_community` (22 providers) | LLM provider abstraction layer. A single `ConduitProvider` trait wraps 22 providers (OpenAI, Anthropic, Gemini, Cohere, Azure, Bedrock, Vertex, Ollama, OpenRouter, Groq …). Equivalent to LangChain's family of `BaseChatModel` integrations spread across `langchain_openai`, `langchain_anthropic`, `langchain_google_genai`, and `langchain_community`. |
| **or-forge** | `langchain.tools` — `Tool`, `StructuredTool`, `@tool` decorator + MCP tool imports | Async tool registry. Tools are registered with JSON-schema argument definitions and invoked by name. Supports importing tool definitions from an MCP server. Equivalent to LangChain's `StructuredTool` / `@tool` decorator combined with the MCP tool-import utilities (`load_mcp_tools`). |
| **or-loom** | LangGraph `StateGraph` + `add_node` / `add_edge` / `compile` | Directed graph execution engine. A `GraphBuilder` compiles a typed state-machine; `LoomOrchestrator` drives node execution, conditional branching, and pause points. The direct Rust equivalent of LangGraph's `StateGraph` — `add_node`, `add_edge`, `compile`, `invoke`. |
| **or-mcp** | `langchain_mcp_adapters` — `MultiServerMCPClient` | MCP (Model Context Protocol) client and server. Connects to remote tool servers over Streamable-HTTP or stdio transports. Maps to `langchain_mcp_adapters.client.MultiServerMCPClient` for consuming remote MCP tools. |
| **or-pipeline** | LangChain `RunnableSequence` / `chain1 \| chain2` pipe operator | Sequential multi-step pipeline. Nodes execute in insertion order; the output state of one step is the input to the next. Equivalent to LangChain's `RunnableSequence` (the `\|` pipe composition), running a series of transforms in a fixed order. |
| **or-prism** | LangChain / LangSmith `callbacks` + OpenTelemetry integration | Observability bootstrap. Installs a global `tracing` subscriber that exports OpenTelemetry spans to an OTLP collector. Analogous to LangChain's `LangSmithCallbackHandler` / `OpenTelemetryCallbackHandler` for tracing agent runs. |
| **or-recall** | LangGraph `MemorySaver` (short-term) + `langchain.memory` stores (long-term) | Three-tier memory system: `Episodic` (specific past events), `Semantic` (general facts), and `Procedural` (how-to knowledge). Optionally backed by SQLite with WAL mode. Maps to LangGraph's in-graph `MemorySaver` for short-term plus LangChain `ConversationBufferMemory` / `VectorStoreRetrieverMemory` for long-term recall. |
| **or-relay** | LangGraph `Send` / fan-out subgraph pattern | Parallel branch executor. Fans out a shared state to N branches concurrently, then deterministically merges all patches back via `OrchState::merge`. Equivalent to LangGraph's `Send` API used in map-reduce subgraphs where multiple agent nodes run in parallel and results are combined. |
| **or-sentinel** | LangGraph agent node + `create_react_agent` / Plan-and-Execute pattern | Agent runtime: wraps an LLM conduit, a tool registry, and a graph engine into a complete Plan→Execute→Observe loop. Equivalent to `create_react_agent` (ReAct loop) or the `PlanAndExecute` agent pattern in LangGraph, where the agent iterates until a terminal condition. |
| **or-sieve** | `langchain.output_parsers` — `JsonOutputParser`, `PydanticOutputParser`, `StrOutputParser` | Structured-output parser. Deserialises LLM text responses into typed Rust structs via JSON-schema, or extracts cleaned plain-text. Maps to LangChain's `JsonOutputParser` / `PydanticOutputParser` (structured) and `StrOutputParser` (text). |

---

## Architecture Layer Map

```
┌─────────────────────────────────────────────────────────────┐
│                     Agent Runtime Layer                      │
│  or-sentinel  ←→  LangGraph create_react_agent / Plan-Exec  │
│  or-colony    ←→  LangGraph supervisor multi-agent pattern   │
├─────────────────────────────────────────────────────────────┤
│                     Orchestration Layer                      │
│  or-loom      ←→  LangGraph StateGraph                       │
│  or-pipeline  ←→  LangChain RunnableSequence                 │
│  or-relay     ←→  LangGraph Send / fan-out                   │
│  or-compass   ←→  LangGraph add_conditional_edges            │
│  or-checkpoint←→  LangGraph MemorySaver / SqliteSaver        │
├─────────────────────────────────────────────────────────────┤
│                     Capability Layer                         │
│  or-conduit   ←→  langchain_openai / anthropic / google ...  │
│  or-forge     ←→  langchain.tools + load_mcp_tools           │
│  or-beacon    ←→  langchain.prompts PromptTemplate           │
│  or-sieve     ←→  langchain.output_parsers                   │
│  or-anchor    ←→  text_splitter + VectorStoreRetriever       │
│  or-recall    ←→  MemorySaver + ConversationBufferMemory     │
├─────────────────────────────────────────────────────────────┤
│                     Infrastructure Layer                     │
│  or-mcp       ←→  langchain_mcp_adapters                     │
│  or-prism     ←→  LangSmith / OpenTelemetry callbacks        │
│  or-bridge    ←→  RunnableLambda (cross-language FFI)        │
│  or-core      ←→  langchain_core                             │
└─────────────────────────────────────────────────────────────┘
```

---

> **References**
> - LangGraph source: <https://github.com/langchain-ai/langgraph>
> - LangChain source: <https://github.com/langchain-ai/langchain>
> - Orchustr crate index: [reference/crate-index.md](./reference/crate-index.md)
