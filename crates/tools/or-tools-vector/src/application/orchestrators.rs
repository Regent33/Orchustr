use crate::domain::contracts::VectorStoreClient;
use crate::domain::entities::{
    CollectionConfig, DeleteRequest, QueryFilter, UpsertBatch, VectorMatch,
};
use crate::domain::errors::VectorError;
use async_trait::async_trait;
use or_tools_core::{Tool, ToolCapability, ToolError, ToolInput, ToolMeta, ToolOutput};

/// Thin orchestrator wrapping a [`VectorStoreClient`] with span logging.
#[derive(Clone)]
pub struct RagOrchestrator<C: VectorStoreClient> {
    client: C,
}

impl<C: VectorStoreClient> RagOrchestrator<C> {
    #[must_use]
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub async fn upsert(&self, batch: UpsertBatch) -> Result<(), VectorError> {
        let span = tracing::info_span!(
            "tools.vector.upsert",
            otel.name = "tools.vector.upsert",
            provider = self.client.name(),
            collection = %batch.collection,
            count = batch.items.len(),
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = self.client.upsert(batch).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }

    pub async fn query(&self, filter: QueryFilter) -> Result<Vec<VectorMatch>, VectorError> {
        let span = tracing::info_span!(
            "tools.vector.query",
            otel.name = "tools.vector.query",
            provider = self.client.name(),
            collection = %filter.collection,
            top_k = filter.top_k,
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = self.client.query(filter).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}

/// Exposes a [`VectorStoreClient`] as an [`or_tools_core::Tool`].
pub struct VectorStoreTool<C: VectorStoreClient> {
    client: C,
}

impl<C: VectorStoreClient> VectorStoreTool<C> {
    #[must_use]
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: VectorStoreClient> Tool for VectorStoreTool<C> {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new(
            format!("vector.{}", self.client.name()),
            format!("{} vector store", self.client.name()),
        )
        .with_capability(ToolCapability::Vector)
        .with_capability(ToolCapability::Network)
    }

    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let op = input
            .payload
            .get("op")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::invalid_input(&input.tool, "missing `op`"))?;

        let payload = match op {
            "upsert" => {
                let batch: UpsertBatch =
                    serde_json::from_value(input.payload.get("data").cloned().unwrap_or_default())
                        .map_err(|e| ToolError::invalid_input(&input.tool, e.to_string()))?;
                self.client.upsert(batch).await?;
                serde_json::json!({ "status": "ok" })
            }
            "query" => {
                let filter: QueryFilter =
                    serde_json::from_value(input.payload.get("data").cloned().unwrap_or_default())
                        .map_err(|e| ToolError::invalid_input(&input.tool, e.to_string()))?;
                let matches = self.client.query(filter).await?;
                serde_json::to_value(matches).map_err(|e| ToolError::Serialization {
                    tool: input.tool.clone(),
                    reason: e.to_string(),
                })?
            }
            "delete" => {
                let req: DeleteRequest =
                    serde_json::from_value(input.payload.get("data").cloned().unwrap_or_default())
                        .map_err(|e| ToolError::invalid_input(&input.tool, e.to_string()))?;
                self.client.delete(req).await?;
                serde_json::json!({ "status": "ok" })
            }
            "ensure_collection" => {
                let cfg: CollectionConfig =
                    serde_json::from_value(input.payload.get("data").cloned().unwrap_or_default())
                        .map_err(|e| ToolError::invalid_input(&input.tool, e.to_string()))?;
                self.client.ensure_collection(cfg).await?;
                serde_json::json!({ "status": "ok" })
            }
            other => {
                return Err(ToolError::invalid_input(
                    &input.tool,
                    format!("unknown op `{other}`"),
                ));
            }
        };
        Ok(ToolOutput::new(input.tool, payload))
    }
}
