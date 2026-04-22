# TypeScript Binding Internals

## Binding Technology

- Runtime package: plain JavaScript in `src/index.js`
- Type declarations: `index.d.ts`
- Dev tooling: `typescript@6.0.2` via `tsconfig.json`
- Native target present elsewhere in repo: `or-bridge/src/node.rs` with NAPI macros

## Package Shape

- `package.json` declares an ESM package.
- `src/native.js` performs optional addon loading.
- `scripts/build-native.js` builds `or-bridge` with the `node` feature and copies the artifact into `bindings/typescript/native/`.
- `npm run typecheck` validates the declaration surface against `tests/typecheck.ts`.
- `npm test` runs `tests/bindings.test.js` directly under Node.

## Relationship to `or-bridge`

- `or-bridge` exposes Node-targeted functions through NAPI.
- `src/native.js` loads the addon when a built artifact is available.
- `src/bridge.js` wraps the native catalog and crate invocation surface in `RustCrateBridge`.
- `src/tools.js` layers friendly wrappers for the Rust `or-tools-*` crates on top of that bridge.
- `src/workflows.js` keeps callback-heavy runtime helpers in JavaScript.

⚠️ Known Gaps & Limitations

- Native addon loading is optional and still local-build oriented.
- No browser-specific bundle configuration exists in the repository.
