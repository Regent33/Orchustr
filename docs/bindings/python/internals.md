# Python Binding Internals

## Binding Technology

- Build backend: `maturin`
- Native technology: PyO3 in `crates/or-bridge` behind the `python` feature
- Runtime package: mostly pure Python modules under `bindings/python/orchustr`

## GIL and Native Boundary

- The current native bridge only exchanges strings and JSON text.
- No explicit long-running GIL-sensitive native loops were found in the PyO3 layer.
- Higher-level async behavior lives in Python modules, not inside the native extension.

## Async Bridge Notes

- `orchustr.conduit` wraps blocking HTTP calls in `asyncio.to_thread`.
- Graph execution is async at the Python layer and does not depend on the native module.

⚠️ Known Gaps & Limitations
- The native module is intentionally narrow and not a full workspace wrapper.
- No separate Python-only performance layer beyond the optional prompt/state helpers exists today.
