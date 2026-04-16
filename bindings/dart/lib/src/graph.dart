import "dart:async";

import "types.dart";

typedef GraphNode = FutureOr<JsonObject> Function(JsonObject state);

final class ExecutionGraph {
  const ExecutionGraph(this._nodes, this._edges, this._entry, this._exit);

  final Map<String, GraphNode> _nodes;
  final Map<String, List<String>> _edges;
  final String _entry;
  final String _exit;

  Future<JsonObject> execute(JsonObject state) async {
    var current = _entry;
    var data = copyJsonObject(state);
    for (var index = 0; index < 1024; index += 1) {
      data = copyJsonObject(await _nodes[current]!(copyJsonObject(data)));
      if (current == _exit) {
        return data;
      }
      final targets = _edges[current] ?? const <String>[];
      if (targets.length != 1) {
        throw StateError("node $current requires exactly one default edge");
      }
      current = targets.single;
    }
    throw StateError("graph exceeded execution limit");
  }
}

final class GraphBuilder {
  final Map<String, GraphNode> _nodes = <String, GraphNode>{};
  final Map<String, List<String>> _edges = <String, List<String>>{};
  String? _entry;
  String? _exit;

  GraphBuilder addNode(String name, GraphNode handler) {
    _nodes[name] = handler;
    return this;
  }

  GraphBuilder addEdge(String source, String target) {
    _edges.putIfAbsent(source, () => <String>[]).add(target);
    return this;
  }

  GraphBuilder setEntry(String name) {
    _entry = name;
    return this;
  }

  GraphBuilder setExit(String name) {
    _exit = name;
    return this;
  }

  ExecutionGraph build() {
    final entry = _entry;
    final exit = _exit;
    if (_nodes.isEmpty || entry == null || exit == null) {
      throw StateError("graph requires nodes, entry, and exit");
    }
    return ExecutionGraph(_nodes, _edges, entry, exit);
  }
}
