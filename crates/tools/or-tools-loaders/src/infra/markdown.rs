use super::shared::{chunk, read_source};
use crate::domain::contracts::DocumentLoader;
use crate::domain::entities::{Document, DocumentKind, LoaderRequest};
use crate::domain::errors::LoaderError;
use async_trait::async_trait;

pub struct MarkdownLoader;

#[async_trait]
impl DocumentLoader for MarkdownLoader {
    fn name(&self) -> &'static str {
        "markdown"
    }

    async fn load(&self, req: LoaderRequest) -> Result<Vec<Document>, LoaderError> {
        let content = read_source(&req).await?;
        // Strip YAML front-matter (--- ... ---) for clean RAG ingestion.
        let clean = strip_frontmatter(&content);
        Ok(chunk(
            clean.to_string(),
            DocumentKind::Markdown,
            req.chunk_size,
        ))
    }
}

fn strip_frontmatter(s: &str) -> &str {
    if !s.starts_with("---") {
        return s;
    }
    let rest = &s[3..];
    if let Some(end) = rest.find("\n---") {
        return rest[end + 4..].trim_start_matches('\n');
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_yaml_front_matter() {
        let md = "---\ntitle: foo\n---\n# Heading";
        assert_eq!(strip_frontmatter(md), "# Heading");
    }

    #[test]
    fn no_frontmatter_passthrough() {
        let md = "# Just heading";
        assert_eq!(strip_frontmatter(md), md);
    }
}
