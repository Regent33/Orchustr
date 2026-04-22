import assert from "node:assert/strict";

import {
  CoreOrchestrator,
  DynState,
  GraphBuilder,
  NodeResult,
  PipelineBuilder,
  PromptBuilder,
  RustCrateBridge,
  TokenBudget,
} from "../src/index.js";

function testPromptBuilderRendersVariables() {
  const template = new PromptBuilder().template("Hello {{name}}").build();
  assert.equal(template.render({ name: "Ralph" }), "Hello Ralph");
}

function testPromptBuilderSanitizesControlCharacters() {
  const template = new PromptBuilder().template("Hello {{name}}").build();
  assert.equal(template.render({ name: "Ra\u0007lph" }), "Hello Ralph");
}

async function testGraphBuilderExecutesAsyncHandlers() {
  const graph = new GraphBuilder()
    .addNode("start", async (state) => ({ ...state, text: "hello" }))
    .addNode("finish", async (state) => ({ ...state, done: state.text.toUpperCase() }))
    .addEdge("start", "finish")
    .setEntry("start")
    .setExit("finish")
    .build();
  const result = await graph.execute({});
  assert.equal(result.done, "HELLO");
}

async function testGraphBuilderSupportsNodeResult() {
  const graph = new GraphBuilder()
    .add_node("think", async (state) => {
      state.thought = "I should look this up";
      return NodeResult.advance(state);
    })
    .add_node("act", async (state) => {
      state.action = "search";
      return NodeResult.exit(state);
    })
    .add_edge("think", "act")
    .set_entry("think")
    .set_exit("act")
    .build();

  const result = await graph.invoke(new DynState({ query: "What is Orchustr?" }));
  assert.equal(result.thought, "I should look this up");
  assert.equal(result.action, "search");
}

function testCoreOrchestratorEnforcesBudget() {
  new CoreOrchestrator().enforceCompletionBudget(new TokenBudget(100, 20), 70);
}

async function testPipelineBuilderExecutesSequentialNodes() {
  const pipeline = new PipelineBuilder()
    .addNode("one", async (state) => ({ ...state, a: 1 }))
    .addNode("two", async (state) => ({ ...state, b: state.a + 1 }))
    .build();
  const result = await pipeline.invoke({});
  assert.equal(result.b, 2);
}

function testRustCrateBridgeCatalogIsOptional() {
  const catalog = RustCrateBridge.catalog();
  assert.ok(Array.isArray(catalog));
}

await testGraphBuilderExecutesAsyncHandlers();
await testGraphBuilderSupportsNodeResult();
await testPipelineBuilderExecutesSequentialNodes();
testPromptBuilderRendersVariables();
testPromptBuilderSanitizesControlCharacters();
testCoreOrchestratorEnforcesBudget();
testRustCrateBridgeCatalogIsOptional();
