mod memory;

use or_recall::{InMemoryRecallStore, MemoryKind, RecallEntry, RecallOrchestrator};

#[tokio::test]
async fn remember_and_recall_short_term_entries() {
    let store = InMemoryRecallStore::new();
    RecallOrchestrator
        .remember(
            &store,
            RecallEntry {
                id: "1".to_owned(),
                kind: MemoryKind::ShortTerm,
                content: "draft context".to_owned(),
                metadata: serde_json::json!({"scope":"chat"}),
            },
        )
        .await
        .unwrap();
    let entries = RecallOrchestrator
        .recall(&store, MemoryKind::ShortTerm)
        .await
        .unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].content, "draft context");
}

#[tokio::test]
async fn recall_returns_empty_for_missing_memory_kind() {
    let store = InMemoryRecallStore::new();
    let entries = RecallOrchestrator
        .recall(&store, MemoryKind::Episodic)
        .await
        .unwrap();
    assert!(entries.is_empty());
}
