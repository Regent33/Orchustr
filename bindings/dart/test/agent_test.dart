/// Agent test for the Orchustr Dart bindings.
///
/// Simulates a ReAct-style agent loop using:
/// - PromptBuilder (Bug 8-equivalent: template rendering)
/// - GraphBuilder  (state-machine graph execution)
/// - ForgeRegistry (tool invocation via MCP mock)
/// - Conduit       (HTTP response parsing, Bug 14: shared HttpClient)
///
/// Also validates Bug 15 fix (server loop error surfacing).
import "dart:async";
import "dart:convert";
import "dart:io";

import "package:orchustr/orchustr.dart";

Future<void> main() async {
  await _run("prompt template preserves special characters", () {
    // Template should NOT be sanitized — only values are sanitized
    final template = PromptBuilder().template("Hello\t{{name}}").build();
    final rendered = template.render(<String, Object?>{"name": "Agent"});
    _expect(rendered == "Hello\tAgent", "tab in template should survive: $rendered");

    // But injected values should still be sanitized
    final rendered2 =
        template.render(<String, Object?>{"name": "Ra\u0007lph"});
    _expect(rendered2 == "Hello\tRalph",
        "control chars in value should be stripped: $rendered2");
  });

  await _run("forge tool invocation with MCP mock", () async {
    final server = await _startMcpServer();
    try {
      final client = await NexusClient.connectHttp(server.uri.toString());
      final forge = ForgeRegistry();
      final imported = await forge.importFromMcp(client);
      _expect(imported == 2, "expected 2 tools, got $imported");

      final searchResult = await forge.invoke(
          "search", <String, Object?>{"query": "weather"});
      _expect(
          searchResult is Map && searchResult["query"] == "weather",
          "search result: $searchResult");

      final calcResult = await forge.invoke(
          "calculate", <String, Object?>{"expr": "2+2"});
      _expect(
          calcResult is Map && calcResult["expr"] == "2+2",
          "calc result: $calcResult");
    } finally {
      await server.server.close();
    }
  });

  await _run("react agent loop with graph", () async {
    var iteration = 0;

    final graph = GraphBuilder()
        .addNode("plan", (state) async {
          if (iteration < 1) {
            return <String, Object?>{
              ...state,
              "action": "use_tool",
              "tool_name": "calculate",
              "tool_args": <String, Object?>{"expr": "2+2"},
            };
          }
          return <String, Object?>{
            ...state,
            "action": "answer",
            "final_answer":
                "The result is ${state["tool_result"] ?? "?"}",
          };
        })
        .addNode("act", (state) async {
          if (state["action"] == "use_tool") {
            return <String, Object?>{
              ...state,
              "tool_result": "4",
            };
          }
          return state;
        })
        .addNode("observe", (state) async {
          iteration++;
          if (state["action"] == "answer") {
            return <String, Object?>{...state, "done": true};
          }
          return state;
        })
        .addNode("finish", (state) async => state)
        .addEdge("plan", "act")
        .addEdge("act", "observe")
        .addEdge("observe", "finish")
        .setEntry("plan")
        .setExit("finish")
        .build();

    // Run the loop manually (graph is single-path per execution)
    var state = <String, Object?>{"task": "What is 2+2?"};
    for (var i = 0; i < 10; i++) {
      state = await graph.execute(state);
      if (state["done"] == true || state["action"] == "answer") break;
    }

    _expect(state["final_answer"] != null, "should produce final_answer");
    _expect(
        state["final_answer"].toString().contains("4"),
        "answer should contain 4: ${state["final_answer"]}");
    _expect(state["task"] == "What is 2+2?",
        "original task should survive");
  });

  await _run("pipeline agent enriches state", () async {
    final graph = GraphBuilder()
        .addNode("classify", (state) async {
          final input = state["input"]?.toString() ?? "";
          return <String, Object?>{
            ...state,
            "intent": input.contains("weather") ? "weather" : "general",
          };
        })
        .addNode("fetch", (state) async {
          return <String, Object?>{
            ...state,
            "context": "fetched data for intent=${state["intent"]}",
          };
        })
        .addNode("generate", (state) async {
          return <String, Object?>{
            ...state,
            "response": "Answer based on: ${state["context"]}",
          };
        })
        .addEdge("classify", "fetch")
        .addEdge("fetch", "generate")
        .setEntry("classify")
        .setExit("generate")
        .build();

    final result = await graph
        .execute(<String, Object?>{"input": "What's the weather?"});

    _expect(result["intent"] == "weather", "intent: ${result["intent"]}");
    _expect(result["context"].toString().contains("weather"),
        "context: ${result["context"]}");
    _expect(result["response"].toString().contains("weather"),
        "response: ${result["response"]}");
    _expect(result["input"] == "What's the weather?",
        "input should survive: ${result["input"]}");
  });

  await _run("conduit parses OpenAI and Anthropic responses via shared client",
      () async {
    final server = await _startConduitServer();
    try {
      final openAi =
          OpenAiConduit("key", "model", endpoint: server.uri.resolve("openai"));
      final anthropic = AnthropicConduit("key", "model",
          endpoint: server.uri.resolve("anthropic"));

      // Test conduit (uses shared HttpClient — Bug 14 fix)
      final r1 = await openAi.completeText("hello");
      final r2 = await anthropic.completeText("hello");
      final r3 = await openAi.completeText("second call"); // reuses connection

      _expect(r1.text == "openai-ok", "openai: ${r1.text}");
      _expect(r2.text == "anthropic-ok", "anthropic: ${r2.text}");
      _expect(r3.text == "openai-ok", "openai reuse: ${r3.text}");
    } finally {
      await server.server.close();
    }
  });

  if (exitCode != 0) {
    throw StateError("one or more Dart agent tests failed");
  }
  stdout.writeln("\nAll Dart agent tests passed!");
}

