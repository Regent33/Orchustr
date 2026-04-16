# Developer Scripts

## `scripts/dev/Invoke-CargoExternalTarget.ps1`

- **Language**: PowerShell
- **Purpose**: runs Cargo with `CARGO_TARGET_DIR` redirected to `%LOCALAPPDATA%\Orchustr\cargo-targets\<profile>` so ad hoc verification builds do not dirty the repository root.
- **How to run**: `./scripts/dev/Invoke-CargoExternalTarget.ps1 phase6 test -p or-prism`
- **Arguments**: first positional argument is the target profile directory name; remaining arguments are forwarded to `cargo`.
- **Output**: prints the resolved `CARGO_TARGET_DIR` and then forwards Cargo output.
- **When to use it**: use it when local Windows policy or repo cleanliness makes external target directories preferable.

⚠️ Known Gaps & Limitations
- The script does not bypass Windows Application Control; it only redirects build artifacts.
- It is specific to PowerShell environments.
