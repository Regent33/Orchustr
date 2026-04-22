import { RustCrateBridge } from "./bridge.js";

function mergeState(left, right) {
  return { ...left, ...right };
}

export class TokenBudget {
  constructor(maxContextTokens, maxCompletionTokens) {
    this.maxContextTokens = maxContextTokens;
    this.maxCompletionTokens = maxCompletionTokens;
  }

  fits(promptTokens, completionTokens) {
    return promptTokens + completionTokens <= this.maxContextTokens;
  }
}

export class RetryPolicy {
  constructor(maxAttempts, baseDelayMs, maxDelayMs, jitter = true) {
    this.maxAttempts = maxAttempts;
    this.baseDelayMs = baseDelayMs;
    this.maxDelayMs = maxDelayMs;
    this.jitter = jitter;
  }
}

export class CoreOrchestrator {
  enforceCompletionBudget(budget, promptTokens) {
    if (!budget.fits(promptTokens, budget.maxCompletionTokens)) {
      throw new Error("budget exceeded");
    }
  }

  nextRetryDelay(policy, attempt) {
    if (attempt <= 0 || attempt > policy.maxAttempts) {
      throw new Error(`invalid retry attempt: ${attempt}`);
    }
    const base = Math.min(
      policy.baseDelayMs * (2 ** (attempt - 1)),
      policy.maxDelayMs,
    );
    if (!policy.jitter || base === 0) return base;
    return Math.floor(Math.random() * (base + 1));
  }
}

export class PrismConfig {
  constructor(otlpEndpoint, serviceName = "orchustr-typescript") {
    this.otlpEndpoint = otlpEndpoint;
    this.serviceName = serviceName;
  }
}

export function installGlobalSubscriber(otlpEndpoint) {
  return RustCrateBridge.invoke("or-prism", "install_global_subscriber", {
    otlp_endpoint: otlpEndpoint,
  });
}

export class PlainText {
  constructor(text) {
    this.text = text;
  }
}

export class TextParser {
  parse(raw) {
    const text = raw.trim();
    if (!text) throw new Error("text must not be empty");
    return new PlainText(text);
  }
}

export class CheckpointGate {
  constructor() {
    this.records = new Map();
  }

  async pause(checkpointId, resumeFrom, state) {
    this.records.set(checkpointId, {
      checkpointId,
      resumeFrom,
      state: { ...state },
    });
  }

  async resume(checkpointId) {
    if (!this.records.has(checkpointId)) {
      throw new Error(`unknown checkpoint: ${checkpointId}`);
    }
    return this.records.get(checkpointId);
  }
}

export class RecallEntry {
  constructor(kind, value) {
    this.kind = kind;
    this.value = value;
  }
}

export class RecallStore {
  constructor() {
    this.entries = [];
  }

  async store(entry) {
    this.entries.push(entry);
  }

  async list(kind) {
    return this.entries.filter((entry) => entry.kind === kind);
  }
}

export class RecallOrchestrator {
  async remember(store, entry) {
    await store.store(entry);
  }

  async recall(store, kind) {
    return await store.list(kind);
  }
}

export class CompassRouterBuilder {
  constructor() {
    this.routes = [];
    this.defaultRoute = null;
  }

  addRoute(name, predicate) {
    this.routes.push([name, predicate]);
    return this;
  }

  setDefault(route) {
    this.defaultRoute = route;
    return this;
  }

  build() {
    const routes = [...this.routes];
    const defaultRoute = this.defaultRoute;
    return {
      select(state) {
        for (const [name, predicate] of routes) {
          if (predicate(state)) return { route: name };
        }
        if (defaultRoute) return { route: defaultRoute };
        throw new Error("no matching route");
      },
    };
  }
}

export class PipelineBuilder {
  constructor() {
    this.nodes = [];
  }

  addNode(name, handler) {
    this.nodes.push([name, handler]);
    return this;
  }

  add_node(name, handler) {
    return this.addNode(name, handler);
  }

  build() {
    if (this.nodes.length === 0) {
      throw new Error("pipeline requires at least one node");
    }
    const nodes = [...this.nodes];
    return {
      async execute(initialState) {
        let state = { ...initialState };
        for (const [, handler] of nodes) {
          state = mergeState(state, { ...(await handler({ ...state })) });
        }
        return state;
      },
      async invoke(initialState) {
        return this.execute(initialState);
      },
    };
  }
}

export class RelayBuilder {
  constructor() {
    this.branches = [];
  }

  addBranch(name, handler) {
    this.branches.push([name, handler]);
    return this;
  }

  add_branch(name, handler) {
    return this.addBranch(name, handler);
  }

  build() {
    if (this.branches.length === 0) {
      throw new Error("relay requires at least one branch");
    }
    return { branches: [...this.branches] };
  }
}

export class RelayExecutor {
  async execute(plan, initialState) {
    const patches = await Promise.all(
      plan.branches.map(async ([name, handler]) => [
        name,
        { ...(await handler({ ...initialState })) },
      ]),
    );
    let state = { ...initialState };
    for (const [, patch] of patches.sort((a, b) => a[0].localeCompare(b[0]))) {
      state = mergeState(state, patch);
    }
    return state;
  }
}

export class ColonyOrchestrator {
  constructor() {
    this.members = [];
  }

  addMember(name, role, agent) {
    this.members.push([{ name, role }, agent]);
    return this;
  }

  async coordinate(initialState) {
    if (this.members.length === 0) {
      throw new Error("colony requires at least one member");
    }
    const transcript = [];
    let state = { ...initialState };
    for (const [member, agent] of this.members) {
      const response = await agent({ ...state }, [...transcript], member);
      const message = response && typeof response === "object" && "content" in response
        ? response
        : { from: member.name, to: "all", content: String(response) };
      transcript.push(message);
      state = { ...state, [member.name]: message.content };
    }
    return {
      summary: transcript.at(-1)?.content ?? "",
      state,
      transcript,
    };
  }
}

export class ColonyBuilder {
  constructor() {
    this.orchestrator = new ColonyOrchestrator();
  }

  addMember(name, role, agent) {
    this.orchestrator.addMember(name, role, agent);
    return this;
  }

  add_member(name, role, agent) {
    return this.addMember(name, role, agent);
  }

  build() {
    return this.orchestrator;
  }
}

export class SentinelConfig {
  constructor(maxSteps = 8, metadata = {}) {
    this.maxSteps = maxSteps;
    this.metadata = metadata;
  }
}

export class StepOutcome {
  constructor(status, state, message = null) {
    this.status = status;
    this.state = state;
    this.message = message;
  }
}

export class SentinelOrchestrator {
  async runAgent(agent, initialState, config) {
    const result = await agent({ ...initialState }, config);
    if (result instanceof StepOutcome) return result;
    if (typeof result === "object" && result !== null) {
      return new StepOutcome("completed", result);
    }
    return new StepOutcome("completed", { ...initialState }, String(result));
  }
}
