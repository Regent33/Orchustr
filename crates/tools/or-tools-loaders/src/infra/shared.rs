use crate::domain::entities::{Document, DocumentKind, LoaderRequest, LoaderSource};
use crate::domain::errors::LoaderError;
use tokio::io::AsyncReadExt;

pub(crate) async fn read_source(req: &LoaderRequest) -> Result<String, LoaderError> {
    match &req.source {
        LoaderSource::Raw { content } => Ok(content.clone()),
        LoaderSource::Path { path } => {
            let mut file = tokio::fs::File::open(path)
                .await
                .map_err(|e| LoaderError::Io {
                    path: path.clone(),
                    reason: e.to_string(),
                })?;
            let mut buf = String::new();
            file.read_to_string(&mut buf)
                .await
                .map_err(|e| LoaderError::Io {
                    path: path.clone(),
                    reason: e.to_string(),
                })?;
            Ok(buf)
        }
    }
}

/// Splits `content` into ≤`chunk_size` character chunks. If `chunk_size` is 0
/// the whole content is returned as a single document.
pub(crate) fn chunk(content: String, kind: DocumentKind, chunk_size: usize) -> Vec<Document> {
    if chunk_size == 0 || content.len() <= chunk_size {
        return vec![Document::new(content, kind)];
    }
    content
        .char_indices()
        .step_by(chunk_size)
        .enumerate()
        .map(|(i, (byte_idx, _))| {
            let end = (byte_idx + chunk_size).min(content.len());
            let slice = content[byte_idx..end].to_string();
            Document {
                content: slice,
                kind,
                chunk_index: i,
                metadata: serde_json::Value::Null,
            }
        })
        .collect()
}
