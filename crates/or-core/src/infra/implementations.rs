use crate::domain::contracts::{PersistenceBackend, VectorStore};
use crate::domain::entities::VectorRecord;
use crate::domain::errors::CoreError;
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type StoredVector = (Vec<f32>, Value);
type VectorStoreMap = HashMap<String, StoredVector>;

#[derive(Debug, Clone, Default)]
pub struct InMemoryPersistenceBackend {
    store: Arc<RwLock<HashMap<String, Value>>>,
}

impl InMemoryPersistenceBackend {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl PersistenceBackend for InMemoryPersistenceBackend {
    async fn save_state(&self, scope: &str, state: Value) -> Result<(), CoreError> {
        self.store.write().await.insert(scope.to_owned(), state);
        Ok(())
    }

    async fn load_state(&self, scope: &str) -> Result<Option<Value>, CoreError> {
        Ok(self.store.read().await.get(scope).cloned())
    }
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryVectorStore {
    store: Arc<RwLock<VectorStoreMap>>,
}

impl InMemoryVectorStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl VectorStore for InMemoryVectorStore {
    async fn upsert(&self, id: &str, vector: Vec<f32>, metadata: Value) -> Result<(), CoreError> {
        self.store
            .write()
            .await
            .insert(id.to_owned(), (vector, metadata));
        Ok(())
    }

    async fn query(&self, vector: Vec<f32>, limit: usize) -> Result<Vec<VectorRecord>, CoreError> {
        let store = self.store.read().await;
        let mut records = store
            .iter()
            .map(|(id, (candidate, metadata))| VectorRecord {
                id: id.clone(),
                score: cosine_similarity(&vector, candidate),
                metadata: metadata.clone(),
            })
            .collect::<Vec<_>>();
        records.sort_by(|left, right| {
            right
                .score
                .partial_cmp(&left.score)
                .unwrap_or(Ordering::Equal)
        });
        records.truncate(limit);
        Ok(records)
    }
}

fn cosine_similarity(left: &[f32], right: &[f32]) -> f32 {
    if left.len() != right.len() || left.is_empty() {
        return 0.0;
    }

    let dot = left.iter().zip(right).map(|(l, r)| l * r).sum::<f32>();
    let left_norm = left.iter().map(|value| value * value).sum::<f32>().sqrt();
    let right_norm = right.iter().map(|value| value * value).sum::<f32>().sqrt();
    if left_norm == 0.0 || right_norm == 0.0 {
        0.0
    } else {
        dot / (left_norm * right_norm)
    }
}
