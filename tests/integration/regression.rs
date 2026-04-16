//! Regression tests for bugs found during the security audit.
//! Each test specifically validates a fix applied during the audit.

use or_beacon::{PromptBuilder, PromptOrchestrator};
use or_conduit::ConduitError;
use serde_json::json;

// ── Regression: or-beacon null-byte injection (Bug #1) ──────────────────

#[test]
fn regression_null_bytes_stripped_from_template_variables() {
    let template = PromptBuilder::new()
        .template("Input: {{data}}")
        .build()
        .unwrap();
    let rendered = PromptOrchestrator
        .render_template(&template, &json!({"data": "before\0after"}))
        .unwrap();
    assert!(
        !rendered.contains('\0'),
        "Regression: null bytes must not appear in rendered output"
    );
}

// ── Regression: or-beacon template re-expansion (Bug #2) ────────────────

#[test]
fn regression_template_values_not_reexpanded() {
    let template = PromptBuilder::new()
        .template("Echo: {{input}}")
        .build()
        .unwrap();
    let rendered = PromptOrchestrator
        .render_template(&template, &json!({"input": "{{other_var}}"}))
        .unwrap();
    assert_eq!(
        rendered, "Echo: {{other_var}}",
        "Regression: template placeholders in values must not re-expand"
    );
}

// ── Regression: or-conduit error display includes context (Bug #3) ──────

#[test]
fn regression_conduit_error_display_includes_context() {
    let err = ConduitError::MissingEnvironmentVariable("MY_KEY".to_owned());
    let msg = format!("{err}");
    assert!(
        msg.contains("MY_KEY"),
        "Regression: error display must include the variable name"
    );
}

// ── Regression: or-conduit auth error variant exists (Bug #4) ───────────

#[test]
fn regression_conduit_auth_error_variant_exists() {
    let err = ConduitError::AuthenticationFailed("empty key".to_owned());
    let msg = format!("{err}");
    assert!(
        msg.contains("empty key"),
        "Regression: AuthenticationFailed variant must exist and display context"
    );
}

// ── Regression: or-conduit timeout error variant exists (Bug #5) ────────

#[test]
fn regression_conduit_timeout_error_variant_exists() {
    let err = ConduitError::Timeout;
    let msg = format!("{err}");
    assert!(
        !msg.is_empty(),
        "Regression: Timeout variant must display something"
    );
}
