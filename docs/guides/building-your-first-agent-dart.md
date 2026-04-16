# Building Your First Agent in Dart/Flutter

This guide walks you through building a basic agent using the `orchustr` Dart bindings, connecting to the native Rust engine.

## Minimal Shape

```dart
import "package:orchustr/orchustr.dart";

Future<void> main() async {
  // Define our agent's cognitive graph
  final graph = GraphBuilder()
      .addNode("think", (state) async {
        return <String, Object?>{
          ...state, 
          "context": "Gathering necessary memories...",
          "stepsleft": (state["stepsleft"] as int) - 1
        };
      })
      .addNode("act", (state) async {
        return <String, Object?>{
          ...state,
          "result": "Invoking requested actions..."
        };
      })
      .addEdge("think", "act")
      .setEntry("think")
      .setExit("act")
      .build();

  // Initialize the native graph orchestrator with state
  final result = await graph.execute(<String, Object?>{
    "query": "What is the status of the server?",
    "stepsleft": 3,
  });

  print("Agent Completed Context: ${result["context"]}");
  print("Agent Completed Result: ${result["result"]}");
}
```

## Integrating Providers and MCP Tools

To connect your graph to large language models or external tools, you will initialize the `OpenAiConduit` or `AnthropicConduit` and load a `ForgeRegistry`:

```dart
import "package:orchustr/orchustr.dart";

Future<void> main() async {
  // Load remote MCP tools to give the agent filesystem and shell access
  final mcpClient = await NexusClient.connectHttp("http://localhost:8080/");
  final registry = ForgeRegistry();
  await registry.importFromMcp(mcpClient);

  // Initialize LLM provider from the environment
  final conduit = OpenAiConduit.fromEnv();
  
  // Here you would inject 'conduit' and 'registry' inside your graph nodes
  // to invoke models and trigger tools based on the current state.
}
```

## Security Notes

- Cross-language bindings currently map to native FFI bridges under the hood. Avoid sending overly massive blobs of text across the graph boundaries without streaming.
- Tools imported from external MCP servers expose real privileges. Review your MCP server configuration manually.
