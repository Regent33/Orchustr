# Building Your First Agent in TypeScript

The TypeScript package exposes graph and prompt primitives as JavaScript helpers and can optionally reach Rust-backed crates through `RustCrateBridge` and the `*Tools` wrappers. A graph-first introduction is still the simplest place to start.

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

If you need search, web, file, vector, comms, or productivity tools, move next to `SearchTools`, `WebTools`, `FileTools`, or the other `*Tools` wrappers after building the optional native addon.

⚠️ Known Gaps & Limitations

- The binding layer exposes `or-sentinel` through TypeScript workflow helpers rather than a raw direct export of the Rust agent runtime.
- Native bridge usage still depends on a local `npm run build:native` step.
