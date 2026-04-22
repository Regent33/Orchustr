# Crate Index

| Crate | Role | Internal deps | Key exports | Status |
|---|---|---|---|---|
| `or-core` | Shared contracts, retry, token budgets, in-memory persistence/vector store. | `(none)` | DynState, OrchState, CoreOrchestrator, RetryPolicy | Complete |
| `or-anchor` | Chunking and in-memory retrieval pipeline. | or-core | AnchorPipeline, AnchorChunk, RetrievedChunk | Complete |
| `or-beacon` | Prompt templating and validation. | or-core | PromptBuilder, PromptTemplate, PromptOrchestrator | Complete |
| `or-bridge` | Native binding gateway for Python, Node, and Dart, including JSON bridge entry points and feature-gated Python helper classes. | or-beacon, or-conduit, or-core, or-loom, or-prism, or-sieve, or-tools-* | render_prompt_json, normalize_state_json, workspace_catalog_json, invoke_crate_json, BridgeError, python(feature) | Partial |
| `or-checkpoint` | Pause/resume state serialization. | or-core | CheckpointGate, CheckpointRecord | Complete |
| `or-colony` | Multi-agent coordination and aggregation. | or-core | ColonyOrchestrator, ColonyAgentTrait, ColonyResult | Complete |
| `or-compass` | Predicate routing. | or-core | CompassRouterBuilder, CompassRouter, RouteSelection | Complete |
| `or-conduit` | LLM provider abstraction and adapters (19 providers). | or-core | ConduitProvider, OpenAiCompatConduit, AnthropicConduit, GeminiConduit, CohereConduit, AI21Conduit, HuggingFaceConduit, ReplicateConduit, AzureConduit, BedrockConduit, VertexConduit | Complete |
| `or-forge` | Async tool registry and MCP imports. | or-mcp | ForgeRegistry, ForgeTool | Complete |
| `or-loom` | Directed graph execution engine with inspection and optional schema compilation support. | or-core, or-schema(feature=`serde`) | GraphBuilder, ExecutionGraph, NodeResult, GraphInspection, NodeRegistry(feature=`serde`) | Complete |
| `or-schema` | Serializable graph descriptors and JSON/YAML loading helpers. | `(none)` | GraphSpec, NodeSpec, EdgeSpec, SchemaError | Complete |
| `or-mcp` | MCP client, server, and transports. | or-core | NexusClient, NexusServer, StreamableHttpTransport | Partial |
| `or-pipeline` | Sequential state pipeline runtime. | or-core | PipelineBuilder, Pipeline | Complete |
| `or-prism` | Observability bootstrap and optional local dashboard bridge. | or-lens(feature=`lens`) | install_global_subscriber, PrismConfig, init_with_dashboard(feature=`lens`) | Partial |
| `or-lens` | Optional local execution dashboard and in-process trace collection. | `(none)` | LensHandle, LensLayer, SpanCollector, ExecutionSnapshot | Partial |
| `or-recall` | Short/long/episodic memory stores. | or-core | RecallStore, InMemoryRecallStore, SqliteRecallStore(feature) | Complete |
| `or-relay` | Parallel branch execution with deterministic merge. | or-core | RelayBuilder, RelayExecutor, RelayPlan | Complete |
| `or-sentinel` | Agent runtime with legacy ReAct plus additive custom loop topologies. | or-conduit, or-core, or-forge, or-loom | SentinelAgent, SentinelAgentBuilder, LoopTopology, ReActTopology, PlanExecuteTopology, ReflectionTopology, StepOutcome | Complete |
| `or-sieve` | Structured-output and plain-text parsing. | `(none)` | JsonParser, TextParser, JsonSchemaOutput | Complete |
| `or-tools-core` | Shared tool traits, registry, dispatcher, metadata, and tool errors. | `(none)` | Tool, ToolRegistry, ToolDispatcher, ToolMeta, ToolError | Implemented |
| `or-tools-search` | Feature-gated search providers and fallback orchestration. | or-tools-core | SearchProvider, SearchQuery, SearchResponse, SearchOrchestrator | Implemented |
| `or-tools-web` | Browser fetch and scraping backends with URL validation. | or-tools-core | WebBrowser, Scraper, FetchRequest, FetchResponse, WebOrchestrator | Implemented |
| `or-tools-vector` | Feature-gated vector store clients and RAG-style operations. | or-core, or-tools-core | VectorStoreClient, CollectionConfig, QueryFilter, VectorMatch, RagOrchestrator | Implemented |
| `or-tools-loaders` | Document loaders for text, markdown, JSON, CSV, HTML, and PDF. | or-tools-core | DocumentLoader, LoaderRequest, Document, LoaderOrchestrator | Implemented |
| `or-tools-exec` | Local and remote code execution runtimes. | or-tools-core | CodeExecutor, ExecRequest, ExecResult, Language, ExecOrchestrator | Implemented |
| `or-tools-file` | File storage and external data source integrations. | or-tools-core | FileStore, DataSource, FileContent, ResearchPaper, FileOrchestrator | Implemented |
| `or-tools-comms` | Outbound messaging integrations for SMS and chat platforms. | or-tools-core | MessageSender, Message, SendResult, Channel, CommsOrchestrator | Implemented |
| `or-tools-productivity` | Productivity clients for email, calendar, tracking, knowledge, and messaging. | or-tools-core | EmailClient, CalendarClient, ProjectTracker, ProductivityOrchestrator, ProductivityTool | Implemented |

## Known Gaps & Limitations

- Status values in this index describe the current codebase, not a strict release endpoint.
- The index emphasizes public crate roles and omits private helper types for brevity.
