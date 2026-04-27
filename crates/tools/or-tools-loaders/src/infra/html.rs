use super::shared::{chunk, read_source};
use crate::domain::contracts::DocumentLoader;
use crate::domain::entities::{Document, DocumentKind, LoaderRequest};
use crate::domain::errors::LoaderError;
use async_trait::async_trait;

pub struct HtmlLoader;

#[async_trait]
impl DocumentLoader for HtmlLoader {
    fn name(&self) -> &'static str {
        "html"
    }

    async fn load(&self, req: LoaderRequest) -> Result<Vec<Document>, LoaderError> {
        let raw = read_source(&req).await?;
        let text = strip_tags(&raw);
        Ok(chunk(text, DocumentKind::Html, req.chunk_size))
    }
}

/// Strips HTML tags and collapses whitespace into readable text.
/// This zero-dependency approach avoids build-script restrictions and is
/// sufficient for RAG ingestion where perfect rendering is not required.
fn strip_tags(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    let mut in_script_or_style = false;
    let mut tag_buf = String::new();

    for ch in html.chars() {
        match ch {
            '<' => {
                // Insert a space at every tag boundary to prevent word merging.
                if !in_script_or_style && !in_tag && out.ends_with(|ch: char| !ch.is_whitespace()) {
                    out.push(' ');
                }
                in_tag = true;
                tag_buf.clear();
            }
            '>' => {
                let tag_lower = tag_buf.trim().to_lowercase();
                if tag_lower.starts_with("script") || tag_lower.starts_with("style") {
                    in_script_or_style = true;
                } else if tag_lower.starts_with("/script") || tag_lower.starts_with("/style") {
                    in_script_or_style = false;
                }
                in_tag = false;
            }
            c if in_tag => tag_buf.push(c),
            c if !in_script_or_style => out.push(c),
            _ => {}
        }
    }
    // Collapse runs of whitespace
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_simple_tags() {
        let html = "<h1>Hello</h1><p>World</p>";
        assert_eq!(strip_tags(html), "Hello World");
    }

    #[test]
    fn skips_script_content() {
        let html = "<p>Visible</p><script>alert(1)</script><p>Also visible</p>";
        let result = strip_tags(html);
        assert!(result.contains("Visible"));
        assert!(result.contains("Also visible"));
        assert!(!result.contains("alert"));
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(strip_tags(""), "");
    }
}
