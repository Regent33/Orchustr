import "dart:async";
import "dart:convert";
import "dart:io";

import "types.dart";

final class CompletionResponse {
  const CompletionResponse(this.text);

  final String text;
}

abstract base class _HttpConduit {
  _HttpConduit(this.apiKey, this.model, this.endpoint, this.headers)
      : _httpClient = HttpClient()
          ..connectionTimeout = const Duration(seconds: 30);

  final String apiKey;
  final String model;
  final Uri endpoint;
  final Map<String, String> headers;

  /// Shared HTTP client — reused across requests for connection pooling.
  final HttpClient _httpClient;

  Future<CompletionResponse> completeMessages(List<JsonObject> messages);

  Future<CompletionResponse> completeText(String prompt) {
    return completeMessages(
      <JsonObject>[
        <String, Object?>{
          "role": "user",
          "content": <JsonObject>[
            <String, Object?>{"type": "text", "text": prompt},
          ],
        },
      ],
    );
  }

  /// Non-streaming fallback — yields the full response as one chunk.
  ///
  /// Override for true SSE streaming.
  Stream<String> streamText(String prompt) async* {
    final response = await completeText(prompt);
    yield response.text;
  }

  /// Performs a JSON POST using the shared [_httpClient].
  Future<CompletionResponse> postJson(
    JsonObject payload,
  ) async {
    final request = await _httpClient.postUrl(endpoint);
    request.headers.contentType = ContentType.json;
    headers.forEach(request.headers.set);
    request.write(jsonEncode(payload));
    final response =
        await request.close().timeout(const Duration(seconds: 30));
    final body = jsonDecode(await response.transform(utf8.decoder).join());
    if (body is! JsonObject) {
      throw StateError("completion response must be a JSON object");
    }
    return CompletionResponse(_extractText(body));
  }
}

final class OpenAiConduit extends _HttpConduit {
  OpenAiConduit(String apiKey, String model, {Uri? endpoint})
      : super(
          apiKey,
          model,
          // Uses the OpenAI Responses API (not Chat Completions).
          // Schema: input=[...], response has output=[{content:[{text:...}]}]
          endpoint ?? Uri.parse("https://api.openai.com/v1/responses"),
          <String, String>{"Authorization": "Bearer $apiKey"},
        );

  factory OpenAiConduit.fromEnv() {
    return OpenAiConduit(
        _requiredEnv("OPENAI_API_KEY"), _requiredEnv("OPENAI_MODEL"));
  }

  @override
  Future<CompletionResponse> completeMessages(List<JsonObject> messages) {
    return postJson(
      <String, Object?>{
        "model": model,
        "input": messages,
        "max_output_tokens": 1024,
      },
    );
  }
}

final class AnthropicConduit extends _HttpConduit {
  AnthropicConduit(String apiKey, String model, {Uri? endpoint})
      : super(
          apiKey,
          model,
          endpoint ?? Uri.parse("https://api.anthropic.com/v1/messages"),
          <String, String>{
            "x-api-key": apiKey,
            "anthropic-version": "2023-06-01",
          },
        );

  factory AnthropicConduit.fromEnv() {
    return AnthropicConduit(
        _requiredEnv("ANTHROPIC_API_KEY"), _requiredEnv("ANTHROPIC_MODEL"));
  }

  @override
  Future<CompletionResponse> completeMessages(List<JsonObject> messages) {
    return postJson(
      <String, Object?>{
        "model": model,
        "messages": messages,
        "max_tokens": 1024,
      },
    );
  }
}

String _extractText(JsonObject body) {
  if (body["choices"] case final List<Object?> choices when choices.isNotEmpty) {
    if (choices.first case final JsonObject first) {
      if (first["message"] case final JsonObject message) {
        if (message["content"] case final String text) {
          return text;
        }
      }
    }
  }
  if (body["output"] case final List<Object?> output) {
    return output
        .whereType<JsonObject>()
        .expand((JsonObject block) =>
            (block["content"] as List<Object?>? ?? const <Object?>[]))
        .whereType<JsonObject>()
        .map((JsonObject item) => item["text"])
        .whereType<String>()
        .join();
  }
  if (body["content"] case final List<Object?> content) {
    return content
        .whereType<JsonObject>()
        .map((JsonObject item) => item["text"])
        .whereType<String>()
        .join();
  }
  return "";
}

