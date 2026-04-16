# Building Your First Agent in TypeScript

The TypeScript package currently exposes graph and prompt primitives as a JavaScript facade. That makes a graph-first introduction the most faithful starting point.

## Minimal Example

```ts
import { GraphBuilder, PromptBuilder } from "@orchustr/core";

const prompt = new PromptBuilder().template("Answer briefly about {{topic}}.").build();

const graph = new GraphBuilder()
  .addNode("buildPrompt", async (state) => ({ ...state, prompt: prompt.render(state) }))
  .setEntry("buildPrompt")
  .setExit("buildPrompt")
  .build();

const result = await graph.execute({ topic: "tool routing" });
console.log(result.prompt);
```

## Next Step

If you need HTTP provider access, move to the package conduit helpers or the Rust crates themselves.

⚠️ Known Gaps & Limitations
- There is no TypeScript exposure of the Rust `or-sentinel` agent runtime today.
- The TypeScript package does not currently load the native NAPI bridge.
