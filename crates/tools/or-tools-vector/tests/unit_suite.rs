use async_trait::async_trait;
use or_tools_core::{Tool, ToolError, ToolInput};
use or_tools_vector::application::orchestrators::VectorStoreTool;
use or_tools_vector::{
    CollectionConfig, DeleteRequest, Distance, QueryFilter, RagOrchestrator, UpsertBatch,
    UpsertItem, VectorError, VectorMatch, VectorStoreClient,
};
use serde_json::{Value, json};

struct StubVectorStore {
    store: std::sync::Arc<tokio::sync::Mutex<Vec<UpsertItem>>>,
}

impl StubVectorStore {
    fn new() -> Self {
        Self {
            store: Default::default(),
        }
    }
}

#[async_trait]
impl VectorStoreClient for StubVectorStore {
    fn name(&self) -> &'static str {
        "stub"
    }

    async fn ensure_collection(&self, _cfg: CollectionConfig) -> Result<(), VectorError> {
        Ok(())
    }

    async fn upsert(&self, batch: UpsertBatch) -> Result<(), VectorError> {
        self.store.lock().await.extend(batch.items);
        Ok(())
    }

    async fn delete(&self, req: DeleteRequest) -> Result<(), VectorError> {
        let mut store = self.store.lock().await;
        store.retain(|item| !req.ids.contains(&item.id));
        Ok(())
    }

    async fn query(&self, filter: QueryFilter) -> Result<Vec<VectorMatch>, VectorError> {
        let store = self.store.lock().await;
        Ok(store
            .iter()
            .take(filter.top_k as usize)
            .map(|item| VectorMatch {
                id: item.id.clone(),
                score: 0.9,
                metadata: item.metadata.clone(),
            })
            .collect())
    }
}

fn make_batch() -> UpsertBatch {
    UpsertBatch {
        collection: "test".into(),
        items: vec![UpsertItem {
            id: "id1".into(),
            vector: vec![0.1, 0.2, 0.3],
            metadata: json!({ "text": "hello" }),
        }],
    }
}

#[tokio::test]
async fn upsert_stores_items() {
    let store = StubVectorStore::new();
    let arc_store = std::sync::Arc::clone(&store.store);
    let orch = RagOrchestrator::new(store);
    orch.upsert(make_batch()).await.unwrap();
    assert_eq!(arc_store.lock().await.len(), 1);
}

#[tokio::test]
async fn query_returns_matches() {
    let store = StubVectorStore::new();
    let orch = RagOrchestrator::new(store);
    orch.upsert(make_batch()).await.unwrap();
    let matches = orch
        .query(QueryFilter {
            collection: "test".into(),
            vector: vec![0.1, 0.2, 0.3],
            top_k: 5,
            filter: None,
        })
        .await
        .unwrap();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].id, "id1");
}

#[tokio::test]
async fn tool_dispatch_upsert() {
    let tool = VectorStoreTool::new(StubVectorStore::new());
    let out = tool
        .invoke(ToolInput::new(
            "vector.stub",
            json!({
                "op": "upsert",
                "data": {
                    "collection": "test",
                    "items": [{ "id": "x", "vector": [0.1], "metadata": {} }]
                }
            }),
        ))
        .await
        .unwrap();
    assert_eq!(out.payload["status"], "ok");
}

#[tokio::test]
async fn tool_dispatch_unknown_op() {
    let tool = VectorStoreTool::new(StubVectorStore::new());
    let err = tool
        .invoke(ToolInput::new("vector.stub", json!({ "op": "bork" })))
        .await
        .unwrap_err();
    assert!(matches!(err, ToolError::InvalidInput { .. }));
}

#[tokio::test]
async fn distance_serializes() {
    let d = Distance::Cosine;
    assert_eq!(d.as_str(), "cosine");
    let json_val: Value = serde_json::to_value(d).unwrap();
    assert_eq!(json_val, "cosine");
}
