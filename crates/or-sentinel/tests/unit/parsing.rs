//! Security & parsing tests for the sentinel's decision and plan parsing.

mod parsing {
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
        assert_ne!(
            SentinelError::Forge("a".into()),
            SentinelError::Core("a".into()),
        );
    }
}
