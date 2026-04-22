# TypeScript Bindings

The TypeScript package lives in `bindings/typescript` and is named `@orchustr/core`. It ships an ESM runtime facade with an `index.d.ts` declaration file, plus an optional native bridge path that can be built locally for Rust-backed crate calls.

## Runtime Support

| Environment | Status | Basis |
|---|---|---|
| Node.js | Supported | CI runs on Node `20` and the package test suite uses `node`. |
| Browser | Partial / unverified | The code uses standard JavaScript objects and `fetch`, but no browser build, bundler config, or browser CI job exists. |

## Installation

- Local package install: `npm install ./bindings/typescript`
- Inside the package directory: `cd bindings/typescript && npm ci`
- Optional native bridge build: `npm run build:native`

## Quickstart

```ts
import { GraphBuilder, PromptBuilder, SearchTools } from "@orchustr/core";

const prompt = new PromptBuilder().template("Hello, {{name}}!").build();
const tools = new SearchTools();

const graph = new GraphBuilder()
  .addNode("greet", async (state) => ({ ...state, message: prompt.render(state) }))
  .setEntry("greet")
  .setExit("greet")
  .build();

const results = tools.search("tavily", {
  query: { query: "agent tooling" },
});
```

## Promise and Async Usage

- Graph execution is promise-based.
- MCP tool imports and provider methods return promises.
- `RustCrateBridge` exposes sync JSON-style native calls when the addon is present.

## Rust to TypeScript Type Mapping

| Rust concept | TypeScript shape |
|---|---|
| `DynState` | `Record<string, unknown>` |
| `PromptBuilder` | JS class with chainable methods |
| `CompletionResponse` | plain object with `text`, `usage`, and `finishReason` |
| `NexusClient` | JS class that uses `fetch` |
| `RustCrateBridge.invoke(...)` | optional NAPI-backed JSON bridge |

## Bundle and Performance Notes

- The package stays lightweight by making the native addon optional.
- Graph, MCP, and workflow helpers remain JS-first, while Rust-backed tool and crate operations can use the native bridge when present.

⚠️ Known Gaps & Limitations

- The package exposes every workspace crate, but not as a literal 1:1 projection of every Rust type or trait.
- Browser support is not explicitly tested or packaged in the repository.
