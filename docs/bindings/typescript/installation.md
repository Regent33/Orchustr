# TypeScript Installation

## Package Metadata

- Package name: `@orchustr/core`
- Module type: ESM (`"type": "module"`)
- Types entry: `index.d.ts`
- CI node version: `20`
- Dev type checker: `typescript@6.0.2`

## Commands

```bash
cd bindings/typescript
npm ci
npm run typecheck
npm test
```

## What Gets Installed

- `src/index.js`: runtime implementation.
- `index.d.ts`: declaration surface.
- `tests/`: local test and typecheck examples.

⚠️ Known Gaps & Limitations
- No publish workflow is defined for npm release automation.
- No WASM build or native addon packaging step exists in the current package.
