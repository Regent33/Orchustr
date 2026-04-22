export class PromptBuilder {
  template(value: string): PromptBuilder;
  build(): { render(context: Record<string, unknown>): string };
}

export class DynState {
  constructor(initial?: Record<string, unknown>);
  toObject(): Record<string, unknown>;
}

export class NodeResult<T extends Record<string, unknown> = Record<string, unknown>> {
  constructor(
    state: T | DynState,
    kind: string,
    next?: string | null,
    checkpointId?: string | null,
  );
  state: DynState;
  kind: string;
  next: string | null;
  checkpointId: string | null;
  static advance<T extends Record<string, unknown>>(state: T | DynState): NodeResult<T>;
  static exit<T extends Record<string, unknown>>(state: T | DynState): NodeResult<T>;
  static branch<T extends Record<string, unknown>>(state: T | DynState, next: string): NodeResult<T>;
  static pause<T extends Record<string, unknown>>(checkpointId: string, state: T | DynState): NodeResult<T>;
}

export class GraphBuilder<T extends Record<string, unknown>> {
  addNode(
    name: string,
    handler: (state: T | DynState) => Promise<T | NodeResult<T>> | T | NodeResult<T>,
  ): GraphBuilder<T>;
  add_node(
    name: string,
    handler: (state: T | DynState) => Promise<T | NodeResult<T>> | T | NodeResult<T>,
  ): GraphBuilder<T>;
  addEdge(source: string, target: string): GraphBuilder<T>;
  add_edge(source: string, target: string): GraphBuilder<T>;
  setEntry(name: string): GraphBuilder<T>;
  set_entry(name: string): GraphBuilder<T>;
  setExit(name: string): GraphBuilder<T>;
  set_exit(name: string): GraphBuilder<T>;
  build(): {
    execute(state: T | DynState): Promise<DynState>;
    invoke(state: T | DynState): Promise<DynState>;
  };
}

export class ForgeRegistry {
  register(
    name: string,
    handler: (args: Record<string, unknown>) => Promise<unknown> | unknown,
  ): void;
  importFromMcp(client: NexusClient): Promise<number>;
  invoke(name: string, args: Record<string, unknown>): Promise<unknown>;
}

export class NexusClient {
  static connectHttp(endpoint: string): Promise<NexusClient>;
  send(method: string, params: Record<string, unknown>): Promise<any>;
  listTools(): Promise<Array<{ name: string }>>;
  invokeTool(name: string, args: Record<string, unknown>): Promise<any>;
}

export class ConduitProvider {
  completeText(prompt: string): Promise<{ text: string }>;
  completeMessages(messages: Array<Record<string, unknown>>): Promise<{ text: string }>;
  streamText(prompt: string): AsyncIterable<string>;
}

export class OpenAiConduit extends ConduitProvider {
  static fromEnv(): OpenAiConduit;
}

export class AnthropicConduit extends ConduitProvider {
  static fromEnv(): AnthropicConduit;
}

export class OpenAiCompatConduit extends ConduitProvider {
  constructor(apiKey: string, model: string, endpoint: string);
  static openrouter(apiKey: string, model: string): OpenAiCompatConduit;
  static groq(apiKey: string, model: string): OpenAiCompatConduit;
  static together(apiKey: string, model: string): OpenAiCompatConduit;
  static fireworks(apiKey: string, model: string): OpenAiCompatConduit;
  static deepseek(apiKey: string, model: string): OpenAiCompatConduit;
  static mistral(apiKey: string, model: string): OpenAiCompatConduit;
  static xai(apiKey: string, model: string): OpenAiCompatConduit;
  static nvidia(apiKey: string, model: string): OpenAiCompatConduit;
  static ollama(model: string, endpoint?: string): OpenAiCompatConduit;
}

export interface CrateBinding {
  crate_name: string;
  binding_mode: string;
  description: string;
  operations: string[];
}

export class RustCrateBridge {
  static available(): boolean;
  static catalog(): CrateBinding[];
  static invoke(
    crateName: string,
    operation: string,
    payload?: Record<string, unknown>,
  ): any;
}

