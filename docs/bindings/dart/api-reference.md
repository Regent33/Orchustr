# Dart API Reference

## `PromptBuilder`

- `template(String value) -> PromptBuilder`
- `build() -> PromptTemplate`

## `PromptTemplate`

- `render(Map<String, Object?> context) -> String`

## `GraphBuilder`

- `addNode(String name, FutureOr<Map<String, Object?>> Function(Map<String, Object?>) handler) -> GraphBuilder`
- `addEdge(String source, String target) -> GraphBuilder`
- `setEntry(String name) -> GraphBuilder`
- `setExit(String name) -> GraphBuilder`
- `build() -> ExecutionGraph`

## `ExecutionGraph`

- `execute(Map<String, Object?> state) -> Future<Map<String, Object?>>`

## `ForgeRegistry`

- `register(String name, FutureOr<Object?> Function(Map<String, Object?>) handler) -> void`
- `importFromMcp(NexusClient client) -> Future<int>`
- `invoke(String name, Map<String, Object?> args) -> Future<Object?>`

## `NexusClient`

- `connectHttp(String endpoint) -> Future<NexusClient>`
- `send(String method, Map<String, Object?> params) -> Future<Object?>`
- `listTools() -> Future<List<Map<String, Object?>>>`
- `invokeTool(String name, Map<String, Object?> args) -> Future<Object?>`

## `OpenAiConduit`

- `OpenAiConduit(String apiKey, String model, {Uri? endpoint})`
- `OpenAiConduit.fromEnv()`
- `completeMessages(List<Map<String, Object?>> messages) -> Future<CompletionResponse>`
- `completeText(String prompt) -> Future<CompletionResponse>`
- `streamText(String prompt) -> Stream<String>`

## `AnthropicConduit`

- `AnthropicConduit(String apiKey, String model, {Uri? endpoint})`
- `AnthropicConduit.fromEnv()`

## Native Bridge Helpers

- `configureNativeBridge({String? libraryPath}) -> void`
- `nativeBridgeAvailable -> bool`
- `RustCrateBridge.available -> bool`
- `RustCrateBridge.catalog() -> List<CrateBinding>`
- `RustCrateBridge.invoke(String crateName, String operation, JsonObject payload) -> Object?`

## Tool Helpers

- `SearchTools`
- `WebTools`
- `VectorTools`
- `LoaderTools`
- `ExecTools`
- `FileTools`
- `CommsTools`
- `ProductivityTools`

## Workflow Helpers

- `TokenBudget`
- `RetryPolicy`
- `CoreOrchestrator`
- `CheckpointGate`
- `RecallStore`
- `RelayExecutor`
- `SentinelOrchestrator`
- `TextParser`
- `installGlobalSubscriber(String otlpEndpoint) -> void`

⚠️ Known Gaps & Limitations

- The Dart API mirrors the crate graph at a binding level, not as a raw export of every Rust item.
- Structured model types are still mostly JSON-like maps in the binding layer.
