# Python Bindings

The Python package lives in `bindings/python` and is named `orchustr` in `pyproject.toml`. It uses **maturin + PyO3** for the native bridge and Python modules for the higher-level workflow surface. The result is a package that can reach the full workspace without forcing every concept through a raw FFI boundary.

## Version Requirements

- Package metadata: `requires-python = ">=3.10"`
- CI validation: Python `3.14.4` in `.github/workflows/ci.yml`

## Installation

- Local editable install with native extension path: `cd bindings/python && maturin develop`
- Package-style install from source: `pip install -e bindings/python`

## Quickstart

```python
from orchustr import PromptBuilder, SearchTools

prompt = PromptBuilder().template("Find news about {{topic}}").build()
tools = SearchTools()

query = prompt.render({"topic": "retrieval engineering"})
results = tools.search("tavily", {"query": {"query": query}})
```

## Async Support

- `GraphBuilder` and graph execution are async-friendly in the Python package.
- HTTP provider helpers use native Python async clients where the binding owns the behavior.
- The native bridge itself stays JSON/string oriented and sync at the boundary.

## Rust to Python Type Mapping

| Rust concept | Python shape |
|---|---|
| `DynState` | `dict[str, object]` |
| `CompletionResponse` | Python dataclass-like class with `text`, `usage`, `finish_reason` fields |
| `PromptBuilder` | Python class in `orchustr.prompt` |
| `RustCrateBridge.invoke(...)` | PyO3-backed JSON bridge behind `_runtime.py` |

## What Is Exposed

- `PromptBuilder`, `GraphBuilder`, `ForgeRegistry`, `NexusClient`, and conduit helpers remain Python-first APIs.
- `RustCrateBridge` exposes the shared native catalog and invocation surface.
- `SearchTools`, `WebTools`, `VectorTools`, `LoaderTools`, `ExecTools`, `FileTools`, `CommsTools`, and `ProductivityTools` wrap the Rust `or-tools-*` crates.
- Workflow helpers such as `CheckpointGate`, `PipelineBuilder`, `RecallStore`, and `SentinelOrchestrator` keep callback-heavy behavior in Python.

⚠️ Known Gaps & Limitations

- Python exposes every workspace crate, but not as a literal 1:1 projection of every Rust type or trait.
- Native bridge availability still depends on building the extension through `maturin`.
