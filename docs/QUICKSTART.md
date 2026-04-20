# Quickstart

> **⚠️ CRITICAL — READ FIRST:** The Orchustr bindings for Python, TypeScript, and Dart all compile native Rust code internally. You **MUST** install the Rust compiler (`rustc` + `cargo`) on your machine **before** running any `pip install`, `npm install`, or `dart pub get` commands. If Rust is not installed, all installations will fail.

---

## Step 1: Install the Rust Toolchain

Install Rust using the official installer. This works on Windows, macOS, and Linux:

```bash
# All platforms — installs rustc, cargo, rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows alternative:** Download and run [https://rustup.rs](https://rustup.rs) directly in your browser.

After installation, verify it works:
```bash
rustc --version   # should print: rustc 1.87.0 (or newer)
cargo --version
```

---

## Step 2: Install Language-Specific Prerequisites

Only install what you need for your target language:

| Language | Tool to install |
|---|---|
| **Python** | Python 3.8+ and `pip` |
| **TypeScript** | [Node.js 20+](https://nodejs.org) and `npm` |
| **Dart / Flutter** | [Dart SDK 3.0+](https://dart.dev/get-dart) or Flutter SDK |

---

## Step 3: Clone the Orchustr Repository

Because Orchustr is not yet fully published to all package registries, you need to download the repository to your machine first:

```bash
git clone https://github.com/Cether144/Orchustr.git
cd Orchustr
```

---

## Step 4: Add Orchustr To Your Own Project

Once cloned, you link it into your existing project using a path reference. Replace `/absolute/path/to/Orchustr` with the real path on your machine (e.g. `C:/dev/Orchustr` on Windows).

### Rust 🦀
Add to your project's `Cargo.toml`. Cargo will compile and link it automatically:
```toml
[dependencies]
or-core     = { path = "/absolute/path/to/Orchustr/crates/or-core" }
or-sentinel = { path = "/absolute/path/to/Orchustr/crates/or-sentinel" }
```
Then run:
```bash
cargo build
```

### TypeScript / Node.js 🟨
Point `npm` at the local bindings folder. This copies the JavaScript API bridge into your `node_modules`:
```bash
# Inside your own project directory
npm install "/absolute/path/to/Orchustr/bindings/typescript"
```
> ⚠️ This only links the JavaScript layer. The native Rust code is **not** compiled automatically on `npm install`. Full native performance is not yet wired through `napi-build`.

### Python 🐍
`pip` will invoke `maturin` automatically, which will call `cargo` to compile the Rust extension:
```bash
# Inside your own project directory (activate venv first if applicable)
pip install "/absolute/path/to/Orchustr/bindings/python"
```
If you get a `cargo not found` error here, go back to Step 1.

### Dart / Flutter 🎯
Add a path dependency to your `pubspec.yaml`:
```yaml
dependencies:
  orchustr:
    path: /absolute/path/to/Orchustr/bindings/dart
```
Then run these **two commands in order** — both are required:
```bash
# Step 1: Fetch the Dart package
dart pub get

# Step 2: Compile the native Rust FFI library (pub get does NOT do this automatically)
cd /absolute/path/to/Orchustr/bindings/dart
dart run tool/build_native.dart
```
> ⚠️ If you skip `build_native.dart`, the Dart code will crash at runtime with a `DynamicLibrary` missing error.

---

## For Contributors: Build and Test the Full Workspace

If you are developing Orchustr itself (not just using it), here is how you run the internal test suites:

```bash
# Rust
cargo check --all-features
cargo test --all-features

# Python
cd bindings/python && maturin develop && pytest tests/

# TypeScript
cd bindings/typescript && npm ci && npm run typecheck && npm test

# Dart
cd bindings/dart && dart pub get && dart run tool/build_native.dart && dart run test/bindings_test.dart
```

---

## Example: Your First Agent (Rust)

```rust
use or_beacon::PromptBuilder;
use or_core::DynState;
use or_loom::{GraphBuilder, NodeResult};

async fn example() -> anyhow::Result<()> {
    let prompt = PromptBuilder::new().template("Hello, {{name}}!").build()?;
    let graph = GraphBuilder::<DynState>::new()
        .add_node("render", move |mut state: DynState| {
            let prompt = prompt.clone();
            async move {
                state.insert("message".into(), serde_json::json!(prompt.render(&state)?));
                Ok(NodeResult::advance(state))
            }
        })
        .set_entry("render")
        .set_exit("render")
        .build()?;
    Ok(())
}
```

---

## ⚠️ Known Gaps & Limitations

- Some advanced features described in docs, such as full native binding parity, are not yet complete in code.
- Local Windows execution may still be affected by Application Control policy on some machines.
- The TypeScript binding does not yet auto-compile native Rust on `npm install`.
