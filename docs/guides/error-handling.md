# Error Handling

Every Rust crate defines its own error enum instead of sharing a single global error type. That keeps crate boundaries explicit and makes it easier to understand which layer failed.

## Patterns in the Repository

- Domain errors are defined in `src/domain/errors.rs`.
- Application orchestrators typically wrap execution in tracing spans and return the crate error type directly.
- Integration crates such as `or-sentinel` often convert lower-level errors into string-carrying variants.

## Practical Advice

- Match on crate-local error enums at the layer where you call into that crate.
- Preserve context when re-wrapping errors instead of collapsing everything into `String` too early.
- Prefer validating state and schemas before execution to fail earlier with more specific variants.

⚠️ Known Gaps & Limitations
- Some crates currently store lower-level failures as strings rather than typed nested enums.
- There is no repository-wide error code registry in source; see the generated reference page for a documentation-only index.
