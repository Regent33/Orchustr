# Python Bindings

The Python package lives in `bindings/python` and is named `orchustr` in `pyproject.toml`. It uses **maturin + PyO3** for the optional native module and pure Python modules for most of the current package surface.

## Version Requirements

- Package metadata: `requires-python = ">=3.10"`
- CI validation: Python `3.14.4` in `.github/workflows/ci.yml`

## Installation

- Local editable install with native extension path: `cd bindings/python && maturin develop`
- Package-style install from source: `pip install -e bindings/python`

## Quickstart

```python
from orchustr import GraphBuilder, PromptBuilder

prompt = PromptBuilder().template("Hello, {{name}}!").build()

async def greet(state: dict) -> dict:
    return {**state, "message": prompt.render(state)}

graph = GraphBuilder().add_node("greet", greet).set_entry("greet").set_exit("greet").build()
```

## Async Support

- `GraphBuilder` and graph execution are async-friendly in the Python package.
- HTTP provider helpers use `asyncio.to_thread` around `urllib.request` rather than a native async HTTP client.
- The native helper module itself exposes sync JSON/string helpers only.

## Rust to Python Type Mapping

| Rust concept | Python shape |
|---|---|
| `DynState` | `dict[str, object]` |
| `CompletionResponse` | Python dataclass-like class with `text`, `usage`, `finish_reason` fields |
| `PromptBuilder` | Python class in `orchustr.prompt` |
| `render_prompt_json` | native helper behind `_runtime.py` |

## Known Limitations vs Native Rust

- Most of the Python package is implemented in Python, not as direct wrappers over the Rust crates.
- The native PyO3 module only exposes prompt rendering and JSON state normalization.

⚠️ Known Gaps & Limitations
- There is no Python exposure of every Rust crate or trait in the current repository.
- Provider support relies on standard-library HTTP helpers rather than the Rust provider adapters.