// ── Test helpers ────────────────────────────────────────────────────

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
  if (!condition) throw StateError(message);
}

// ── Mock MCP Server ─────────────────────────────────────────────────

Future<({HttpServer server, Uri uri})> _startMcpServer() async {
  final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
  unawaited(() async {
    try {
      await for (final request in server) {
        final body = jsonDecode(await utf8.decoder.bind(request).join())
            as Map<String, Object?>;
        final responseBody = switch (body["method"]) {
          "tools/list" => <String, Object?>{
              "jsonrpc": "2.0",
              "id": body["id"],
              "result": <String, Object?>{
                "tools": <Object?>[
                  <String, Object?>{"name": "search"},
                  <String, Object?>{"name": "calculate"},
                ],
              },
            },
          "tools/call" => <String, Object?>{
              "jsonrpc": "2.0",
              "id": body["id"],
              "result": (body["params"] as Map<String, Object?>)["arguments"],
            },
          _ => <String, Object?>{
              "jsonrpc": "2.0",
              "id": body["id"],
              "result": const <String, Object?>{},
            },
        };
        request.response.headers.contentType = ContentType.json;
        request.response.write(jsonEncode(responseBody));
        await request.response.close();
      }
    } catch (error, stackTrace) {
      stderr.writeln("MCP SERVER ERROR: $error");
      stderr.writeln(stackTrace);
      exitCode = 1;
    }
  }());
  return (
    server: server,
    uri: Uri.parse("http://${server.address.host}:${server.port}/"),
  );
}

// ── Mock Conduit Server ─────────────────────────────────────────────

Future<({HttpServer server, Uri uri})> _startConduitServer() async {
  final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
  unawaited(() async {
    try {
      await for (final request in server) {
        await utf8.decoder.bind(request).join(); // consume body
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
          _ => <String, Object?>{"content": <Object?>[]},
        };
        request.response.headers.contentType = ContentType.json;
        request.response.write(jsonEncode(responseBody));
        await request.response.close();
      }
    } catch (error, stackTrace) {
      stderr.writeln("CONDUIT SERVER ERROR: $error");
      stderr.writeln(stackTrace);
      exitCode = 1;
    }
  }());
  return (
    server: server,
    uri: Uri.parse("http://${server.address.host}:${server.port}/"),
  );
}
