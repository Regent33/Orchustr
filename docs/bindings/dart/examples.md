# Dart Examples

## Render a Prompt

```dart
import "package:orchustr/orchustr.dart";

void main() {
  final template = PromptBuilder().template("Hello {{name}}").build();
  print(template.render(<String, Object?>{"name": "Ralph"}));
}
```

## Execute a Graph

```dart
import "package:orchustr/orchustr.dart";

Future<void> main() async {
  final graph = GraphBuilder()
      .addNode("start", (state) async => <String, Object?>{...state, "text": "hello"})
      .addNode("finish", (state) => <String, Object?>{...state, "done": "${state["text"]}".toUpperCase()})
      .addEdge("start", "finish")
      .setEntry("start")
      .setExit("finish")
      .build();

  final result = await graph.execute(<String, Object?>{});
  print(result["done"]);
}
```

## Import MCP Tools

```dart
import "package:orchustr/orchustr.dart";

Future<void> main() async {
  final client = await NexusClient.connectHttp("http://localhost:8080/");
  final forge = ForgeRegistry();
  await forge.importFromMcp(client);
}
```

⚠️ Known Gaps & Limitations
- The examples use the current binding-layer implementations, which are lighter-weight than the Rust crate internals.
- A real MCP server or model endpoint is still required for end-to-end networked examples.
