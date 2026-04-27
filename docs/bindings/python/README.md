# Python Bindings

The Python package lives in `bindings/python` and is named `orchustr`. It combines Python-first workflow helpers with an optional PyO3 native bridge surface for selected Rust-backed helpers.

## Installation

- Editable install: `pip install -e bindings/python`
- Native-extension development path: `cd bindings/python && maturin develop`

## Quickstart

```python
from orchustr import GraphBuilder, DynState, NodeResult
import asyncio

async def think_node(state: DynState) -> NodeResult:
    state["thought"] = "I should look this up"
    return NodeResult.advance(state)

async def act_node(state: DynState) -> NodeResult:
    state["action"] = "search"
    return NodeResult.exit(state)

async def main():
    graph = (
        GraphBuilder()
        .add_node("think", think_node)
        .add_node("act", act_node)
        .add_edge("think", "act")
        .set_entry("think")
        .set_exit("act")
        .build()
    )
    result = await graph.invoke(DynState({"query": "What is Orchustr?"}))
    print(result)

asyncio.run(main())
```

## What Is Exposed

- Python-first helpers: `DynState`, `NodeResult`, `GraphBuilder`, `PromptBuilder`, `ForgeRegistry`, `NexusClient`, `PipelineBuilder`, `RelayBuilder`, `ColonyBuilder`, `ColonyOrchestrator`, `SentinelOrchestrator`, `RecallStore`, `CheckpointGate`, and other workflow helpers in `orchustr.workflows`
- Optional native wrappers (pyo3 classes from `_orchustr`): `PyGraphBuilder`, `PyExecutionGraph`, `PyDynState`, `PyNodeResult`, `PyPromptBuilder`, `PyPipelineBuilder`, `PyConduitProvider`, `PyForgeRegistry`, `PyColonyBuilder`, and `PyRelayBuilder`. Registries / builders **retain** registered Python callables — for example `PyForgeRegistry.register("search", fn)` followed by `PyForgeRegistry.invoke("search", args)` actually calls `fn`. (Earlier versions silently discarded the handler argument.)
- Tool wrappers: `SearchTools`, `WebTools`, `VectorTools`, `LoaderTools`, `ExecTools`, `FileTools`, `CommsTools`, and `ProductivityTools` — all routed through the Rust bridge via `invoke_crate_json`.

## Rust to Python Shape Mapping

| Rust concept | Python shape |
|---|---|
| `DynState` | `orchustr.state.DynState` |
| `NodeResult` | `orchustr.result.NodeResult` |
| `PromptBuilder` | `orchustr.prompt.PromptBuilder` |
| `GraphBuilder<DynState>` | `orchustr.graph.GraphBuilder` |
| `or-bridge` native classes | optional `Py*` classes exported from `_runtime.py` |

## Two `GraphBuilder`s — which to use

`orchustr` exports **two** classes named `GraphBuilder`:

- `orchustr.GraphBuilder` (pure Python, [graph.py](../../../bindings/python/orchustr/graph.py)) — drives the graph executor in Python, calling each handler with an awaitable and following branch / pause / advance results. **Use this for actual graph execution.**
- `orchustr.PyGraphBuilder` (pyo3 bridge type from `_orchustr`) — structural metadata container. Stores the registered handlers so `PyExecutionGraph.get_handler(name)` returns the actual callable. Useful as a callback-handle store or for porting agent runtimes; does not itself execute the graph.

If you import `from orchustr import GraphBuilder`, you get the
pure-Python class. The native one only appears via `from orchustr._runtime import PyGraphBuilder`.

## Known Gaps & Limitations

- The native bridge is optional; many workflow helpers intentionally remain Python-first.
- Local development installs from source still need to build the extension when native bridge features are required.
- `PyGraphBuilder` and `PyForgeRegistry` retain handlers but do not yet drive a Rust-side graph executor — running the graph still happens in Python (audit item #23).
