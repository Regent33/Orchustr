import "dart:convert";

import "native_bridge.dart";
import "types.dart";

final _variablePattern = RegExp(r"{{([A-Za-z0-9_]+)}}");

final class PromptTemplate {
  const PromptTemplate(this.template);

  final String template;

  String render(JsonObject context) {
    final bridge = OrchustrNativeBridge.instance;
    if (bridge != null) {
      return bridge.renderPromptJson(template, jsonEncode(context));
    }

    var rendered = template;
    for (final match in _variablePattern.allMatches(template)) {
      final variable = match.group(1)!;
      if (!context.containsKey(variable)) {
        throw StateError("missing variable: $variable");
      }
      rendered = rendered.replaceAll(
        "{{$variable}}",
        sanitizeText("${context[variable]}"),
      );
    }
    return rendered;
  }
}

final class PromptBuilder {
  String? _template;

  PromptBuilder template(String value) {
    _template = sanitizeText(value);
    return this;
  }

  PromptTemplate build() {
    final template = _template;
    if (template == null || template.isEmpty) {
      throw StateError("template must not be empty");
    }
    return PromptTemplate(template);
  }
}
