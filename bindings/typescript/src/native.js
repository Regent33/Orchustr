import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const __dirname = path.dirname(fileURLToPath(import.meta.url));

function _candidates() {
  const envPath = process.env.ORCHUSTR_NODE_ADDON;
  return [
    envPath,
    path.resolve(__dirname, "..", "native", "or_bridge.node"),
    path.resolve(__dirname, "..", "..", "native", "or_bridge.node"),
    path.resolve(process.cwd(), "bindings", "typescript", "native", "or_bridge.node"),
  ].filter(Boolean);
}

function _loadNative() {
  for (const candidate of _candidates()) {
    try {
      if (fs.existsSync(candidate)) {
        return require(candidate);
      }
    } catch {
      // Keep the binding optional so the pure JS layer still works.
    }
  }
  return null;
}

export const nativeBridge = _loadNative();
export const nativeBridgeAvailable = nativeBridge !== null;
