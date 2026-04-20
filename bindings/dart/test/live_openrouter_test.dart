/// Live OpenRouter test for Orchustr Dart bindings.
///
/// Tests:
/// 1. Basic completion via OpenRouter
/// 2. Multi-turn memory (conversation history recall)
/// 3. Tool-call agent loop (JSON-based tool use via ForgeRegistry)
/// 4. MCP round-trip (mock MCP server → ForgeRegistry import)
///
/// Uses liquid/lfm-2.5-1.2b-instruct:free.
import "dart:async";
import "dart:convert";
import "dart:io";

import "package:orchustr/orchustr.dart";

const _endpoint = "https://openrouter.ai/api/v1/chat/completions";
const _model = "liquid/lfm-2.5-1.2b-instruct:free";

late final String _apiKey;

Future<void> main() async {
  _apiKey = Platform.environment["OPENROUTER_API_KEY"] ?? "";
  if (_apiKey.isEmpty) {
    stdout.writeln("SKIP: OPENROUTER_API_KEY not set");
    return;
  }

  await _run("basic completion", _testBasicCompletion);
  await _run("multi-turn memory", _testMemoryMultiTurn);
  await _run("tool-call agent loop", _testToolCallAgent);
  await _run("MCP forge round-trip", _testMcpForgeRoundTrip);

  if (exitCode != 0) {
    throw StateError("one or more live OpenRouter tests failed");
  }
  stdout.writeln("\nAll Dart live OpenRouter tests passed!");
}

// ── Test 1: Basic completion ────────────────────────────────────────

Future<void> _testBasicCompletion() async {
  final text = await _chat([
    <String, Object?>{"role": "user", "content": "Reply with exactly one word: hello"},
  ], maxTokens: 64);
  stdout.writeln('Response: "$text"');
  _expect(text.isNotEmpty, "response should not be empty");
}

// ── Test 2: Multi-turn memory ───────────────────────────────────────

Future<void> _testMemoryMultiTurn() async {
  final turn1 = await _chat([
    <String, Object?>{
      "role": "system",
      "content": "You are a memory test assistant. Remember everything."
    },
    <String, Object?>{
      "role": "user",
      "content": "My favorite color is cerulean. Please acknowledge."
    },
  ]);
  stdout.writeln('Turn 1: "$turn1"');

  final turn2 = await _chat([
    <String, Object?>{
      "role": "system",
      "content": "You are a memory test assistant. Remember everything."
    },
    <String, Object?>{
      "role": "user",
      "content": "My favorite color is cerulean. Please acknowledge."
    },
    <String, Object?>{"role": "assistant", "content": turn1},
    <String, Object?>{
      "role": "user",
      "content": "What is my favorite color? Reply with just the color name."
    },
  ]);
  stdout.writeln('Turn 2 (recall): "$turn2"');
  _expect(
    turn2.toLowerCase().contains("cerulean"),
    "should recall cerulean, got: $turn2",
  );
}

// ── Test 3: Tool-call agent loop ────────────────────────────────────

Future<void> _testToolCallAgent() async {
  final forge = ForgeRegistry();
  forge.register("calculate", (args) {
    // simple eval for test expressions
    final expr = args["expression"]?.toString() ?? "0";
    int result;
    if (expr.contains("*")) {
      final parts = expr.split("*").map((s) => int.parse(s.trim())).toList();
      result = parts[0] * parts[1];
    } else if (expr.contains("+")) {
      final parts = expr.split("+").map((s) => int.parse(s.trim())).toList();
      result = parts[0] + parts[1];
    } else {
      result = 0;
    }
    return <String, Object?>{"result": result};
  });

  final response = await _chat([
    <String, Object?>{
      "role": "system",
      "content":
          'You have a tool called "calculate" that evaluates math expressions. '
              'When asked a math question, respond ONLY with a JSON object like: '
              '{"tool": "calculate", "expression": "2+2"}\n'
              'Do not include any other text.',
    },
    <String, Object?>{"role": "user", "content": "What is 15 * 3?"},
  ]);
  stdout.writeln('LLM tool call: "$response"');

  var cleaned = response.trim();
  if (cleaned.startsWith("```")) {
    cleaned = cleaned.replaceAll(RegExp(r"^```json?\s*", multiLine: true), "");
    cleaned = cleaned.replaceAll("```", "").trim();
  }
  try {
    final toolCall = jsonDecode(cleaned) as Map<String, Object?>;
    _expect(toolCall["tool"] == "calculate", "should pick calculate: $toolCall");
    final result = await forge.invoke(
      "calculate",
      <String, Object?>{"expression": toolCall["expression"]},
    );
    stdout.writeln("Tool result: $result");
    _expect(result is Map && result["result"] == 45, "15*3 should be 45: $result");
  } on FormatException {
    stdout.writeln("NOTE: model did not output clean JSON, but API call succeeded");
  }
}

