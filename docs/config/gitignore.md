# .gitignore

## File

- Path: `.gitignore`

## Ignored Paths

- `/target`
- `/.target_*`
- `/node_modules`
- `/bindings/python/.pytest_cache`
- `/bindings/python/pytest-cache-files-*`
- `/bindings/python/**/__pycache__/`
- `/bindings/python/**/*.pyc`
- `/bindings/dart/.dart_tool`
- `/bindings/dart/native`
- `/bindings/dart/pubspec.lock`
- `/bindings/typescript/native`
- `Cargo.lock`
- `.DS_Store`

## Why These Rules Exist

- `target` and `/.target_*` keep Cargo build outputs out of version control.
- `node_modules`, Python bytecode, and pytest cache directories keep local package and test artifacts out of the repo.
- Dart native artifacts and TypeScript native addon builds are generated locally and should not be committed.
- `Cargo.lock` is ignored in this workspace, which is an explicit project choice even though Rust applications often commit it.

⚠️ Known Gaps & Limitations

- Ignoring `Cargo.lock` is a project policy choice that may not fit every Rust release strategy.
- This file covers the local build artifacts observed in the current repository, not every possible editor or tool cache.
