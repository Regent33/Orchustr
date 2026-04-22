# Python Binding Internals

## Binding Technology

- Build backend: `maturin`
- Native technology: PyO3 in `crates/or-bridge` behind the `python` feature
- Runtime package: Python modules under `bindings/python/orchustr`

## GIL and Native Boundary

- The native bridge exchanges strings and JSON text.
- No explicit long-running GIL-sensitive native loops were found in the PyO3 layer.
- Higher-level async behavior and callback-heavy workflow composition live in Python modules, not inside the native extension.

## Async Bridge Notes

- `orchustr.bridge` uses the shared `or-bridge` JSON invocation surface.
- `orchustr.tools` translates Python dictionaries into native bridge payloads for the Rust `or-tools-*` crates.
- Graph, relay, pipeline, checkpoint, recall, and sentinel helpers stay in Python where closures and host-language control flow are easier to express.

⚠️ Known Gaps & Limitations

- The native module is intentionally JSON-oriented rather than exposing every Rust trait or generic directly.
- When the extension is unavailable, native crate invocation is unavailable too, but the Python-local helpers still remain usable.
