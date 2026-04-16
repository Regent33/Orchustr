use crate::domain::contracts::RecallStore;
use crate::domain::entities::{MemoryKind, RecallEntry};
use crate::domain::errors::RecallError;

#[derive(Debug, Clone, Default)]
pub struct RecallOrchestrator;

impl RecallOrchestrator {
    pub async fn remember<S: RecallStore>(
        &self,
        store: &S,
        entry: RecallEntry,
    ) -> Result<(), RecallError> {
        let span = tracing::info_span!(
            "recall.remember",
            otel.name = "recall.remember",
            status = tracing::field::Empty
        );
        let _guard = span.enter();
        let result = store.store(entry).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }

    pub async fn recall<S: RecallStore>(
        &self,
        store: &S,
        kind: MemoryKind,
    ) -> Result<Vec<RecallEntry>, RecallError> {
        let span = tracing::info_span!(
            "recall.recall",
            otel.name = "recall.recall",
            status = tracing::field::Empty
        );
        let _guard = span.enter();
        let result = store.list(kind).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}
