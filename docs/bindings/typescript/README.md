# TypeScript Bindings

The TypeScript package lives in `bindings/typescript` and is published as `@orchustr/core`. It exposes JavaScript-first graph, prompt, conduit, tool, and workflow helpers plus an optional native bridge for selected Rust-backed crate calls.

## Installation

- Local package install: `npm install ./bindings/typescript`
- Inside the package directory: `cd bindings/typescript && npm ci`
- Optional native bridge build: `npm run build:native`

## Quickstart

```ts
import { DynState, GraphBuilder, NodeResult } from "@orchustr/core";

const graph = new GraphBuilder()
  .addNode("think", async (state) => {
    state.thought = "I should look this up";
    return NodeResult.advance(state);
  })
  .addNode("act", async (state) => {
    state.action = "search";
    return NodeResult.exit(state);
  })
  .addEdge("think", "act")
  .setEntry("think")
  .setExit("act")
  .build();

const result = await graph.invoke(new DynState({ query: "What is Orchustr?" }));
console.log(result);
```

## What Is Exposed

- State and graph helpers: `DynState`, `NodeResult`, `GraphBuilder`
- Prompt and conduit helpers: `PromptBuilder`, `ConduitProvider`, `OpenAiConduit`, `AnthropicConduit`, `OpenAiCompatConduit`
- Tools and MCP helpers: `ForgeRegistry`, `NexusClient`, and the `*Tools` wrappers
- Workflow helpers: `PipelineBuilder`, `RelayBuilder`, `ColonyBuilder`, `SentinelOrchestrator`, `PrismConfig`, and related classes from `src/workflows.js`
- Optional native bridge: `RustCrateBridge`

## Known Gaps & Limitations

- The package is JavaScript-first; it does not attempt to mirror every Rust type exactly.
- Browser support is not a primary tested target in this repository.
