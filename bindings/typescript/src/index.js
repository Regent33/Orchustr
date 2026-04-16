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
    this._template = sanitize(value);
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
      this.tools.set(tool.name, (args) => client.invokeTool(tool.name, args));
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

export class OpenAiConduit {
  static fromEnv() {
    return new OpenAiConduit(process.env.OPENAI_API_KEY, process.env.OPENAI_MODEL);
  }

  constructor(apiKey, model) {
    this.apiKey = apiKey;
    this.model = model;
  }

  async completeText(prompt) {
    return { text: prompt };
  }

  async *streamText(prompt) {
    for (const chunk of prompt.split(/\s+/).filter(Boolean)) yield chunk;
  }
}

export class AnthropicConduit extends OpenAiConduit {
  static fromEnv() {
    return new AnthropicConduit(process.env.ANTHROPIC_API_KEY, process.env.ANTHROPIC_MODEL);
  }
}
