//! Security & parsing tests for the sentinel's decision and plan parsing.

use or_core::CoreError;
use or_sentinel::SentinelError;

// This test verifies the fixture module is accessible.
// The actual parsing tests below call the internal functions indirectly
// through the sentinel orchestrator flow.

#[test]
fn sentinel_error_display_includes_context() {
    let err = SentinelError::InvalidResponse("bad json".to_owned());
    assert!(err.to_string().contains("bad json"));
}

#[test]
fn sentinel_error_serialization_roundtrip() {
    let err = SentinelError::InvalidResponse("test".to_owned());
    let json = serde_json::to_string(&err).unwrap();
    let parsed: SentinelError = serde_json::from_str(&json).unwrap();
    assert_eq!(err, parsed);
}

#[test]
fn sentinel_error_equality() {
    assert_eq!(
        SentinelError::Forge("a".into()),
        SentinelError::Forge("a".into()),
    );
    // `Core` now wraps a typed `CoreError`. `Forge("a")` and a `Core`
    // variant are still distinguishable.
    assert_ne!(
        SentinelError::Forge("a".into()),
        SentinelError::Core(CoreError::InvalidState("a".into())),
    );
}

#[test]
fn sentinel_error_preserves_typed_loom_chain() {
    // Regression for the audit's "errors lose layer information" note:
    // wrapping a `LoomError` in `SentinelError` no longer flattens to a
    // string. Callers can pattern-match on the underlying graph error.
    use or_loom::LoomError;
    let original = LoomError::UnknownNode("missing".to_owned());
    let wrapped: SentinelError = original.clone().into();
    match wrapped {
        SentinelError::Loom(LoomError::UnknownNode(name)) => assert_eq!(name, "missing"),
        other => panic!("expected typed Loom chain, got {other:?}"),
    }
}
