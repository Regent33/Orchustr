# TypeScript Bindings

The TypeScript package lives in `bindings/typescript` and is named `@orchustr/core`. The package currently ships a **pure JavaScript runtime facade** with an `index.d.ts` declaration file; it does **not** yet load a built NAPI addon at runtime even though `or-bridge` contains a Node-targeted native surface.

## Runtime Support

| Environment | Status | Basis |
|---|---|---|
| Node.js | 🟢 Supported | CI runs on Node `20` and the package test suite uses `node`. |
| Browser | 🟡 Partial / unverified | The code uses standard JavaScript objects and `fetch`, but no browser build, bundler config, or browser CI job exists. |

## Installation

- Local package install: `npm install ./bindings/typescript`
- Inside the package directory: `cd bindings/typescript && npm ci`

## Quickstart

```ts
import { GraphBuilder, PromptBuilder } from "@orchustr/core";

const prompt = new PromptBuilder().template("Hello, {{name}}!").build();

const graph = new GraphBuilder()
  .addNode("greet", async (state) => ({ ...state, message: prompt.render(state) }))
  .setEntry("greet")
  .setExit("greet")
  .build();
```

## Promise and Async Usage

- Graph execution is promise-based.
- MCP tool imports and provider methods return promises.
- There is no native streaming iterator wired to the Rust layer in the current package.

## Rust to TypeScript Type Mapping

| Rust concept | TypeScript shape |
|---|---|
| `DynState` | `Record<string, unknown>` |
| `PromptBuilder` | JS class with chainable methods |
| `CompletionResponse` | plain object with `text`, `usage`, and `finishReason` |
| `NexusClient` | JS class that uses `fetch` |

## Bundle and Performance Notes

- The package is currently light because it is plain JS plus types.
- Native NAPI acceleration is not used yet, so runtime characteristics are those of JavaScript helpers rather than Rust-backed execution.

⚠️ Known Gaps & Limitations
- The current package does not yet consume the NAPI exports provided by `or-bridge`.
- Browser support is not explicitly tested or packaged in the repository.
