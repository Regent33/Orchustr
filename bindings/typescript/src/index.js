function sanitize(value) {
  return Array.from(value)
    .filter((character) => character >= " " || character === "\n" || character === "\t")
    .join("");
}

function isPromiseLike(value) {
  return value !== null && typeof value === "object" && typeof value.then === "function";
}

async function maybeAwait(value) {
  return isPromiseLike(value) ? await value : value;
}

export class DynState {
  constructor(initial = {}) {
    Object.assign(this, initial);
  }

  toObject() {
    return { ...this };
  }
}

export class NodeResult {
  constructor(state, kind, next = null, checkpointId = null) {
    this.state = state instanceof DynState ? new DynState(state) : new DynState(state ?? {});
    this.kind = kind;
    this.next = next;
    this.checkpointId = checkpointId;
  }

  static advance(state) {
    return new NodeResult(state, "advance");
  }

  static exit(state) {
    return new NodeResult(state, "exit");
  }

  static branch(state, next) {
    return new NodeResult(state, "branch", next);
  }

  static pause(checkpointId, state) {
    return new NodeResult(state, "pause", null, checkpointId);
  }
}

function coerceState(state) {
  return state instanceof DynState ? new DynState(state) : new DynState(state ?? {});
}

function coerceNodeResult(result, current, exit) {
  if (result instanceof NodeResult) return result;
  const state = coerceState(result);
  return current === exit ? NodeResult.exit(state) : NodeResult.advance(state);
}

export class PromptBuilder {
  constructor() {
    this._template = null;
  }

  template(value) {
    this._template = value; // preserve template as-is; sanitize values at render time
    return this;
  }

  build() {
    if (!this._template) throw new Error("template must not be empty");
    const template = this._template;
    return {
      render(context) {
        let rendered = template;
        for (const [, variable] of template.matchAll(/{{([A-Za-z0-9_]+)}}/g)) {
          if (!(variable in context)) throw new Error(`missing variable: ${variable}`);
          rendered = rendered.replaceAll(`{{${variable}}}`, sanitize(String(context[variable])));
        }
        return rendered;
      },
    };
  }
}

export class GraphBuilder {
  constructor() {
    this.nodes = new Map();
    this.edges = new Map();
    this.entry = null;
    this.exit = null;
  }

  addNode(name, handler) {
    this.nodes.set(name, handler);
    return this;
  }

  add_node(name, handler) {
    return this.addNode(name, handler);
  }

  addEdge(source, target) {
    this.edges.set(source, [...(this.edges.get(source) ?? []), target]);
    return this;
  }

  add_edge(source, target) {
    return this.addEdge(source, target);
  }

  setEntry(name) {
    this.entry = name;
    return this;
  }

  set_entry(name) {
    return this.setEntry(name);
  }

  setExit(name) {
    this.exit = name;
    return this;
  }

  set_exit(name) {
    return this.setExit(name);
  }

  build() {
    const nodes = this.nodes;
    const edges = this.edges;
    const entry = this.entry;
    const exit = this.exit;
    if (!entry || !exit) throw new Error("graph requires entry and exit nodes");
    return {
      async execute(initialState) {
        return this.invoke(initialState);
      },
      async invoke(initialState) {
        let current = entry;
        let state = coerceState(initialState);
        for (let index = 0; index < 1024; index += 1) {
          const handler = nodes.get(current);
          if (!handler) throw new Error(`unknown node: ${current}`);
          const raw = await maybeAwait(handler(coerceState(state)));
          const result = coerceNodeResult(raw, current, exit);
          state = coerceState(result.state);
          if (result.kind === "exit") return state;
          if (result.kind === "advance" && current === exit) return state;
          if (result.kind === "pause") {
            throw new Error(`graph paused at checkpoint ${result.checkpointId ?? "<unknown>"}`);
          }
          if (result.kind === "branch") {
            if (!result.next) throw new Error(`node ${current} returned branch without a next node`);
            current = result.next;
            continue;
          }
          const targets = edges.get(current) ?? [];
          if (targets.length !== 1) throw new Error(`node ${current} requires one edge`);
          current = targets[0];
        }
        throw new Error("graph exceeded execution limit");
      },
    };
  }
}

export class ConduitProvider {
  async completeText(_prompt) {
    throw new Error("completeText must be implemented by a conduit provider");
  }

  async completeMessages(_messages) {
    throw new Error("completeMessages must be implemented by a conduit provider");
  }

  async *streamText(prompt) {
    const response = await this.completeText(prompt);
    yield response.text;
  }
}

export class ForgeRegistry {
  constructor() {
    this.tools = new Map();
  }

  register(name, handler) {
    this.tools.set(name, handler);
  }

