# Crate Index

| Crate | Role | Internal deps | Key exports | Status |
|---|---|---|---|---|
| `or-core` | Shared contracts, retry, token budgets, in-memory persistence/vector store. | `(none)` | DynState, OrchState, CoreOrchestrator, RetryPolicy | 🟢 Complete |
| `or-anchor` | Chunking and in-memory retrieval pipeline. | or-core | AnchorPipeline, AnchorChunk, RetrievedChunk | 🟢 Complete |
| `or-beacon` | Prompt templating and validation. | or-core | PromptBuilder, PromptTemplate, PromptOrchestrator | 🟢 Complete |
| `or-bridge` | Native prompt/state bridge for Python, Node, and Dart. | or-beacon, or-core | render_prompt_json, normalize_state_json, BridgeError, orchustr_bridge_version (C-ABI) | 🟡 Partial |
| `or-checkpoint` | Pause/resume state serialization. | or-core | CheckpointGate, CheckpointRecord | 🟢 Complete |
| `or-colony` | Multi-agent coordination and aggregation. | or-core | ColonyOrchestrator, ColonyAgentTrait, ColonyResult | 🟢 Complete |
| `or-compass` | Predicate routing. | or-core | CompassRouterBuilder, CompassRouter, RouteSelection | 🟢 Complete |
| `or-conduit` | LLM provider abstraction and adapters (22 providers). | or-core | ConduitProvider, OpenAiCompatConduit, AnthropicConduit, GeminiConduit, CohereConduit, AI21Conduit, HuggingFaceConduit, ReplicateConduit, AzureConduit, BedrockConduit, VertexConduit | 🟢 Complete |
| `or-forge` | Async tool registry and MCP imports. | or-mcp | ForgeRegistry, ForgeTool | 🟢 Complete |
| `or-loom` | Directed graph execution engine. | or-core | GraphBuilder, ExecutionGraph, NodeResult | 🟢 Complete |
| `or-mcp` | MCP client, server, and transports. | or-core | NexusClient, NexusServer, StreamableHttpTransport | 🟡 Partial |
| `or-pipeline` | Sequential state pipeline runtime. | or-core | PipelineBuilder, Pipeline | 🟢 Complete |
| `or-prism` | Observability bootstrap and OTLP export install. | `(none)` | install_global_subscriber, PrismConfig | 🟡 Partial |
| `or-recall` | Short/long/episodic memory stores. | or-core | RecallStore, InMemoryRecallStore, SqliteRecallStore(feature) | 🟢 Complete |
| `or-relay` | Parallel branch execution with deterministic merge. | or-core | RelayBuilder, RelayExecutor, RelayPlan | 🟢 Complete |
| `or-sentinel` | Agent runtime and plan/execute loop. | or-conduit, or-core, or-forge, or-loom | SentinelAgent, PlanExecuteAgent, StepOutcome | 🟢 Complete |
| `or-sieve` | Structured-output and plain-text parsing. | `(none)` | JsonParser, TextParser, JsonSchemaOutput | 🟢 Complete |

⚠️ Known Gaps & Limitations
- Status values in this index describe the current codebase (all tests passing), not a strict release endpoint.
- The index emphasizes public crate roles and omits private helper types for brevity.
