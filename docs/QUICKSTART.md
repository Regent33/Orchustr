# Quickstart

## Prerequisites

- Rust `1.87.0` with `clippy` and `rustfmt`
- Python if you want `bindings/python`
- Node.js `20` if you want `bindings/typescript`

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

## Python and TypeScript

- Python: `cd bindings/python && maturin develop && pytest tests/`
- TypeScript: `cd bindings/typescript && npm ci && npm run typecheck && npm test`

⚠️ Known Gaps & Limitations
- Some advanced features described in docs, such as full native binding parity, are not yet complete in code.
- Local Windows execution may still be affected by Application Control policy on some machines.
