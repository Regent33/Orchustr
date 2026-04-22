import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, copyFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..", "..", "..");
const nativeDir = path.resolve(__dirname, "..", "native");

execFileSync(
  "cargo",
  ["build", "-p", "or-bridge", "--features", "node"],
  { cwd: root, stdio: "inherit" },
);

mkdirSync(nativeDir, { recursive: true });

const candidates = [
  path.resolve(root, "target", "debug", process.platform === "win32" ? "or_bridge.dll" : process.platform === "darwin" ? "libor_bridge.dylib" : "libor_bridge.so"),
  path.resolve(root, "target", "release", process.platform === "win32" ? "or_bridge.dll" : process.platform === "darwin" ? "libor_bridge.dylib" : "libor_bridge.so"),
];

const source = candidates.find((candidate) => existsSync(candidate));
if (!source) {
  throw new Error("could not find compiled or-bridge native library");
}

copyFileSync(source, path.resolve(nativeDir, "or_bridge.node"));
console.log("Native addon copied to bindings/typescript/native/or_bridge.node");
