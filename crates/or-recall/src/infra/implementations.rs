use crate::domain::contracts::RecallStore;
use crate::domain::entities::{MemoryKind, RecallEntry};
use crate::domain::errors::RecallError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Default)]
pub struct InMemoryRecallStore {
    entries: Arc<RwLock<HashMap<MemoryKind, Vec<RecallEntry>>>>,
}

impl InMemoryRecallStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl RecallStore for InMemoryRecallStore {
    async fn store(&self, entry: RecallEntry) -> Result<(), RecallError> {
        self.entries
            .write()
            .await
            .entry(entry.kind.clone())
            .or_default()
            .push(entry);
        Ok(())
    }

    async fn list(&self, kind: MemoryKind) -> Result<Vec<RecallEntry>, RecallError> {
        Ok(self
            .entries
            .read()
            .await
            .get(&kind)
            .cloned()
            .unwrap_or_default())
    }
}
