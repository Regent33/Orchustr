import "dart:async";
import "dart:convert";
import "dart:io";

import "types.dart";

final class CompletionResponse {
  const CompletionResponse(this.text);

  final String text;
}

abstract base class _HttpConduit {
  _HttpConduit(this.apiKey, this.model, this.endpoint, this.headers);

  final String apiKey;
  final String model;
  final Uri endpoint;
  final Map<String, String> headers;

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

  Stream<String> streamText(String prompt) async* {
    final response = await completeText(prompt);
    for (final chunk in response.text.split(RegExp(r"\s+"))) {
      if (chunk.isNotEmpty) {
        yield chunk;
      }
    }
  }
}

final class OpenAiConduit extends _HttpConduit {
  OpenAiConduit(String apiKey, String model, {Uri? endpoint})
      : super(
          apiKey,
          model,
          endpoint ?? Uri.parse("https://api.openai.com/v1/responses"),
          <String, String>{"Authorization": "Bearer $apiKey"},
        );

  factory OpenAiConduit.fromEnv() {
    return OpenAiConduit(
        _requiredEnv("OPENAI_API_KEY"), _requiredEnv("OPENAI_MODEL"));
  }

  @override
  Future<CompletionResponse> completeMessages(List<JsonObject> messages) {
    return _postJson(
      endpoint,
      <String, Object?>{
        "model": model,
        "input": messages,
        "max_output_tokens": 1024,
      },
      headers,
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
    return _postJson(
      endpoint,
      <String, Object?>{
        "model": model,
        "messages": messages,
        "max_tokens": 1024,
      },
      headers,
    );
  }
}

Future<CompletionResponse> _postJson(
  Uri endpoint,
  JsonObject payload,
  Map<String, String> headers,
) async {
  final client = HttpClient()..connectionTimeout = const Duration(seconds: 30);
  try {
    final request = await client.postUrl(endpoint);
    request.headers.contentType = ContentType.json;
    headers.forEach(request.headers.set);
    request.write(jsonEncode(payload));
    final response = await request.close().timeout(const Duration(seconds: 30));
    final body = jsonDecode(await response.transform(utf8.decoder).join());
    if (body is! JsonObject) {
      throw StateError("completion response must be a JSON object");
    }
    return CompletionResponse(_extractText(body));
  } finally {
    client.close(force: true);
  }
}

String _extractText(JsonObject body) {
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

String _requiredEnv(String key) {
  final value = Platform.environment[key];
  if (value == null || value.isEmpty) {
    throw StateError("missing environment variable: $key");
  }
  return value;
}
