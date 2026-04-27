use super::shared::{chunk, read_source};
use crate::domain::contracts::DocumentLoader;
use crate::domain::entities::{Document, DocumentKind, LoaderRequest};
use crate::domain::errors::LoaderError;
use async_trait::async_trait;

pub struct JsonLoader;

#[async_trait]
impl DocumentLoader for JsonLoader {
    fn name(&self) -> &'static str {
        "json"
    }

    async fn load(&self, req: LoaderRequest) -> Result<Vec<Document>, LoaderError> {
        let raw = read_source(&req).await?;

        // Validate it parses as JSON, then pretty-print for readability.
        let parsed: serde_json::Value =
            serde_json::from_str(&raw).map_err(|e| LoaderError::Parse(e.to_string()))?;
        let pretty =
            serde_json::to_string_pretty(&parsed).map_err(|e| LoaderError::Parse(e.to_string()))?;

        Ok(chunk(pretty, DocumentKind::Json, req.chunk_size))
    }
}
