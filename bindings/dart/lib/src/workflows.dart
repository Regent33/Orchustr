import "dart:async";
import "dart:math";

import "bridge.dart";
import "types.dart";

JsonObject _merge(JsonObject left, JsonObject right) =>
    <String, Object?>{...left, ...right};

Future<JsonObject> _toJsonObject(FutureOr<JsonObject> value) async =>
    copyJsonObject(await Future.value(value));

final class TokenBudget {
  const TokenBudget(this.maxContextTokens, this.maxCompletionTokens);

  final int maxContextTokens;
  final int maxCompletionTokens;

  bool fits(int promptTokens, int completionTokens) {
    return promptTokens + completionTokens <= maxContextTokens;
  }
}

final class RetryPolicy {
  const RetryPolicy(this.maxAttempts, this.baseDelayMs, this.maxDelayMs,
      {this.jitter = true});

  final int maxAttempts;
  final int baseDelayMs;
  final int maxDelayMs;
  final bool jitter;
}

final class CoreOrchestrator {
  void enforceCompletionBudget(TokenBudget budget, int promptTokens) {
    if (!budget.fits(promptTokens, budget.maxCompletionTokens)) {
      throw StateError("budget exceeded");
    }
  }

  int nextRetryDelay(RetryPolicy policy, int attempt) {
    if (attempt <= 0 || attempt > policy.maxAttempts) {
      throw StateError("invalid retry attempt: $attempt");
    }
    final base =
        min(policy.baseDelayMs * (1 << (attempt - 1)), policy.maxDelayMs);
    if (!policy.jitter || base == 0) {
      return base;
    }
    return Random().nextInt(base + 1);
  }
}

final class PrismConfig {
  const PrismConfig(this.otlpEndpoint, {this.serviceName = "orchustr-dart"});

  final String otlpEndpoint;
  final String serviceName;
}

Object? installGlobalSubscriber(String otlpEndpoint) {
  return RustCrateBridge.invoke("or-prism", "install_global_subscriber",
      <String, Object?>{"otlp_endpoint": otlpEndpoint});
}

final class PlainText {
  const PlainText(this.text);

  final String text;
}

final class TextParser {
  PlainText parse(String raw) {
    final text = raw.trim();
    if (text.isEmpty) {
      throw StateError("text must not be empty");
    }
    return PlainText(text);
  }
}

final class CheckpointRecord {
  const CheckpointRecord(this.checkpointId, this.resumeFrom, this.state);

  final String checkpointId;
  final String resumeFrom;
  final JsonObject state;
}

final class CheckpointGate {
  final Map<String, CheckpointRecord> _records = <String, CheckpointRecord>{};

  Future<void> pause(
      String checkpointId, String resumeFrom, JsonObject state) async {
    _records[checkpointId] =
        CheckpointRecord(checkpointId, resumeFrom, copyJsonObject(state));
  }

  Future<CheckpointRecord> resume(String checkpointId) async {
    final record = _records[checkpointId];
    if (record == null) {
      throw StateError("unknown checkpoint: $checkpointId");
    }
    return record;
  }
}

final class RecallEntry {
  const RecallEntry(this.kind, this.value);

  final String kind;
  final JsonObject value;
}

final class RecallStore {
  final List<RecallEntry> _entries = <RecallEntry>[];

  Future<void> store(RecallEntry entry) async {
    _entries.add(entry);
  }

  Future<List<RecallEntry>> list(String kind) async {
    return _entries.where((RecallEntry entry) => entry.kind == kind).toList();
  }
}

final class RecallOrchestrator {
  Future<void> remember(RecallStore store, RecallEntry entry) {
    return store.store(entry);
  }

  Future<List<RecallEntry>> recall(RecallStore store, String kind) {
    return store.list(kind);
  }
}

typedef RoutePredicate = bool Function(JsonObject state);

final class CompassRouterBuilder {
  final List<(String, RoutePredicate)> _routes = <(String, RoutePredicate)>[];
  String? _defaultRoute;

  CompassRouterBuilder addRoute(String name, RoutePredicate predicate) {
    _routes.add((name, predicate));
    return this;
  }

  CompassRouterBuilder setDefault(String route) {
    _defaultRoute = route;
    return this;
  }

  CompassRouter build() => CompassRouter(List<(String, RoutePredicate)>.from(_routes), _defaultRoute);
}

final class CompassRouter {
  const CompassRouter(this._routes, this._defaultRoute);

  final List<(String, RoutePredicate)> _routes;
  final String? _defaultRoute;

  String select(JsonObject state) {
    for (final (String route, RoutePredicate predicate) in _routes) {
      if (predicate(state)) {
        return route;
      }
    }
    final fallback = _defaultRoute;
    if (fallback == null) {
      throw StateError("no matching route");
    }
    return fallback;
  }
}