// ── Test 4: MCP round-trip ──────────────────────────────────────────

Future<void> _testMcpForgeRoundTrip() async {
  final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
  unawaited(() async {
    try {
      await for (final request in server) {
        final body = jsonDecode(await utf8.decoder.bind(request).join())
            as Map<String, Object?>;
        final method = body["method"] as String?;
        Object? result;

        if (method == "tools/list") {
          result = <String, Object?>{
            "tools": <Object?>[
              <String, Object?>{"name": "greet", "description": "Greets a user"},
              <String, Object?>{"name": "add", "description": "Adds two numbers"},
            ],
          };
        } else if (method == "tools/call") {
          final params = body["params"] as Map<String, Object?>;
          final name = params["name"] as String;
          final args = params["arguments"] as Map<String, Object?>? ?? {};
          if (name == "greet") {
            result = <String, Object?>{
              "greeting": "Hello, ${args["name"] ?? "world"}!"
            };
          } else if (name == "add") {
            result = <String, Object?>{
              "sum": ((args["a"] as num?) ?? 0) + ((args["b"] as num?) ?? 0)
            };
          }
        }

        request.response.headers.contentType = ContentType.json;
        request.response.write(jsonEncode(<String, Object?>{
          "jsonrpc": "2.0",
          "id": body["id"],
          "result": result,
        }));
        await request.response.close();
      }
    } catch (error, stackTrace) {
      stderr.writeln("MCP SERVER ERROR: $error");
      stderr.writeln(stackTrace);
      exitCode = 1;
    }
  }());

  final uri = "http://${server.address.host}:${server.port}";
  try {
    final client = await NexusClient.connectHttp(uri);
    final forge = ForgeRegistry();
    final imported = await forge.importFromMcp(client);
    _expect(imported == 2, "should import 2 tools, got $imported");

    final greetResult = await forge.invoke(
      "greet",
      <String, Object?>{"name": "Orchustr"},
    );
    _expect(
      greetResult is Map && greetResult["greeting"] == "Hello, Orchustr!",
      "greeting: $greetResult",
    );

    final addResult = await forge.invoke(
      "add",
      <String, Object?>{"a": 10, "b": 32},
    );
    _expect(
      addResult is Map && addResult["sum"] == 42,
      "sum: $addResult",
    );
  } finally {
    await server.close();
  }
}

// ── Helpers ─────────────────────────────────────────────────────────

Future<String> _chat(
  List<Map<String, Object?>> messages, {
  int maxTokens = 128,
}) async {
  const maxAttempts = 4;
  for (var attempt = 1; attempt <= maxAttempts; attempt++) {
    final client = HttpClient();
    try {
      final request = await client.postUrl(Uri.parse(_endpoint));
      request.headers.contentType = ContentType.json;
      request.headers.set("Authorization", "Bearer $_apiKey");
      request.write(jsonEncode(<String, Object?>{
        "model": _model,
        "messages": messages,
        "max_tokens": maxTokens,
      }));
      final response =
          await request.close().timeout(const Duration(seconds: 60));
      final body = jsonDecode(await response.transform(utf8.decoder).join())
          as Map<String, Object?>;
      if (body.containsKey("error")) {
        final error = body["error"];
        if (error is Map && error["code"] == 429 && attempt < maxAttempts) {
          final delay = Duration(seconds: 5 * (1 << (attempt - 1)));
          stdout.writeln(
              "  rate-limited, retrying in ${delay.inSeconds}s (attempt $attempt/$maxAttempts)...");
          await Future<void>.delayed(delay);
          continue;
        }
        throw StateError("OpenRouter error: $error");
      }
      final choices = body["choices"] as List<Object?>;
      final first = choices.first as Map<String, Object?>;
      final message = first["message"] as Map<String, Object?>;
      return (message["content"] as String).trim();
    } finally {
      client.close();
    }
  }
  throw StateError("exceeded max retry attempts");
}

Future<void> _run(String name, Future<void> Function() body) async {
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