  async importFromMcp(client) {
    for (const tool of await client.listTools()) {
      // Capture client reference explicitly to guard against later mutation
      const boundClient = client;
      const boundName = tool.name;
      this.tools.set(tool.name, (args) => boundClient.invokeTool(boundName, args));
    }
    return this.tools.size;
  }

  async invoke(name, args) {
    if (!this.tools.has(name)) throw new Error(`unknown tool: ${name}`);
    return await this.tools.get(name)(args);
  }
}

export class NexusClient {
  constructor(endpoint) {
    this.endpoint = endpoint;
    this.nextId = 1;
  }

  static async connectHttp(endpoint) {
    return new NexusClient(endpoint);
  }

  async send(method, params) {
    const response = await fetch(this.endpoint, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ jsonrpc: "2.0", id: this.nextId++, method, params }),
    });
    const body = await response.json();
    return body.result;
  }

  async listTools() {
    return (await this.send("tools/list", {})).tools ?? [];
  }

  async invokeTool(name, args) {
    return await this.send("tools/call", { name, arguments: args });
  }
}

// ── Server-Sent Events (SSE) parsing ────────────────────────────────

/**
 * Parses a Server-Sent Events stream from a `fetch` Response body and
 * yields one `{ event, data }` object per terminated event. The `event`
 * field is the most recent `event:` line (or `"message"` if absent),
 * and `data` is the concatenation of all `data:` lines for that event.
 *
 * Stops yielding when the stream ends. Throws if `response.body` is
 * absent (non-streaming environments — fall back to `completeText`).
 */
async function* _sseEvents(response) {
  if (!response.body) {
    throw new Error("response has no body — streaming is not supported in this environment");
  }
  const decoder = new TextDecoder("utf-8");
  const reader = response.body.getReader();
  let buffer = "";
  while (true) {
    const { value, done } = await reader.read();
    if (done) break;
    buffer += decoder.decode(value, { stream: true });
    let separatorIdx;
    // SSE events are terminated by a blank line (\n\n or \r\n\r\n).
    while ((separatorIdx = _findEventBoundary(buffer)) !== -1) {
      const rawEvent = buffer.slice(0, separatorIdx.start);
      buffer = buffer.slice(separatorIdx.end);
      const parsed = _parseSseEvent(rawEvent);
      if (parsed) yield parsed;
    }
  }
  // Flush any trailing event without a terminating blank line.
  if (buffer.trim().length > 0) {
    const parsed = _parseSseEvent(buffer);
    if (parsed) yield parsed;
  }
}

function _findEventBoundary(buffer) {
  const lf = buffer.indexOf("\n\n");
  const crlf = buffer.indexOf("\r\n\r\n");
  if (lf === -1 && crlf === -1) return -1;
  if (crlf !== -1 && (lf === -1 || crlf < lf)) {
    return { start: crlf, end: crlf + 4 };
  }
  return { start: lf, end: lf + 2 };
}

function _parseSseEvent(raw) {
  let event = "message";
  const dataLines = [];
  for (const line of raw.split(/\r?\n/)) {
    if (!line || line.startsWith(":")) continue;
    const colon = line.indexOf(":");
    if (colon === -1) continue;
    const field = line.slice(0, colon);
    // Per spec, a single space after the colon is part of the delimiter.
    const value = line.slice(colon + 1).replace(/^ /, "");
    if (field === "event") event = value;
    else if (field === "data") dataLines.push(value);
  }
  if (dataLines.length === 0) return null;
  return { event, data: dataLines.join("\n") };
}

// ── Response text extraction ────────────────────────────────────────

/**
 * Extract text from OpenAI Responses API (output), OpenAI Chat Completions
 * API (choices), or Anthropic Messages API (content) response bodies.
 */
function _extractText(body) {
  if (Array.isArray(body.choices) && body.choices.length > 0) {
    const content = body.choices[0]?.message?.content;
    if (typeof content === "string") return content;
  }
  if (body.output) {
    return body.output
      .flatMap((block) => block.content ?? [])
      .filter((item) => typeof item === "object" && item !== null)
      .map((item) => item.text ?? "")
      .join("");
  }
  if (body.content) {
    return body.content
      .filter((item) => typeof item === "object" && item !== null)
      .map((item) => item.text ?? "")
      .join("");
  }
  return "";
}

// ── Real LLM conduit implementations (Bugs 9-10 fix) ───────────────

export class OpenAiConduit extends ConduitProvider {
  static fromEnv() {
    return new OpenAiConduit(process.env.OPENAI_API_KEY, process.env.OPENAI_MODEL);
  }

  constructor(apiKey, model) {
    super();
    this.apiKey = apiKey;
    this.model = model;
    // Uses the OpenAI Responses API (not Chat Completions).
    // Schema: input=[...], response has output=[{content:[{text:...}]}]
    this.endpoint = "https://api.openai.com/v1/responses";
  }

