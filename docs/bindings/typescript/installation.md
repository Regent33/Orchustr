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
npm run build:native
npm run typecheck
npm test
```

## What Gets Installed

- `src/index.js`: runtime implementation.
- `index.d.ts`: declaration surface.
- `src/native.js`: optional native addon loader.
- `scripts/build-native.js`: local native build helper.
- `tests/`: local test and typecheck examples.

⚠️ Known Gaps & Limitations

- No publish workflow is defined for npm release automation.
- The native addon is a local build step rather than an automatically packaged artifact.
