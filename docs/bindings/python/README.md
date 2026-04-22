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

- Python-first helpers: `DynState`, `NodeResult`, `GraphBuilder`, `PromptBuilder`, `ForgeRegistry`, `NexusClient`, `PipelineBuilder`, `RelayBuilder`, `ColonyBuilder`, and workflow helpers in `orchustr.workflows`
- Optional native wrappers: `PyGraphBuilder`, `PyExecutionGraph`, `PyDynState`, `PyNodeResult`, `PyPromptBuilder`, `PyPipelineBuilder`, `PyConduitProvider`, `PyForgeRegistry`, `PyColonyBuilder`, and `PyRelayBuilder`
- Tool wrappers: `SearchTools`, `WebTools`, `VectorTools`, `LoaderTools`, `ExecTools`, `FileTools`, `CommsTools`, and `ProductivityTools`

## Rust to Python Shape Mapping

| Rust concept | Python shape |
|---|---|
| `DynState` | `orchustr.state.DynState` |
| `NodeResult` | `orchustr.result.NodeResult` |
| `PromptBuilder` | `orchustr.prompt.PromptBuilder` |
| `GraphBuilder<DynState>` | `orchustr.graph.GraphBuilder` |
| `or-bridge` native classes | optional `Py*` classes exported from `_runtime.py` |

## Known Gaps & Limitations

- The native bridge is optional; many workflow helpers intentionally remain Python-first.
- Local development installs from source still need to build the extension when native bridge features are required.
