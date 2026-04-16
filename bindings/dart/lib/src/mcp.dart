import "dart:convert";
import "dart:io";

import "types.dart";

final class NexusClient {
  NexusClient._(this._endpoint);

  final Uri _endpoint;
  int _nextId = 1;

  static Future<NexusClient> connectHttp(String endpoint) async {
    return NexusClient._(Uri.parse(endpoint));
  }

  Future<Object?> send(String method, JsonObject params) async {
    final client = HttpClient()
      ..connectionTimeout = const Duration(seconds: 30);
    try {
      final request = await client.postUrl(_endpoint);
      request.headers.contentType = ContentType.json;
      request.write(
        jsonEncode(
          <String, Object?>{
            "jsonrpc": "2.0",
            "id": _nextId++,
            "method": method,
            "params": params,
          },
        ),
      );
      final response =
          await request.close().timeout(const Duration(seconds: 30));
      final body = jsonDecode(await response.transform(utf8.decoder).join());
      if (body is! JsonObject) {
        throw StateError("MCP response must be a JSON object");
      }
      if (body["error"] != null) {
        throw StateError("MCP request failed: ${jsonEncode(body["error"])}");
      }
      return body["result"];
    } finally {
      client.close(force: true);
    }
  }

  Future<List<JsonObject>> listTools() async {
    final result = await send("tools/list", const <String, Object?>{});
    if (result is! JsonObject || result["tools"] is! List<Object?>) {
      return const <JsonObject>[];
    }
    return (result["tools"]! as List<Object?>)
        .whereType<JsonObject>()
        .map(copyJsonObject)
        .toList(growable: false);
  }

  Future<Object?> invokeTool(String name, JsonObject args) {
    return send(
      "tools/call",
      <String, Object?>{"name": name, "arguments": copyJsonObject(args)},
    );
  }
}
