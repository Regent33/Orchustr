import "dart:async";
import "dart:convert";
import "dart:io";

import "package:orchustr/orchustr.dart";
import "package:orchustr/src/native_bridge.dart";

Future<void> main() async {
  await _run("prompt builder renders variables", () {
    final template = PromptBuilder().template("Hello {{name}}").build();
    _expect(
        template.render(<String, Object?>{"name": "Ralph"}) == "Hello Ralph",
        "prompt render");
  });

  await _run("prompt builder sanitizes control characters", () {
    final template = PromptBuilder().template("Hello {{name}}").build();
    _expect(
        template.render(<String, Object?>{"name": "Ra\u0007lph"}) ==
            "Hello Ralph",
        "prompt sanitize");
  });

  await _run("graph builder executes async handlers", () async {
    final graph = GraphBuilder()
        .addNode("start",
            (state) async => <String, Object?>{...state, "text": "hello"})
        .addNode(
            "finish",
            (state) => <String, Object?>{
                  ...state,
                  "done": "${state["text"]}".toUpperCase()
                })
        .addEdge("start", "finish")
        .setEntry("start")
        .setExit("finish")
        .build();
    final result = await graph.execute(<String, Object?>{});
    _expect(result["done"] == "HELLO", "graph execute");
  });

  await _run("core orchestrator enforces budget", () {
    CoreOrchestrator()
        .enforceCompletionBudget(const TokenBudget(100, 20), 70);
  });

  await _run("pipeline builder executes sequential nodes", () async {
    final pipeline = PipelineBuilder()
        .addNode("one", (state) async => <String, Object?>{...state, "a": 1})
        .addNode("two", (state) => <String, Object?>{
              ...state,
              "b": (state["a"] as int) + 1,
            })
        .build();
    final result = await pipeline.execute(<String, Object?>{});
    _expect(result["b"] == 2, "pipeline execute");
  });

  await _run("forge registry imports MCP tools", () async {
    final server = await _startServer();
    try {
      final client = await NexusClient.connectHttp(server.uri.toString());
      final forge = ForgeRegistry();
      final imported = await forge.importFromMcp(client);
      _expect(imported == 1, "tool import count");
      final result =
          await forge.invoke("echo", <String, Object?>{"message": "hi"});
      _expect(result is Map && result["echo"] == "hi", "tool invoke");
    } finally {
      await server.server.close();
    }
  });

  await _run("conduits parse OpenAI and Anthropic responses", () async {
    final server = await _startServer();
    try {
      final openAi =
          OpenAiConduit("key", "model", endpoint: server.uri.resolve("openai"));
      final anthropic = AnthropicConduit("key", "model",
          endpoint: server.uri.resolve("anthropic"));
      final openAiResponse = await openAi.completeText("hello");
      final anthropicResponse = await anthropic.completeText("hello");
      _expect(openAiResponse.text == "openai-ok", "openai response");
      _expect(anthropicResponse.text == "anthropic-ok", "anthropic response");
    } finally {
      await server.server.close();
    }
  });

  await _run("native bridge is optional but usable when present", () {
    final bridge = OrchustrNativeBridge.instance;
    if (bridge == null) {
      return;
    }
    final normalized = bridge.normalizeStateJson("{\"count\":1}");
    _expect(normalized.contains("\"count\":1"), "native normalize");
  });

  await _run("crate bridge catalog is optional", () {
    final catalog = RustCrateBridge.catalog();
    _expect(catalog is List<CrateBinding>, "bridge catalog");
  });

  if (exitCode != 0) {
    throw StateError("one or more Dart binding tests failed");
  }
}

Future<void> _run(String name, FutureOr<void> Function() body) async {
  try {
    await body();
    stdout.writeln("PASS $name");
  } catch (error, stackTrace) {
    stderr.writeln("FAIL $name");
    stderr.writeln(error);
    stderr.writeln(stackTrace);
    exitCode = 1;
  }
}

void _expect(bool condition, String message) {
  if (!condition) {
    throw StateError(message);
  }
}

Future<({HttpServer server, Uri uri})> _startServer() async {
  final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
  unawaited(() async {
    try {
      await for (final request in server) {
        final body = jsonDecode(await utf8.decoder.bind(request).join())
            as Map<String, Object?>;
        final path = request.uri.path;
        final responseBody = switch (path) {
          "/openai" => <String, Object?>{
              "output": <Object?>[
                <String, Object?>{
                  "content": <Object?>[
                    <String, Object?>{"text": "openai-ok"}
                  ],
                },
              ],
            },
          "/anthropic" => <String, Object?>{
              "content": <Object?>[
                <String, Object?>{"text": "anthropic-ok"}
              ],
            },
          _ => _mcpResponse(body),
        };
        request.response.headers.contentType = ContentType.json;
        request.response.write(jsonEncode(responseBody));
        await request.response.close();
      }
    } catch (error, stackTrace) {
      stderr.writeln("TEST SERVER ERROR: $error");
      stderr.writeln(stackTrace);
      exitCode = 1;
    }
  }());
  return (
    server: server,
    uri: Uri.parse("http://${server.address.host}:${server.port}/"),
  );
}

Map<String, Object?> _mcpResponse(Map<String, Object?> body) {
  return switch (body["method"]) {
    "tools/list" => <String, Object?>{
        "jsonrpc": "2.0",
        "id": body["id"],
        "result": <String, Object?>{
          "tools": <Object?>[
            <String, Object?>{"name": "echo"}
          ],
        },
      },
    "tools/call" => <String, Object?>{
        "jsonrpc": "2.0",
        "id": body["id"],
        "result": <String, Object?>{
          "echo": ((body["params"] as Map<String, Object?>)["arguments"]
              as Map<String, Object?>)["message"],
        },
      },
    _ => <String, Object?>{
        "jsonrpc": "2.0",
        "id": body["id"],
        "result": const <String, Object?>{},
      },
  };
}
