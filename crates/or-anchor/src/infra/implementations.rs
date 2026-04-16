use crate::domain::entities::{AnchorChunk, RetrievedChunk};
use crate::domain::errors::AnchorError;
use crate::infra::adapters::{chunk_text, embed};
use or_core::{InMemoryVectorStore, VectorStore};

#[derive(Debug, Clone)]
pub struct AnchorPipeline {
    chunk_size: usize,
    store: InMemoryVectorStore,
}

impl Default for AnchorPipeline {
    fn default() -> Self {
        Self {
            chunk_size: 32,
            store: InMemoryVectorStore::new(),
        }
    }
}

impl AnchorPipeline {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size;
        self
    }

    pub async fn index_document(
        &self,
        document_id: &str,
        text: &str,
    ) -> Result<Vec<AnchorChunk>, AnchorError> {
        let chunks = chunk_text(document_id, text, self.chunk_size);
        for chunk in &chunks {
            self.store
                .upsert(
                    &chunk.id,
                    embed(&chunk.text),
                    serde_json::json!({ "text": chunk.text }),
                )
                .await
                .map_err(|error| AnchorError::VectorStore(error.to_string()))?;
        }
        Ok(chunks)
    }

    pub async fn retrieve(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<RetrievedChunk>, AnchorError> {
        let mut results = self
            .store
            .query(embed(query), limit)
            .await
            .map_err(|error| AnchorError::VectorStore(error.to_string()))?
            .into_iter()
            .map(|record| RetrievedChunk {
                id: record.id,
                text: record.metadata["text"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                score: record.score,
            })
            .collect::<Vec<_>>();
        results.sort_by(|left, right| {
            right
                .score
                .partial_cmp(&left.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(results)
    }
}