typedef PipelineNode = FutureOr<JsonObject> Function(JsonObject state);

final class PipelineBuilder {
  final List<(String, PipelineNode)> _nodes = <(String, PipelineNode)>[];

  PipelineBuilder addNode(String name, PipelineNode handler) {
    _nodes.add((name, handler));
    return this;
  }

  Pipeline build() {
    if (_nodes.isEmpty) {
      throw StateError("pipeline requires at least one node");
    }
    return Pipeline(List<(String, PipelineNode)>.from(_nodes));
  }
}

final class Pipeline {
  const Pipeline(this._nodes);

  final List<(String, PipelineNode)> _nodes;

  Future<JsonObject> execute(JsonObject initialState) async {
    var state = copyJsonObject(initialState);
    for (final (_, PipelineNode handler) in _nodes) {
      state = _merge(state, await _toJsonObject(handler(copyJsonObject(state))));
    }
    return state;
  }
}

typedef RelayBranch = FutureOr<JsonObject> Function(JsonObject state);

final class RelayBuilder {
  final List<(String, RelayBranch)> _branches = <(String, RelayBranch)>[];

  RelayBuilder addBranch(String name, RelayBranch handler) {
    _branches.add((name, handler));
    return this;
  }

  RelayPlan build() {
    if (_branches.isEmpty) {
      throw StateError("relay requires at least one branch");
    }
    return RelayPlan(List<(String, RelayBranch)>.from(_branches));
  }
}

final class RelayPlan {
  const RelayPlan(this.branches);

  final List<(String, RelayBranch)> branches;
}

final class RelayExecutor {
  Future<JsonObject> execute(RelayPlan plan, JsonObject initialState) async {
    final patches = await Future.wait(plan.branches.map(((
      String,
      RelayBranch,
    ) branch) async {
      final (String name, RelayBranch handler) = branch;
      return (name, await _toJsonObject(handler(copyJsonObject(initialState))));
    }));
    patches.sort((left, right) => left.$1.compareTo(right.$1));
    var state = copyJsonObject(initialState);
    for (final (_, JsonObject patch) in patches) {
      state = _merge(state, patch);
    }
    return state;
  }
}

typedef ColonyAgent = FutureOr<Object?> Function(
  JsonObject state,
  List<ColonyMessage> transcript,
  ColonyMember member,
);

final class ColonyMember {
  const ColonyMember(this.name, this.role);

  final String name;
  final String role;
}

final class ColonyMessage {
  const ColonyMessage(this.from, this.to, this.content);

  final String from;
  final String to;
  final String content;
}

final class ColonyResult {
  const ColonyResult(this.summary, this.state, this.transcript);

  final String summary;
  final JsonObject state;
  final List<ColonyMessage> transcript;
}

final class ColonyOrchestrator {
  final List<(ColonyMember, ColonyAgent)> _members =
      <(ColonyMember, ColonyAgent)>[];

  ColonyOrchestrator addMember(String name, String role, ColonyAgent agent) {
    _members.add((ColonyMember(name, role), agent));
    return this;
  }

  Future<ColonyResult> coordinate(JsonObject initialState) async {
    if (_members.isEmpty) {
      throw StateError("colony requires at least one member");
    }
    var state = copyJsonObject(initialState);
    final List<ColonyMessage> transcript = <ColonyMessage>[];
    for (final (ColonyMember member, ColonyAgent agent) in _members) {
      final response = await Future.value(
        agent(copyJsonObject(state), List<ColonyMessage>.from(transcript), member),
      );
      final message = switch (response) {
        ColonyMessage value => value,
        _ => ColonyMessage(member.name, "all", "$response"),
      };
      transcript.add(message);
      state[member.name] = message.content;
    }
    return ColonyResult(
      transcript.isEmpty ? "" : transcript.last.content,
      state,
      transcript,
    );
  }
}

final class SentinelConfig {
  const SentinelConfig({this.maxSteps = 8, this.metadata = const <String, Object?>{}});

  final int maxSteps;
  final JsonObject metadata;
}

final class StepOutcome {
  const StepOutcome(this.status, this.state, {this.message});

  final String status;
  final JsonObject state;
  final String? message;
}

typedef SentinelAgent = FutureOr<Object?> Function(
  JsonObject state,
  SentinelConfig config,
);

final class SentinelOrchestrator {
  Future<StepOutcome> runAgent(
    SentinelAgent agent,
    JsonObject initialState,
    SentinelConfig config,
  ) async {
    final result = await Future.value(agent(copyJsonObject(initialState), config));
    return switch (result) {
      StepOutcome value => value,
      JsonObject state => StepOutcome("completed", state),
      _ => StepOutcome("completed", initialState, message: "$result"),
    };
  }
}
