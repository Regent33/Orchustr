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

## Call a Rust Tool Crate

```ts
import { SearchTools } from "@orchustr/core";

const search = new SearchTools();
const results = search.search("tavily", {
  query: { query: "Rust agent frameworks" },
});

console.log(results.results?.length ?? 0);
```

⚠️ Known Gaps & Limitations

- Native crate examples require a built addon from `npm run build:native`.
