# Quickstart

> **⚠️ READ FIRST:** Rust is required for the Python binding install and for any local native bridge builds in TypeScript or Dart. The TypeScript and Dart packages can still be installed as language-only packages first, but native crate access depends on local Rust builds.

---

## Step 1: Install the Rust Toolchain

Install Rust using the official installer. This works on Windows, macOS, and Linux:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows alternative:** Download and run [https://rustup.rs](https://rustup.rs) directly in your browser.

After installation, verify it works:

```bash
rustc --version
cargo --version
```

---

## Step 2: Install Language-Specific Prerequisites

Only install what you need for your target language:

| Language | Tool to install |
|---|---|
| **Python** | Python 3.10+ and `pip` |
| **TypeScript** | [Node.js 20+](https://nodejs.org) and `npm` |
| **Dart / Flutter** | [Dart SDK 3.0+](https://dart.dev/get-dart) or Flutter SDK |

---

## Step 3: Clone the Orchustr Repository

> **Note:** `or-core` and `or-prism` are already published on [crates.io](https://crates.io). For Rust-only projects you can depend on them directly by version. For Python, TypeScript, and Dart, cloning the repository remains the easiest path for local native builds and current binding development.

```bash
git clone https://github.com/Cether144/Orchustr.git
cd Orchustr
```

---

## Step 4: Add Orchustr To Your Own Project

Once cloned, you link it into your existing project using a path reference. Replace `/absolute/path/to/Orchustr` with the real path on your machine.

### Rust

**Option A - crates.io**

```toml
[dependencies]
or-core  = "0.1.2"
or-prism = "0.1.2"
```

**Option B - local path**

```toml
[dependencies]
or-core     = { path = "/absolute/path/to/Orchustr/crates/or-core" }
or-sentinel = { path = "/absolute/path/to/Orchustr/crates/or-sentinel" }
```

Then run:

```bash
cargo build
```

### TypeScript / Node.js

Point `npm` at the local bindings folder:

```bash
npm install "/absolute/path/to/Orchustr/bindings/typescript"
```

If you want native crate access through `RustCrateBridge` and the `*Tools` wrappers, build the optional addon:

```bash
cd /absolute/path/to/Orchustr/bindings/typescript
npm ci
npm run build:native
```

### Python

`pip` will invoke `maturin`, which will call `cargo` to compile the Rust extension:

```bash
pip install "/absolute/path/to/Orchustr/bindings/python"
```

If you get a `cargo not found` error here, go back to Step 1.

### Dart / Flutter

Add a path dependency to your `pubspec.yaml`:

```yaml
dependencies:
  orchustr:
    path: /absolute/path/to/Orchustr/bindings/dart
```

Then run these commands if you want the optional native bridge:

```bash
dart pub get
cd /absolute/path/to/Orchustr/bindings/dart
dart run tool/build_native.dart
```

If you skip `build_native.dart`, the Dart package still works for the Dart-local helpers, but `RustCrateBridge` and the Rust-backed `*Tools` helpers will not be available.

---

## For Contributors: Build and Test the Full Workspace

```bash
# Rust
cargo check --all-features
cargo test --all-features

# Python
cd bindings/python && maturin develop && pytest tests/

# TypeScript
cd bindings/typescript && npm ci && npm run build:native && npm run typecheck && npm test

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

- Binding coverage now exists across the workspace, but not as a raw 1:1 export of every Rust type or trait.
- Local Windows execution may still be affected by Application Control policy on some machines.
- The TypeScript binding still does not auto-compile native Rust on `npm install`.
