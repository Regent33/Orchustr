import "dart:convert";

import "native_bridge.dart";
import "types.dart";

final class CrateBinding {
  const CrateBinding({
    required this.crateName,
    required this.bindingMode,
    required this.description,
    required this.operations,
  });

  final String crateName;
  final String bindingMode;
  final String description;
  final List<String> operations;

  factory CrateBinding.fromJson(JsonObject json) {
    return CrateBinding(
      crateName: "${json["crate_name"]}",
      bindingMode: "${json["binding_mode"]}",
      description: "${json["description"]}",
      operations: (json["operations"] as List<Object?>? ?? const <Object?>[])
          .map((value) => "$value")
          .toList(),
    );
  }
}

final class RustCrateBridge {
  static bool get available => OrchustrNativeBridge.instance != null;

  static List<CrateBinding> catalog() {
    final bridge = OrchustrNativeBridge.instance;
    if (bridge == null) {
      return const <CrateBinding>[];
    }
    final decoded = jsonDecode(bridge.workspaceCatalogJson()) as List<Object?>;
    return decoded
        .whereType<JsonObject>()
        .map(CrateBinding.fromJson)
        .toList();
  }

  static Object? invoke(
    String crateName,
    String operation,
    JsonObject payload,
  ) {
    final bridge = OrchustrNativeBridge.instance;
    if (bridge == null) {
      throw StateError("native bridge is not available");
    }
    return jsonDecode(bridge.invokeCrateJson(
      crateName,
      operation,
      jsonEncode(payload),
    ));
  }
}
