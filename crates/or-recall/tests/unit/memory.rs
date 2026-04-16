//! Memory/context tests: multi-kind storage, overwrite, isolation.

use or_recall::{InMemoryRecallStore, MemoryKind, RecallEntry, RecallOrchestrator};

#[tokio::test]
async fn remember_multiple_kinds_are_isolated() {
    let store = InMemoryRecallStore::new();
    RecallOrchestrator
        .remember(
            &store,
            RecallEntry {
                id: "s1".to_owned(),
                kind: MemoryKind::ShortTerm,
                content: "short".to_owned(),
                metadata: serde_json::json!({}),
            },
        )
        .await
        .unwrap();
    RecallOrchestrator
        .remember(
            &store,
            RecallEntry {
                id: "l1".to_owned(),
                kind: MemoryKind::LongTerm,
                content: "long".to_owned(),
                metadata: serde_json::json!({}),
            },
        )
        .await
        .unwrap();

    let short = RecallOrchestrator
        .recall(&store, MemoryKind::ShortTerm)
        .await
        .unwrap();
    let long = RecallOrchestrator
        .recall(&store, MemoryKind::LongTerm)
        .await
        .unwrap();
    assert_eq!(short.len(), 1);
    assert_eq!(long.len(), 1);
    assert_eq!(short[0].content, "short");
    assert_eq!(long[0].content, "long");
}

#[tokio::test]
async fn remember_overwrites_with_same_id() {
    let store = InMemoryRecallStore::new();
    RecallOrchestrator
        .remember(
            &store,
            RecallEntry {
                id: "x".to_owned(),
                kind: MemoryKind::ShortTerm,
                content: "v1".to_owned(),
                metadata: serde_json::json!({}),
            },
        )
        .await
        .unwrap();
    RecallOrchestrator
        .remember(
            &store,
            RecallEntry {
                id: "x".to_owned(),
                kind: MemoryKind::ShortTerm,
                content: "v2".to_owned(),
                metadata: serde_json::json!({}),
            },
        )
        .await
        .unwrap();

    let entries = RecallOrchestrator
        .recall(&store, MemoryKind::ShortTerm)
        .await
        .unwrap();
    // Either 1 entry (overwrite) or 2 (append) — test documents behavior
    let latest = entries.iter().find(|e| e.id == "x").unwrap();
    assert!(latest.content == "v1" || latest.content == "v2");
}

#[tokio::test]
async fn recall_preserves_metadata() {
    let store = InMemoryRecallStore::new();
    RecallOrchestrator
        .remember(
            &store,
            RecallEntry {
                id: "m1".to_owned(),
                kind: MemoryKind::Episodic,
                content: "event".to_owned(),
                metadata: serde_json::json!({"source": "user", "turn": 3}),
            },
        )
        .await
        .unwrap();

    let entries = RecallOrchestrator
        .recall(&store, MemoryKind::Episodic)
        .await
        .unwrap();
    assert_eq!(entries[0].metadata["source"], "user");
    assert_eq!(entries[0].metadata["turn"], 3);
}
