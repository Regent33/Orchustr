use super::shared::read_source;
use crate::domain::contracts::DocumentLoader;
use crate::domain::entities::{Document, DocumentKind, LoaderRequest};
use crate::domain::errors::LoaderError;
use async_trait::async_trait;
use serde_json::{Map, Value};

pub struct CsvLoader;

#[async_trait]
impl DocumentLoader for CsvLoader {
    fn name(&self) -> &'static str {
        "csv"
    }

    async fn load(&self, req: LoaderRequest) -> Result<Vec<Document>, LoaderError> {
        let raw = read_source(&req).await?;
        let mut reader = csv::Reader::from_reader(raw.as_bytes());

        let headers: Vec<String> = reader
            .headers()
            .map_err(|e| LoaderError::Parse(e.to_string()))?
            .iter()
            .map(String::from)
            .collect();

        let mut docs: Vec<Document> = Vec::new();
        let mut chunk_idx = 0usize;
        let mut buf = String::new();
        let chunk_size = if req.chunk_size == 0 {
            usize::MAX
        } else {
            req.chunk_size
        };

        for (i, result) in reader.records().enumerate() {
            let record = result.map_err(|e| LoaderError::Parse(e.to_string()))?;
            let obj: Map<String, Value> = headers
                .iter()
                .zip(record.iter())
                .map(|(h, v)| (h.clone(), Value::String(v.to_string())))
                .collect();
            let row_str = serde_json::to_string(&obj).unwrap_or_default();
            buf.push_str(&row_str);
            buf.push('\n');

            if buf.len() >= chunk_size {
                docs.push(Document {
                    content: std::mem::take(&mut buf),
                    kind: DocumentKind::Csv,
                    chunk_index: chunk_idx,
                    metadata: Value::Null,
                });
                chunk_idx += 1;
            }
            let _ = i;
        }
        if !buf.is_empty() {
            docs.push(Document {
                content: buf,
                kind: DocumentKind::Csv,
                chunk_index: chunk_idx,
                metadata: Value::Null,
            });
        }
        Ok(docs)
    }
}
