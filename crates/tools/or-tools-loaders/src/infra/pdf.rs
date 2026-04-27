use super::shared::chunk;
use crate::domain::contracts::DocumentLoader;
use crate::domain::entities::{Document, DocumentKind, LoaderRequest, LoaderSource};
use crate::domain::errors::LoaderError;
use async_trait::async_trait;
use lopdf::Document as LoPdf;

pub struct PdfLoader;

#[async_trait]
impl DocumentLoader for PdfLoader {
    fn name(&self) -> &'static str {
        "pdf"
    }

    async fn load(&self, req: LoaderRequest) -> Result<Vec<Document>, LoaderError> {
        let pdf = match &req.source {
            LoaderSource::Path { path } => LoPdf::load(path).map_err(|e| LoaderError::Io {
                path: path.clone(),
                reason: e.to_string(),
            })?,
            LoaderSource::Raw { content } => {
                // Expect base64-encoded bytes for binary PDF.
                let bytes = base64_decode(content)?;
                LoPdf::load_mem(&bytes).map_err(|e| LoaderError::Parse(e.to_string()))?
            }
        };

        let mut text = String::new();
        for (_, page_id) in pdf.get_pages() {
            if let Ok(content) = pdf.extract_text(&[page_id.0]) {
                text.push_str(&content);
                text.push('\n');
            }
        }
        Ok(chunk(text, DocumentKind::Pdf, req.chunk_size))
    }
}

fn base64_decode(s: &str) -> Result<Vec<u8>, LoaderError> {
    use std::io::Read;
    let mut decoder =
        base64::read::DecoderReader::new(s.as_bytes(), &base64::engine::general_purpose::STANDARD);
    let mut out = Vec::new();
    decoder
        .read_to_end(&mut out)
        .map_err(|e| LoaderError::Parse(e.to_string()))?;
    Ok(out)
}
