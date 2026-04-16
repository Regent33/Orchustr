# Binding Architecture

The repository exposes two language packages, but they are not symmetrical yet. Python uses a mixed strategy with an optional native PyO3 module plus pure-Python fallbacks, while the TypeScript package is currently a pure JavaScript facade paired with `.d.ts` declarations.

## Current Binding Stack

- **Rust bridge**: `or-bridge` exports `render_prompt_json` and `normalize_state_json`.
- **Python package**: `bindings/python` can load `orchustr._orchustr` when built through `maturin`, but most package functionality is implemented in Python modules today.
- **TypeScript package**: `bindings/typescript` ships `src/index.js` and `index.d.ts`; it does not currently import a generated native addon.

## Bridge Role

- `or-bridge/src/python.rs` exposes PyO3 functions and a `_orchustr` module.
- `or-bridge/src/node.rs` exposes NAPI functions with the same narrow helper surface.
- The bridge keeps the native API intentionally small: prompt rendering and JSON state normalization only.

## Data Conversion

- Rust `DynState` maps to JSON object strings at the native bridge boundary.
- Python primarily exchanges `dict` objects and falls back to pure-Python rendering if the native module is unavailable.
- TypeScript primarily exchanges plain JavaScript objects and strings in the facade package.

⚠️ Known Gaps & Limitations
- The TypeScript package does not yet consume the NAPI exports from `or-bridge`.
- The Python package does not expose the full Rust workspace API through PyO3; most higher-level behavior is implemented separately in Python.
