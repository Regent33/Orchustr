# TypeScript Binding Internals

## Binding Technology

- Runtime package: plain JavaScript in `src/index.js`
- Type declarations: `index.d.ts`
- Dev tooling: `typescript@6.0.2` via `tsconfig.json`
- Native target present elsewhere in repo: `or-bridge/src/node.rs` with NAPI macros

## Package Shape

- `package.json` declares an ESM package.
- `npm run typecheck` validates the declaration surface against `tests/typecheck.ts`.
- `npm test` runs `tests/bindings.test.js` directly under Node.

## Relationship to `or-bridge`

- `or-bridge` exposes Node-targeted functions through NAPI.
- The current TypeScript package does not import or load that addon.
- As a result, the package behaves like a lightweight JS facade today rather than a true native bridge.

⚠️ Known Gaps & Limitations
- There is no NAPI packaging or runtime loading code in `bindings/typescript`.
- No browser-specific bundle configuration exists in the repository.
