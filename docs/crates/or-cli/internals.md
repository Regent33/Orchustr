# or-cli Internals

`or-cli` follows the same lightweight backend crate split used by the stronger runtime crates in this workspace:

- `application/`: orchestrates scaffold generation, config loading, lint validation, and runner handoff.
- `domain/`: owns CLI entities and `CliError`.
- `infra/`: embeds and renders templates.

## Design Notes

- Template files are embedded with `include_str!()` so the binary can scaffold projects without external assets at runtime.
- The crate depends on `or-schema` for descriptor parsing and `or-lens` for local dashboard bootstrap checks.
- The current `ProjectRunner` abstraction keeps `run_project` testable without forcing a specific runtime implementation into the CLI crate.