const _openAiCompatEndpoints = <String, String>{
  "openai": "https://api.openai.com/v1/chat/completions",
  "openrouter": "https://openrouter.ai/api/v1/chat/completions",
  "together": "https://api.together.xyz/v1/chat/completions",
  "groq": "https://api.groq.com/openai/v1/chat/completions",
  "fireworks": "https://api.fireworks.ai/inference/v1/chat/completions",
  "deepseek": "https://api.deepseek.com/v1/chat/completions",
  "mistral": "https://api.mistral.ai/v1/chat/completions",
  "xai": "https://api.x.ai/v1/chat/completions",
  "nvidia": "https://integrate.api.nvidia.com/v1/chat/completions",
  "ollama": "http://localhost:11434/v1/chat/completions",
};

/// Generic OpenAI-compatible conduit for providers that speak the Chat Completions API.
/// Use the named factory constructors ([openrouter], [groq], [together], [fireworks],
/// [deepseek], [mistral], [xai], [nvidia], [ollama]) or pass a custom endpoint directly.
final class OpenAiCompatConduit extends _HttpConduit {
  OpenAiCompatConduit(String apiKey, String model, Uri endpoint)
      : super(
          apiKey,
          model,
          endpoint,
          <String, String>{"Authorization": "Bearer $apiKey"},
        );

  factory OpenAiCompatConduit.openrouter(String apiKey, String model) =>
      OpenAiCompatConduit(apiKey, model, Uri.parse(_openAiCompatEndpoints["openrouter"]!));

  factory OpenAiCompatConduit.groq(String apiKey, String model) =>
      OpenAiCompatConduit(apiKey, model, Uri.parse(_openAiCompatEndpoints["groq"]!));

  factory OpenAiCompatConduit.together(String apiKey, String model) =>
      OpenAiCompatConduit(apiKey, model, Uri.parse(_openAiCompatEndpoints["together"]!));

  factory OpenAiCompatConduit.fireworks(String apiKey, String model) =>
      OpenAiCompatConduit(apiKey, model, Uri.parse(_openAiCompatEndpoints["fireworks"]!));

  factory OpenAiCompatConduit.deepseek(String apiKey, String model) =>
      OpenAiCompatConduit(apiKey, model, Uri.parse(_openAiCompatEndpoints["deepseek"]!));

  factory OpenAiCompatConduit.mistral(String apiKey, String model) =>
      OpenAiCompatConduit(apiKey, model, Uri.parse(_openAiCompatEndpoints["mistral"]!));

  factory OpenAiCompatConduit.xai(String apiKey, String model) =>
      OpenAiCompatConduit(apiKey, model, Uri.parse(_openAiCompatEndpoints["xai"]!));

  factory OpenAiCompatConduit.nvidia(String apiKey, String model) =>
      OpenAiCompatConduit(apiKey, model, Uri.parse(_openAiCompatEndpoints["nvidia"]!));

  factory OpenAiCompatConduit.ollama(String model, {Uri? endpoint}) =>
      OpenAiCompatConduit("", model, endpoint ?? Uri.parse(_openAiCompatEndpoints["ollama"]!));

  @override
  Future<CompletionResponse> completeMessages(List<JsonObject> messages) {
    return postJson(
      <String, Object?>{
        "model": model,
        "messages": messages,
        "max_tokens": 1024,
      },
    );
  }
}

String _requiredEnv(String key) {
  final value = Platform.environment[key];
  if (value == null || value.isEmpty) {
    throw StateError("missing environment variable: $key");
  }
  return value;
}
