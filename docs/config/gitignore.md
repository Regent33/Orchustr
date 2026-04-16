# .gitignore

## File

- Path: `.gitignore`

## Ignored Paths

- `/target`
- `/.target_*`
- `/node_modules`
- `/bindings/python/.pytest_cache`
- `/bindings/python/pytest-cache-files-*`
- `Cargo.lock`
- `.DS_Store`

## Why These Rules Exist

- `target` and `/.target_*` keep Cargo build outputs out of version control.
- `node_modules` and pytest cache directories keep local package and test artifacts out of the repo.
- `Cargo.lock` is ignored in this workspace, which is an explicit project choice even though Rust applications often commit it.

⚠️ Known Gaps & Limitations
- Ignoring `Cargo.lock` is a project policy choice that may not fit every Rust release strategy.
- The file does not currently ignore Python `__pycache__` directories or TypeScript local caches beyond `node_modules`.
