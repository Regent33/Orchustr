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

## Streaming

`OpenAiConduit`, `AnthropicConduit`, and `OpenAiCompatConduit` (which
covers OpenRouter, Groq, Together, Fireworks, DeepSeek, Mistral, xAI,
NVIDIA, and Ollama) all expose a real SSE-driven `streamText(prompt)`
async iterator that yields token-level deltas as they arrive:

```ts
const conduit = OpenAiCompatConduit.openrouter(apiKey, model);
for await (const delta of conduit.streamText("Say hi token by token")) {
  process.stdout.write(delta);
}
```

The shared `_sseEvents(response)` parser in `src/index.js` handles the
three wire formats:
- OpenAI Responses API → `event: response.output_text.delta`
- Anthropic Messages API → `event: content_block_delta`
- OpenAI Chat Completions (everything else) → `data: {choices:[{delta:{content:...}}]}`

If the runtime lacks `ReadableStream` support on the fetch response,
`streamText` falls back to a single non-streaming chunk.

## Known Gaps & Limitations

- The package is JavaScript-first; it does not attempt to mirror every Rust type exactly.
- Browser support is not a primary tested target in this repository.
- `addNode`/`add_node` aliases are both supported during a deprecation cycle; pick the camelCase form for new code.
