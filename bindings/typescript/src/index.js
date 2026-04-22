function sanitize(value) {
  return Array.from(value)
    .filter((character) => character >= " " || character === "\n" || character === "\t")
    .join("");
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

  addEdge(source, target) {
    this.edges.set(source, [...(this.edges.get(source) ?? []), target]);
    return this;
  }

  setEntry(name) {
    this.entry = name;
    return this;
  }

  setExit(name) {
    this.exit = name;
    return this;
  }

  build() {
    const nodes = this.nodes;
    const edges = this.edges;
    const entry = this.entry;
    const exit = this.exit;
    if (!entry || !exit) throw new Error("graph requires entry and exit nodes");
    return {
      async execute(initialState) {
        let current = entry;
        let state = { ...initialState };
        for (let index = 0; index < 1024; index += 1) {
          state = { ...(await nodes.get(current)({ ...state })) };
          if (current === exit) return state;
          const targets = edges.get(current) ?? [];
          if (targets.length !== 1) throw new Error(`node ${current} requires one edge`);
          current = targets[0];
        }
        throw new Error("graph exceeded execution limit");
      },
    };
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

export class OpenAiConduit {
  static fromEnv() {
    return new OpenAiConduit(process.env.OPENAI_API_KEY, process.env.OPENAI_MODEL);
  }

  constructor(apiKey, model) {
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
   * Non-streaming fallback — yields the full response as one chunk.
   * Override for true SSE streaming.
   */
  async *streamText(prompt) {
    const result = await this.completeText(prompt);
    yield result.text;
  }
}

export class AnthropicConduit {
  static fromEnv() {
    return new AnthropicConduit(
      process.env.ANTHROPIC_API_KEY,
      process.env.ANTHROPIC_MODEL,
    );
  }

  constructor(apiKey, model) {
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
   * Non-streaming fallback — yields the full response as one chunk.
   * Override for true SSE streaming.
   */
  async *streamText(prompt) {
    const result = await this.completeText(prompt);
    yield result.text;
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
export class OpenAiCompatConduit {
  constructor(apiKey, model, endpoint) {
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

  async *streamText(prompt) {
    const result = await this.completeText(prompt);
    yield result.text;
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