  async completeText(prompt) {
    return this.completeMessages([
      { role: "user", content: [{ type: "text", text: prompt }] },
    ]);
  }

  async completeMessages(messages) {
    const response = await fetch(this.endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${this.apiKey}`,
      },
      body: JSON.stringify({
        model: this.model,
        input: messages,
        max_output_tokens: 1024,
      }),
    });
    if (!response.ok) {
      const errorBody = await response.text();
      throw new Error(`OpenAI API error: ${response.status} ${errorBody}`);
    }
    const body = await response.json();
    return { text: _extractText(body) };
  }

  /**
   * Streams the response token-by-token using the OpenAI Responses API
   * SSE protocol. Yields each text delta as it arrives. If the upstream
   * does not return an SSE body (e.g. unusual environments without
   * ReadableStream), falls back to a single non-streaming chunk.
   */
  async *streamText(prompt) {
    const response = await fetch(this.endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${this.apiKey}`,
        Accept: "text/event-stream",
      },
      body: JSON.stringify({
        model: this.model,
        input: [{ role: "user", content: [{ type: "text", text: prompt }] }],
        max_output_tokens: 1024,
        stream: true,
      }),
    });
    if (!response.ok) {
      const errorBody = await response.text();
      throw new Error(`OpenAI API error: ${response.status} ${errorBody}`);
    }
    if (!response.body) {
      // Non-streaming environment fallback.
      const fallback = await this.completeText(prompt);
      yield fallback.text;
      return;
    }
    for await (const { event, data } of _sseEvents(response)) {
      if (data === "[DONE]") return;
      // The Responses API emits multiple event types; we want only the
      // text-delta events. Unknown / non-text events are skipped.
      if (event && event !== "response.output_text.delta") continue;
      try {
        const payload = JSON.parse(data);
        if (typeof payload.delta === "string") yield payload.delta;
      } catch {
        // Skip malformed event payloads rather than fail the stream.
      }
    }
  }
}

export class AnthropicConduit extends ConduitProvider {
  static fromEnv() {
    return new AnthropicConduit(
      process.env.ANTHROPIC_API_KEY,
      process.env.ANTHROPIC_MODEL,
    );
  }

  constructor(apiKey, model) {
    super();
    this.apiKey = apiKey;
    this.model = model;
    this.endpoint = "https://api.anthropic.com/v1/messages";
  }

  async completeText(prompt) {
    return this.completeMessages([
      { role: "user", content: [{ type: "text", text: prompt }] },
    ]);
  }

  async completeMessages(messages) {
    const response = await fetch(this.endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "x-api-key": this.apiKey,
        "anthropic-version": "2023-06-01",
      },
      body: JSON.stringify({
        model: this.model,
        messages,
        max_tokens: 1024,
      }),
    });
    if (!response.ok) {
      const errorBody = await response.text();
      throw new Error(`Anthropic API error: ${response.status} ${errorBody}`);
    }
    const body = await response.json();
    return { text: _extractText(body) };
  }

  /**
   * Streams the response token-by-token using the Anthropic Messages
   * API SSE protocol. Yields each `text_delta` chunk as it arrives.
   * Falls back to a single non-streaming chunk if the upstream does
   * not return an SSE body.
   */
  async *streamText(prompt) {
    const response = await fetch(this.endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "x-api-key": this.apiKey,
        "anthropic-version": "2023-06-01",
        Accept: "text/event-stream",
      },
      body: JSON.stringify({
        model: this.model,
        messages: [{ role: "user", content: [{ type: "text", text: prompt }] }],
        max_tokens: 1024,
        stream: true,
      }),
    });
    if (!response.ok) {
      const errorBody = await response.text();
      throw new Error(`Anthropic API error: ${response.status} ${errorBody}`);
    }
    if (!response.body) {
      const fallback = await this.completeText(prompt);
      yield fallback.text;
      return;
    }
    for await (const { event, data } of _sseEvents(response)) {
      if (event !== "content_block_delta") continue;
      try {
        const payload = JSON.parse(data);
        if (payload?.delta?.type === "text_delta" && typeof payload.delta.text === "string") {
          yield payload.delta.text;
        }
      } catch {
        // Skip malformed event payloads.
      }
    }
  }
}

const _OPENAI_COMPAT_ENDPOINTS = {
  openai: "https://api.openai.com/v1/chat/completions",
  openrouter: "https://openrouter.ai/api/v1/chat/completions",
  together: "https://api.together.xyz/v1/chat/completions",
  groq: "https://api.groq.com/openai/v1/chat/completions",
  fireworks: "https://api.fireworks.ai/inference/v1/chat/completions",
  deepseek: "https://api.deepseek.com/v1/chat/completions",
  mistral: "https://api.mistral.ai/v1/chat/completions",
  xai: "https://api.x.ai/v1/chat/completions",
  nvidia: "https://integrate.api.nvidia.com/v1/chat/completions",
  ollama: "http://localhost:11434/v1/chat/completions",
};

