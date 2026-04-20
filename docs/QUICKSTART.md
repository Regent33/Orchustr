# Quickstart

## Prerequisites

- Rust `1.87.0` with `clippy` and `rustfmt`
- Python if you want `bindings/python`
- Node.js `20` if you want `bindings/typescript`
- Dart SDK if you want `bindings/dart`

## Adding Orchustr To Your Project

Orchustr is designed to be an easily insertable library. To use it in a new or existing app, you simply link to the downloaded repository.

### 1. Rust 🦀
Add Orchustr to your `Cargo.toml`. *(Once the Crates.io rate-limit passes, you can just `cargo add or-core` globally)*:
```toml
[dependencies]
or-core = { path = "path/to/orchustr/crates/or-core" }
or-sentinel = { path = "path/to/orchustr/crates/or-sentinel" }
```

### 2. TypeScript / Node.js 🟨
Install the TypeScript bindings into your app from the local path:
```bash
npm install "/path/to/orchustr/bindings/typescript"
```

### 3. Python 🐍
Install the Python bindings into your app from the local path:
```bash
pip install "/path/to/orchustr/bindings/python"
```

### 4. Dart / Flutter 🎯
Add a path reference to your `pubspec.yaml` pointing to the Dart bindings:
```yaml
dependencies:
  orchustr:
    path: /path/to/orchustr/bindings/dart
```
```bash
dart pub get
```

---

## Build the Rust Workspace

```bash
cargo check --all-features
cargo test --all-features
```

## Try a Prompt and Graph

```rust
use or_beacon::PromptBuilder;
use or_core::DynState;
use or_loom::{GraphBuilder, NodeResult};

# async fn example() -> anyhow::Result<()> {
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
# Ok(()) }
```

## Python, TypeScript, and Dart

- Python: `cd bindings/python && maturin develop && pytest tests/`
- TypeScript: `cd bindings/typescript && npm ci && npm run typecheck && npm test`
- Dart: `cd bindings/dart && dart pub get && dart run tool/build_native.dart && dart run test/bindings_test.dart`

⚠️ Known Gaps & Limitations
- Some advanced features described in docs, such as full native binding parity, are not yet complete in code.
- Local Windows execution may still be affected by Application Control policy on some machines.
