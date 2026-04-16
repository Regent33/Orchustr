# TypeScript Examples

## Render a Prompt

```ts
import { PromptBuilder } from "@orchustr/core";

const prompt = new PromptBuilder().template("Summarize {{topic}}.").build();
console.log(prompt.render({ topic: "routing" }));
```

## Build a Small Graph

```ts
import { GraphBuilder } from "@orchustr/core";

const graph = new GraphBuilder()
  .addNode("fetch", async (state) => ({ ...state, text: "hello" }))
  .addNode("summarize", async (state) => ({ ...state, summary: String(state.text).toUpperCase() }))
  .addEdge("fetch", "summarize")
  .setEntry("fetch")
  .setExit("summarize")
  .build();

const result = await graph.execute({});
console.log(result.summary);
```

⚠️ Known Gaps & Limitations
- These examples use the current JS package surface and do not invoke the Rust NAPI bridge.
