# Python Installation

## Supported Build Path

The repository configures Python packaging through `bindings/python/pyproject.toml` with `maturin` as the build backend and `../../crates/or-bridge/Cargo.toml` as the Rust manifest path.

## Commands

```bash
cd bindings/python
pip install maturin pytest
maturin develop
```

## Package Layout

- `orchustr/__init__.py`: top-level exports.
- `orchustr/_runtime.py`: optional native bridge loader.
- `orchustr/_orchustr.pyi`: native helper stubs.
- `orchustr/bridge.py`: crate catalog and invocation helpers.
- `orchustr/tools.py`: Rust `or-tools-*` wrappers.
- `orchustr/workflows.py`: binding-local helpers for workflow crates.
- `orchustr/*.py`: Python graph, prompt, conduit, forge, and MCP facades.

⚠️ Known Gaps & Limitations

- No separate wheel publishing workflow exists in the repository today.
- Installing the package still requires building the `or-bridge` extension through `maturin`.
