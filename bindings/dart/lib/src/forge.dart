import "dart:async";

import "mcp.dart";
import "types.dart";

typedef ForgeToolHandler = FutureOr<Object?> Function(JsonObject args);

final class ForgeRegistry {
  final Map<String, ForgeToolHandler> _tools = <String, ForgeToolHandler>{};

  void register(String name, ForgeToolHandler handler) {
    _tools[name] = handler;
  }

  Future<int> importFromMcp(NexusClient client) async {
    for (final tool in await client.listTools()) {
      final toolName = "${tool["name"]}";
      _tools[toolName] = (JsonObject args) => client.invokeTool(toolName, args);
    }
    return _tools.length;
  }

  Future<Object?> invoke(String name, JsonObject args) async {
    final handler = _tools[name];
    if (handler == null) {
      throw StateError("unknown tool: $name");
    }
    return await Future<Object?>.value(handler(args));
  }
}
