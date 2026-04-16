use crate::domain::entities::{AnchorChunk, RetrievedChunk};
use crate::domain::errors::AnchorError;
use crate::infra::implementations::AnchorPipeline;

#[derive(Debug, Clone, Default)]
pub struct AnchorOrchestrator;

impl AnchorOrchestrator {
    pub async fn index_document(
        &self,
        pipeline: &AnchorPipeline,
        document_id: &str,
        text: &str,
    ) -> Result<Vec<AnchorChunk>, AnchorError> {
        let span = tracing::info_span!(
            "anchor.index_document",
            otel.name = "anchor.index_document",
            status = tracing::field::Empty
        );
        let _guard = span.enter();
        let result = pipeline.index_document(document_id, text).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }

    pub async fn retrieve(
        &self,
        pipeline: &AnchorPipeline,
        query: &str,
        limit: usize,
    ) -> Result<Vec<RetrievedChunk>, AnchorError> {
        let span = tracing::info_span!(
            "anchor.retrieve",
            otel.name = "anchor.retrieve",
            status = tracing::field::Empty
        );
        let _guard = span.enter();
        let result = pipeline.retrieve(query, limit).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}
