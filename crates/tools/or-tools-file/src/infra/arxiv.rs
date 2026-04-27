use super::shared::transport;
use crate::domain::contracts::DataSource;
use crate::domain::entities::ResearchPaper;
use crate::domain::errors::FileError;
use async_trait::async_trait;
use serde_json::Value;
use url::Url;

const PROVIDER: &str = "arxiv";
const DEFAULT_URL: &str = "https://export.arxiv.org/api/query";

pub struct ArxivSource {
    client: reqwest::Client,
    endpoint: String,
}

impl ArxivSource {
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint: DEFAULT_URL.to_string(),
        }
    }

    #[must_use]
    pub fn with_endpoint(client: reqwest::Client, endpoint: impl Into<String>) -> Self {
        Self {
            client,
            endpoint: endpoint.into(),
        }
    }
}

impl Default for ArxivSource {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DataSource for ArxivSource {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn fetch(&self, query: Value) -> Result<Value, FileError> {
        let q = query.get("query").and_then(|v| v.as_str()).unwrap_or("");
        let max = query
            .get("max_results")
            .and_then(|v| v.as_u64())
            .unwrap_or(5);
        let mut url =
            Url::parse(&self.endpoint).map_err(|e| FileError::Transport(e.to_string()))?;
        url.query_pairs_mut()
            .append_pair("search_query", q)
            .append_pair("max_results", &max.to_string());
        let resp = self.client.get(url).send().await.map_err(transport)?;
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(FileError::Upstream {
                provider: PROVIDER.into(),
                status: status.as_u16(),
                body,
            });
        }
        // Parse Atom XML minimally — extract entry titles and IDs as JSON
        let papers = parse_atom(&body);
        Ok(serde_json::to_value(papers).unwrap_or(Value::Null))
    }
}

fn parse_atom(xml: &str) -> Vec<ResearchPaper> {
    let mut papers = Vec::new();
    for entry in xml.split("<entry>").skip(1) {
        let id = extract_tag(entry, "id").unwrap_or_default();
        let title = extract_tag(entry, "title").unwrap_or_default();
        let summary = extract_tag(entry, "summary").unwrap_or_default();
        let published = extract_tag(entry, "published");
        let pdf_url = format!("{}.pdf", id.trim().replace("/abs/", "/pdf/"));
        papers.push(ResearchPaper {
            id: id.trim().to_string(),
            title: title.trim().to_string(),
            authors: Vec::new(),
            summary: summary.trim().to_string(),
            pdf_url,
            published,
            categories: Vec::new(),
        });
    }
    papers
}

fn extract_tag(s: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = s.find(&open)? + open.len();
    let end = s.find(&close)?;
    Some(s[start..end].to_string())
}
