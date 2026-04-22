import { nativeBridge } from "./native.js";

export class RustCrateBridge {
  static available() {
    return nativeBridge !== null;
  }

  static catalog() {
    if (!nativeBridge) return [];
    return JSON.parse(nativeBridge.workspaceCatalogJson());
  }

  static invoke(crateName, operation, payload = {}) {
    if (!nativeBridge) {
      throw new Error("native bridge is not available");
    }
    return JSON.parse(
      nativeBridge.invokeCrateJson(
        crateName,
        operation,
        JSON.stringify(payload),
      ),
    );
  }
}
