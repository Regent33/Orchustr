# API Reference Matrix

This matrix focuses on the user-facing surfaces that changed or became newly important across Phases 1-6. It is intentionally narrower than a full workspace dump so it can stay accurate.

## Availability Legend

- `Native`: exposed by the Rust crate directly
- `Binding helper`: implemented in the binding language with Orchustr-compatible behavior
- `Native wrapper`: exposed through the optional native bridge
- `Not exposed`: no matching public surface today

## Graph, State, and Prompt Surfaces

| Surface | Rust | Python | TypeScript | Notes |
|---|---|---|---|---|
| `DynState` | Native (`or-core`) | Binding helper (`orchustr.state.DynState`) plus `PyDynState` when the native bridge is present | Binding helper (`DynState`) | Python and TypeScript now expose additive state classes for graph authoring. |
| `NodeResult` | Native (`or-loom`) | Binding helper (`orchustr.result.NodeResult`) plus `PyNodeResult` when the native bridge is present | Binding helper (`NodeResult`) | Supports `advance`, `exit`, `branch`, and `pause`. |
| `GraphBuilder` | Native (`or-loom`) | Binding helper (`orchustr.graph.GraphBuilder`) plus `PyGraphBuilder` and `PyExecutionGraph` | Binding helper (`GraphBuilder`) | Python and TypeScript graph helpers support explicit `NodeResult`-based control flow and `invoke()`. |
| `PromptBuilder` | Native (`or-beacon`) | Binding helper plus `PyPromptBuilder` | Binding helper | Prompt helpers remain intentionally ergonomic in each binding. |
| `PipelineBuilder` | Native (`or-pipeline`) | Binding helper plus `PyPipelineBuilder` and `PyPipeline` | Binding helper | Sequential pipeline helpers exist in both updated bindings. |
| `RelayBuilder` | Native (`or-relay`) | Binding helper plus `PyRelayBuilder` and `PyRelayPlan` | Binding helper | TypeScript includes both `addBranch` and `add_branch` aliases. |
| `ColonyBuilder` | Native (`or-colony`) | Binding helper plus `PyColonyBuilder` | Binding helper | Added as an ergonomic builder surface in Python and TypeScript. |

## Schema and Agent Surfaces

| Surface | Rust | Python | TypeScript | Notes |
|---|---|---|---|---|
| `GraphSpec` / `NodeSpec` / `EdgeSpec` | Native (`or-schema`) | Not exposed | Not exposed | Serializable graph descriptors are currently Rust-first. |
| `NodeRegistry` | Native (`or-loom`, feature=`serde`) | Not exposed | Not exposed | Compiles named handlers and conditional edges into a live `ExecutionGraph`. |
| `LoopTopology` | Native (`or-sentinel`) | Not exposed | Not exposed | Additive custom loop extension point for sentinel agents. |
| `SentinelAgentBuilder` | Native (`or-sentinel`) | Not exposed | Not exposed | Builds agents from built-in or custom topologies while preserving legacy `SentinelAgent::new`. |
| `ReActTopology` / `PlanExecuteTopology` / `ReflectionTopology` | Native (`or-sentinel`) | Not exposed | Not exposed | Built-in loop shapes added in Phase 1. |

## Observability Surfaces

| Surface | Rust | Python | TypeScript | Notes |
|---|---|---|---|---|
| `install_global_subscriber` | Native (`or-prism`) | Binding helper (`install_global_subscriber`) | Binding helper (`installGlobalSubscriber`) | Installs OTLP export and JSON tracing output. |
| `init_with_dashboard` | Native (`or-prism`, feature=`lens`) | Not exposed | Not exposed | Starts the local `or-lens` dashboard and installs a trace mirroring layer. |
| `or-lens` dashboard | Native (`or-lens`, feature=`dashboard`) | Not exposed | Not exposed | Current implementation is an in-process Axum dashboard backed by `LensLayer` and `SpanCollector`. |

## MCP and CLI Surfaces

| Surface | Rust | Python | TypeScript | Notes |
|---|---|---|---|---|
| `ForgeRegistry::import_all_from_mcp` | Native (`or-forge`) | Not exposed | Not exposed | Imports every tool exposed by one HTTP MCP server and returns `ImportSummary`. |
| `ForgeRegistry::import_all_from_multi_mcp` | Native (`or-forge`) | Not exposed | Not exposed | Bridges `or-mcp::MultiMcpClient` into `ForgeRegistry` without creating a crate cycle. |
| `MultiMcpClient` / `MultiMcpSession` | Native (`or-mcp`) | Not exposed | Not exposed | Connects to multiple MCP servers concurrently and prefixes duplicate tool names by server name. |
| `known_servers::*` | Native (`or-mcp`) | Not exposed | Not exposed | Curated typed presets for filesystem, Brave Search, GitHub, Slack, and Postgres MCP servers. |
| `orchustr` CLI | Native binary (`or-cli`) | Not exposed | Not exposed | Supports `init`, `lint`, `run`, `trace`, `new node`, and `new topology`. |

## Notes

- Python and TypeScript now have stronger offline graph-authoring parity than before, but they still do not mirror every Rust type one-for-one.
- `or-schema` and `NodeRegistry` are intentionally Rust-first right now so graph descriptors can compile against registered Rust handlers safely.
- The current local dashboard path is in-process rather than a standalone OTLP receiver service.
- `or-cli::run_project` currently validates and hands parsed config to a runner implementation; the default runner is intentionally a no-op scaffold hook.
