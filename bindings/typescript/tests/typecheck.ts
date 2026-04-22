import {
  DynState,
  GraphBuilder,
  NodeResult,
  PipelineBuilder,
  PromptBuilder,
  RustCrateBridge,
} from "../index.js";

const template = new PromptBuilder().template("Hello {{name}}").build();
const rendered: string = template.render({ name: "Ralph" });

const graph = new GraphBuilder<Record<string, string>>()
  .add_node("start", async (state) => NodeResult.advance({ ...state, text: rendered }))
  .add_node("finish", async (state) => {
    const current = state as Record<string, string>;
    return NodeResult.exit({ ...current, done: current.text.toUpperCase() });
  })
  .add_edge("start", "finish")
  .set_entry("start")
  .set_exit("finish")
  .build();

void graph.invoke(new DynState({}));

const pipeline = new PipelineBuilder<Record<string, string>>()
  .add_node("one", async (state) => ({ ...state, a: "1" }))
  .build();

void pipeline.invoke({});
void RustCrateBridge.catalog();
