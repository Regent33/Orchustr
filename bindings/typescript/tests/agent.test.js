/**
 * Agent test for the Orchustr TypeScript bindings.
 *
 * Simulates a ReAct-style agent loop using:
 * - PromptBuilder (template rendering)
 * - GraphBuilder  (state-machine graph)
 * - ForgeRegistry (tool invocation)
 *
 * Validates fixes for bugs 9-10 (real conduit impls) and 11 (type alignment).
 */
import assert from "node:assert/strict";

import { ForgeRegistry, GraphBuilder, PromptBuilder } from "../src/index.js";

// ── Prompt Builder Tests ────────────────────────────────────────────

function testPromptTemplatePreservesSpecialChars() {
  // Bug fix: template string should NOT be sanitized, only values
  const template = new PromptBuilder().template("Hello\t{{name}}").build();
  const rendered = template.render({ name: "Agent" });
  assert.equal(rendered, "Hello\tAgent", "tab in template should survive");

  // Values should still be sanitized
  const rendered2 = template.render({ name: "Ra\u0007lph" });
  assert.equal(rendered2, "Hello\tRalph", "control chars in value should be stripped");
  console.log("PASS prompt_template_preserves_special_chars");
}

// ── Forge Registry Agent Tool Tests ─────────────────────────────────

async function testForgeToolInvocation() {
  const forge = new ForgeRegistry();

  // Register mock tools
  forge.register("search", async (args) => ({
    results: [`result for ${args.query}`],
  }));
  forge.register("calculate", async (args) => ({
    answer: Function(`"use strict"; return (${args.expr})`)(),
  }));

  // Invoke tools
  const searchResult = await forge.invoke("search", { query: "weather" });
  assert.deepEqual(searchResult, { results: ["result for weather"] });

  const calcResult = await forge.invoke("calculate", { expr: "2+2" });
  assert.deepEqual(calcResult, { answer: 4 });

  // Unknown tool should throw
  await assert.rejects(() => forge.invoke("unknown", {}), /unknown tool/);
  console.log("PASS forge_tool_invocation");
}

// ── ReAct Agent Loop Test ───────────────────────────────────────────

async function testReactAgentLoop() {
  const forge = new ForgeRegistry();
  forge.register("calculate", async (args) => ({
    answer: Function(`"use strict"; return (${args.expr})`)(),
  }));

  const graph = new GraphBuilder()
    .addNode("plan", async (state) => {
      if ((state.iteration ?? 0) < 1) {
        return {
          ...state,
          action: "use_tool",
          toolName: "calculate",
          toolArgs: { expr: "2+2" },
        };
      }
      return {
        ...state,
        action: "answer",
        finalAnswer: `The result is ${state.toolResult?.answer ?? "?"}`,
      };
    })
    .addNode("act", async (state) => {
      if (state.action === "use_tool") {
        const result = await forge.invoke(state.toolName, state.toolArgs);
        return { ...state, toolResult: result };
      }
      return state;
    })
    .addNode("observe", async (state) => {
      const iteration = (state.iteration ?? 0) + 1;
      if (state.action === "answer") {
        return { ...state, iteration, _next: "finish" };
      }
      return { ...state, iteration, _next: "plan" };
    })
    .addNode("finish", async (state) => state)
    .addEdge("plan", "act")
    .addEdge("act", "observe")
    .addEdge("observe", "finish")
    .setEntry("plan")
    .setExit("finish")
    .build();

  // Run with a manual loop (the graph only supports single-path edges,
  // so we simulate the loop externally)
  let state = { task: "What is 2+2?", iteration: 0 };
  for (let i = 0; i < 10; i++) {
    state = await graph.execute(state);
    if (state._next === "finish" || state.action === "answer") break;
  }

  assert.ok(state.finalAnswer, "agent should produce a final answer");
  assert.ok(
    state.finalAnswer.includes("4"),
    `answer should contain 4, got: ${state.finalAnswer}`,
  );
  assert.equal(state.task, "What is 2+2?", "original task should survive");
  console.log("PASS react_agent_loop");
}

// ── Pipeline Agent Test ─────────────────────────────────────────────

async function testPipelineAgent() {
  const graph = new GraphBuilder()
    .addNode("classify", async (state) => ({
      ...state,
      intent: state.input.includes("weather") ? "weather" : "general",
    }))
    .addNode("fetch", async (state) => ({
      ...state,
      context: `fetched data for intent=${state.intent}`,
    }))
    .addNode("generate", async (state) => ({
      ...state,
      response: `Answer based on: ${state.context}`,
    }))
    .addEdge("classify", "fetch")
    .addEdge("fetch", "generate")
    .setEntry("classify")
    .setExit("generate")
    .build();

  const result = await graph.execute({ input: "What's the weather?" });

  assert.equal(result.intent, "weather");
  assert.ok(result.context.includes("weather"));
  assert.ok(result.response.includes("weather"));
  assert.equal(result.input, "What's the weather?", "input should survive");
  console.log("PASS pipeline_agent");
}

// ── Run all tests ───────────────────────────────────────────────────

testPromptTemplatePreservesSpecialChars();
await testForgeToolInvocation();
await testReactAgentLoop();
await testPipelineAgent();
console.log("\nAll TypeScript agent tests passed!");