/**
 * Generic OpenAI-compatible conduit for providers that speak the Chat Completions API.
 * Use the static factory methods (openrouter, groq, together, fireworks, deepseek,
 * mistral, xai, nvidia, ollama) or pass a custom endpoint directly.
 */
export class OpenAiCompatConduit extends ConduitProvider {
  constructor(apiKey, model, endpoint) {
    super();
    this.apiKey = apiKey;
    this.model = model;
    this.endpoint = endpoint;
  }

  static openrouter(apiKey, model) { return new OpenAiCompatConduit(apiKey, model, _OPENAI_COMPAT_ENDPOINTS.openrouter); }
  static groq(apiKey, model)       { return new OpenAiCompatConduit(apiKey, model, _OPENAI_COMPAT_ENDPOINTS.groq); }
  static together(apiKey, model)   { return new OpenAiCompatConduit(apiKey, model, _OPENAI_COMPAT_ENDPOINTS.together); }
  static fireworks(apiKey, model)  { return new OpenAiCompatConduit(apiKey, model, _OPENAI_COMPAT_ENDPOINTS.fireworks); }
  static deepseek(apiKey, model)   { return new OpenAiCompatConduit(apiKey, model, _OPENAI_COMPAT_ENDPOINTS.deepseek); }
  static mistral(apiKey, model)    { return new OpenAiCompatConduit(apiKey, model, _OPENAI_COMPAT_ENDPOINTS.mistral); }
  static xai(apiKey, model)        { return new OpenAiCompatConduit(apiKey, model, _OPENAI_COMPAT_ENDPOINTS.xai); }
  static nvidia(apiKey, model)     { return new OpenAiCompatConduit(apiKey, model, _OPENAI_COMPAT_ENDPOINTS.nvidia); }
  static ollama(model, endpoint)   { return new OpenAiCompatConduit("", model, endpoint ?? _OPENAI_COMPAT_ENDPOINTS.ollama); }

  async completeText(prompt) {
    return this.completeMessages([
      { role: "user", content: [{ type: "text", text: prompt }] },
    ]);
  }

  async completeMessages(messages) {
    const response = await fetch(this.endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${this.apiKey}`,
      },
      body: JSON.stringify({
        model: this.model,
        messages,
        max_tokens: 1024,
      }),
    });
    if (!response.ok) {
      const errorBody = await response.text();
      throw new Error(`OpenAI-compat API error: ${response.status} ${errorBody}`);
    }
    const body = await response.json();
    return { text: _extractText(body) };
  }

  /**
   * Streams the response token-by-token using the OpenAI Chat
   * Completions SSE protocol (the canonical wire format for every
   * compat provider listed in `_OPENAI_COMPAT_ENDPOINTS`). Yields
   * each `choices[0].delta.content` chunk as it arrives. Falls back
   * to a single non-streaming chunk if the upstream lacks an SSE body.
   */
  async *streamText(prompt) {
    const response = await fetch(this.endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${this.apiKey}`,
        Accept: "text/event-stream",
      },
      body: JSON.stringify({
        model: this.model,
        messages: [{ role: "user", content: [{ type: "text", text: prompt }] }],
        max_tokens: 1024,
        stream: true,
      }),
    });
    if (!response.ok) {
      const errorBody = await response.text();
      throw new Error(`OpenAI-compat API error: ${response.status} ${errorBody}`);
    }
    if (!response.body) {
      const fallback = await this.completeText(prompt);
      yield fallback.text;
      return;
    }
    for await (const { data } of _sseEvents(response)) {
      if (data === "[DONE]") return;
      try {
        const payload = JSON.parse(data);
        const content = payload?.choices?.[0]?.delta?.content;
        if (typeof content === "string" && content.length > 0) {
          yield content;
        }
      } catch {
        // Skip malformed event payloads.
      }
    }
  }
}

export { RustCrateBridge } from "./bridge.js";
export {
  CommsTools,
  ExecTools,
  FileTools,
  LoaderTools,
  ProductivityTools,
  SearchTools,
  VectorTools,
  WebTools,
} from "./tools.js";
export {
  CheckpointGate,
  ColonyBuilder,
  ColonyOrchestrator,
  CompassRouterBuilder,
  CoreOrchestrator,
  PipelineBuilder,
  PlainText,
  PrismConfig,
  RecallEntry,
  RecallOrchestrator,
  RecallStore,
  RelayBuilder,
  RelayExecutor,
  RetryPolicy,
  SentinelConfig,
  SentinelOrchestrator,
  StepOutcome,
  TextParser,
  TokenBudget,
  installGlobalSubscriber,
} from "./workflows.js";
