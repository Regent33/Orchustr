import assert from "node:assert/strict";

import {
  AnthropicConduit,
  CoreOrchestrator,
  DynState,
  GraphBuilder,
  NodeResult,
  OpenAiCompatConduit,
  OpenAiConduit,
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

// ── SSE streaming tests (regression for audit #24) ─────────────────

/**
 * Builds a Response-like object whose body streams the given chunks
 * one at a time. Mirrors what `fetch` returns for a streaming SSE
 * endpoint without needing a real network call.
 */
function makeSseResponse(chunks) {
  const encoder = new TextEncoder();
  const queue = chunks.map((chunk) => encoder.encode(chunk));
  return {
    ok: true,
    status: 200,
    body: {
      getReader() {
        return {
          async read() {
            if (queue.length === 0) return { done: true, value: undefined };
            return { done: false, value: queue.shift() };
          },
        };
      },
    },
    async text() {
      return "";
    },
  };
}

async function withStubFetch(stub, fn) {
  const original = globalThis.fetch;
  globalThis.fetch = stub;
  try {
    return await fn();
  } finally {
    globalThis.fetch = original;
  }
}

async function collect(iter) {
  const out = [];
  for await (const value of iter) out.push(value);
  return out;
}

async function testOpenAiCompatStreamsDeltas() {
  const stub = async () =>
    makeSseResponse([
      'data: {"choices":[{"delta":{"content":"Hello"}}]}\n\n',
      'data: {"choices":[{"delta":{"content":" world"}}]}\n\n',
      "data: [DONE]\n\n",
    ]);
  const conduit = new OpenAiCompatConduit("k", "m", "https://example.invalid/x");
  const tokens = await withStubFetch(stub, () => collect(conduit.streamText("hi")));
  assert.deepEqual(tokens, ["Hello", " world"]);
}

async function testAnthropicStreamsDeltas() {
  const stub = async () =>
    makeSseResponse([
      'event: message_start\ndata: {"type":"message_start"}\n\n',
      'event: content_block_delta\ndata: {"type":"content_block_delta","delta":{"type":"text_delta","text":"Hi "}}\n\n',
      'event: content_block_delta\ndata: {"type":"content_block_delta","delta":{"type":"text_delta","text":"there"}}\n\n',
      'event: message_stop\ndata: {"type":"message_stop"}\n\n',
    ]);
  const conduit = new AnthropicConduit("k", "claude-x");
  const tokens = await withStubFetch(stub, () => collect(conduit.streamText("hi")));
  assert.deepEqual(tokens, ["Hi ", "there"]);
}

async function testOpenAiResponsesApiStreamsDeltas() {
  const stub = async () =>
    makeSseResponse([
      'event: response.output_text.delta\ndata: {"type":"response.output_text.delta","delta":"alpha"}\n\n',
      'event: response.output_text.delta\ndata: {"type":"response.output_text.delta","delta":"beta"}\n\n',
      "data: [DONE]\n\n",
    ]);
  const conduit = new OpenAiConduit("k", "m");
  const tokens = await withStubFetch(stub, () => collect(conduit.streamText("hi")));
  assert.deepEqual(tokens, ["alpha", "beta"]);
}

await testGraphBuilderExecutesAsyncHandlers();
await testGraphBuilderSupportsNodeResult();
await testPipelineBuilderExecutesSequentialNodes();
testPromptBuilderRendersVariables();
testPromptBuilderSanitizesControlCharacters();
testCoreOrchestratorEnforcesBudget();
testRustCrateBridgeCatalogIsOptional();
await testOpenAiCompatStreamsDeltas();
await testAnthropicStreamsDeltas();
await testOpenAiResponsesApiStreamsDeltas();
console.log("All TypeScript bindings tests passed!");
