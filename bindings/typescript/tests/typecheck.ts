import {
  GraphBuilder,
  PipelineBuilder,
  PromptBuilder,
  RustCrateBridge,
} from "../index.js";

const template = new PromptBuilder().template("Hello {{name}}").build();
const rendered: string = template.render({ name: "Ralph" });

const graph = new GraphBuilder<Record<string, string>>()
  .addNode("start", async (state) => ({ ...state, text: rendered }))
  .addNode("finish", async (state) => ({ ...state, done: state.text.toUpperCase() }))
  .addEdge("start", "finish")
  .setEntry("start")
  .setExit("finish")
  .build();

void graph.execute({});

const pipeline = new PipelineBuilder<Record<string, string>>()
  .addNode("one", async (state) => ({ ...state, a: "1" }))
  .build();

void pipeline.execute({});
void RustCrateBridge.catalog();