export class SearchTools {
  static search(
    provider: string,
    query: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
}

export class WebTools {
  static fetch(
    provider: string,
    request: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static scrape(
    provider: string,
    url: string,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
}

export class VectorTools {
  static ensureCollection(
    provider: string,
    data: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static upsert(
    provider: string,
    data: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static delete(
    provider: string,
    data: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static query(
    provider: string,
    data: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
}

export class LoaderTools {
  static load(request: Record<string, unknown>): Promise<any> | any;
}

export class ExecTools {
  static execute(
    request: Record<string, unknown>,
    providers?: string[],
    config?: Record<string, unknown>,
  ): Promise<any> | any;
}

export class FileTools {
  static read(
    path: string,
    provider?: string,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static write(
    path: string,
    content: string,
    provider?: string,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static list(
    path: string,
    provider?: string,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static delete(
    path: string,
    provider?: string,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static fetch(
    provider: string,
    query: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
}

export class CommsTools {
  static send(
    provider: string,
    to: string,
    body: string,
    from?: string | null,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
}

export class ProductivityTools {
  static listEmails(
    provider: string,
    query?: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static sendEmail(
    provider: string,
    item: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static listEvents(
    provider: string,
    query?: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static createEvent(
    provider: string,
    item: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static listIssues(
    provider: string,
    query?: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static createIssue(
    provider: string,
    item: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static searchPages(
    provider: string,
    query?: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static createPage(
    provider: string,
    item: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static postMessage(
    provider: string,
    channel: string,
    text: string,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
  static searchMessages(
    provider: string,
    query?: Record<string, unknown>,
    config?: Record<string, unknown>,
  ): Promise<any> | any;
}

export class TokenBudget {
  constructor(maxContextTokens: number, maxCompletionTokens: number);
  fits(promptTokens: number, completionTokens: number): boolean;
}

export class RetryPolicy {
  constructor(
    maxAttempts: number,
    baseDelayMs: number,
    maxDelayMs: number,
    jitter?: boolean,
  );
}

export class CoreOrchestrator {
  enforceCompletionBudget(budget: TokenBudget, promptTokens: number): void;
  nextRetryDelay(policy: RetryPolicy, attempt: number): number;
}

export class PrismConfig {
  constructor(otlpEndpoint: string, serviceName?: string);
}

export function installGlobalSubscriber(otlpEndpoint: string): unknown;

export class PlainText {
  text: string;
}

export class TextParser {
  parse(raw: string): PlainText;
}

export class CheckpointGate {
  pause(
    checkpointId: string,
    resumeFrom: string,
    state: Record<string, unknown>,
  ): Promise<void>;
  resume(
    checkpointId: string,
  ): Promise<{ checkpointId: string; resumeFrom: string; state: Record<string, unknown> }>;
}

export class RecallEntry {
  constructor(kind: string, value: Record<string, unknown>);
  kind: string;
  value: Record<string, unknown>;
}

export class RecallStore {
  store(entry: RecallEntry): Promise<void>;
  list(kind: string): Promise<RecallEntry[]>;
}

export class RecallOrchestrator {
  remember(store: RecallStore, entry: RecallEntry): Promise<void>;
  recall(store: RecallStore, kind: string): Promise<RecallEntry[]>;
}

export class CompassRouterBuilder<T extends Record<string, unknown>> {
  addRoute(name: string, predicate: (state: T) => boolean): CompassRouterBuilder<T>;
  setDefault(route: string): CompassRouterBuilder<T>;
  build(): { select(state: T): { route: string } };
}

export class PipelineBuilder<T extends Record<string, unknown>> {
  addNode(name: string, handler: (state: T) => Promise<T> | T): PipelineBuilder<T>;
  add_node(name: string, handler: (state: T) => Promise<T> | T): PipelineBuilder<T>;
  build(): { execute(state: T): Promise<T>; invoke(state: T): Promise<T> };
}

export class RelayBuilder<T extends Record<string, unknown>> {
  addBranch(name: string, handler: (state: T) => Promise<T> | T): RelayBuilder<T>;
  add_branch(name: string, handler: (state: T) => Promise<T> | T): RelayBuilder<T>;
  build(): { branches: Array<[string, (state: T) => Promise<T> | T]> };
}

export class RelayExecutor<T extends Record<string, unknown>> {
  execute(
    plan: { branches: Array<[string, (state: T) => Promise<T> | T]> },
    initialState: T,
  ): Promise<T>;
}

export class ColonyOrchestrator<T extends Record<string, unknown>> {
  addMember(
    name: string,
    role: string,
    agent: (
      state: T,
      transcript: Array<{ from: string; to: string; content: string }>,
      member: { name: string; role: string },
    ) => Promise<unknown> | unknown,
  ): ColonyOrchestrator<T>;
  coordinate(
    initialState: T,
  ): Promise<{ summary: string; state: T; transcript: Array<{ from: string; to: string; content: string }> }>;
}

export class ColonyBuilder<T extends Record<string, unknown>> {
  addMember(
    name: string,
    role: string,
    agent: (
      state: T,
      transcript: Array<{ from: string; to: string; content: string }>,
      member: { name: string; role: string },
    ) => Promise<unknown> | unknown,
  ): ColonyBuilder<T>;
  add_member(
    name: string,
    role: string,
    agent: (
      state: T,
      transcript: Array<{ from: string; to: string; content: string }>,
      member: { name: string; role: string },
    ) => Promise<unknown> | unknown,
  ): ColonyBuilder<T>;
  build(): ColonyOrchestrator<T>;
}

export class SentinelConfig {
  constructor(maxSteps?: number, metadata?: Record<string, unknown>);
}

export class StepOutcome<T extends Record<string, unknown>> {
  constructor(status: string, state: T, message?: string | null);
}

export class SentinelOrchestrator<T extends Record<string, unknown>> {
  runAgent(
    agent:
      | ((state: T, config: SentinelConfig) => Promise<T | StepOutcome<T> | unknown>)
      | ((state: T, config: SentinelConfig) => T | StepOutcome<T> | unknown),
    initialState: T,
    config: SentinelConfig,
  ): Promise<StepOutcome<T>>;
}
