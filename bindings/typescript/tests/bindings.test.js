import assert from "node:assert/strict";

import { GraphBuilder, PromptBuilder } from "../src/index.js";

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

await testGraphBuilderExecutesAsyncHandlers();
testPromptBuilderRendersVariables();
testPromptBuilderSanitizesControlCharacters();
