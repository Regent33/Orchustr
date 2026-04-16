typedef JsonObject = Map<String, Object?>;

JsonObject copyJsonObject(JsonObject value) => Map<String, Object?>.from(value);

String sanitizeText(String value) {
  final buffer = StringBuffer();
  for (final rune in value.runes) {
    if (rune >= 0x20 || rune == 0x0A || rune == 0x09) {
      buffer.writeCharCode(rune);
    }
  }
  return buffer.toString();
}
