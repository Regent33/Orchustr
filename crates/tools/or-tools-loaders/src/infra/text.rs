use super::shared::{chunk, read_source};
use crate::domain::contracts::DocumentLoader;
use crate::domain::entities::{Document, DocumentKind, LoaderRequest};
use crate::domain::errors::LoaderError;
use async_trait::async_trait;

pub struct TextLoader;

#[async_trait]
impl DocumentLoader for TextLoader {
    fn name(&self) -> &'static str {
        "text"
    }

    async fn load(&self, req: LoaderRequest) -> Result<Vec<Document>, LoaderError> {
        let content = read_source(&req).await?;
        Ok(chunk(content, DocumentKind::Text, req.chunk_size))
    }
}
