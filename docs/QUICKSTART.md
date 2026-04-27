# Quickstart

> **Warning:** Rust is required for workspace development, for the Python binding install, and for any optional native bridge builds in TypeScript or Dart.

## Step 1: Install Prerequisites

Install what you need for your target workflow:

| Target | Requirement |
|---|---|
| Rust workspace | Rust `1.87.0+` |
| Python bindings | Python `3.10+` and `pip` |
| TypeScript bindings | Node.js `20+` and `npm` |
| Dart bindings | Dart SDK `3.0+` |

## Step 2: Clone the Repository

```bash
git clone https://github.com/Cether144/Orchustr.git
cd Orchustr
```

## Step 3: Verify the Rust Workspace

```bash
cargo check --all-features
cargo test --all-features
```

## Step 4: Use the New CLI Scaffold

The current repository includes the `orchustr` CLI in `or-cli`:

```bash
cargo run -p or-cli -- init my-agent --lang python --topology react --provider anthropic
```

Other current CLI commands:

```bash
cargo run -p or-cli -- lint docs/examples/
cargo run -p or-cli -- trace path/to/project
```

Notes:

- `lint` validates graph specs and `orchustr.yaml` references offline. Each validated path is printed.
- `trace` boots the local `or-lens` dashboard, prints the bound port, and keeps it serving until you hit Ctrl-C.
- `run` parses the project's config + graph and shells out to the language entrypoint declared in `orchustr.yaml`:
  - `language: rust` → `cargo run` in the project dir
  - `language: python` → first of `main.py` / `agent.py` / `app.py` via `python` (Windows) or `python3` (Unix)
  - `language: typescript` → `npm start` if `package.json` exists, otherwise `npx tsx src/index.ts`
  - `language: dart` → `dart run` against `pubspec.yaml`

  stdout/stderr are inherited from the child process, and `kill_on_drop` ensures the child is reaped if `orchustr run` is interrupted.

## Step 5: Install a Binding Package

### Python

```bash
cd bindings/python
pip install -e .
pytest tests/ -v
```

This builds the PyO3 extension from `or-bridge` with the `python` feature.

### TypeScript

```bash
cd bindings/typescript
npm ci
npm run typecheck
npm test
```

If you want the optional native addon path:

```bash
npm run build:native
```

### Dart

```bash
cd bindings/dart
dart pub get
dart analyze
dart test
```

If you want the optional native bridge:

```bash
dart run tool/build_native.dart
```

## Step 6: Build From Source by Area

```bash
# MCP and tool import work
cargo test -p or-mcp -p or-forge

# CLI
cargo test -p or-cli

# Local dashboard
cargo test -p or-lens --features dashboard
cargo test -p or-prism --features lens
```

## Known Gaps & Limitations

- Binding coverage is intentionally hybrid rather than a raw 1:1 export of every Rust type or trait.
- Local Windows execution may still be affected by Application Control policy on some machines.
- The TypeScript and Dart native bridges are optional; language-local helpers still work without them.
