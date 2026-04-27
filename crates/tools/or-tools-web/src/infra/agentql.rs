use super::shared::{load_credential, transport};
use crate::domain::contracts::Scraper;
use crate::domain::entities::ScrapedPage;
use crate::domain::errors::WebError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

const PROVIDER: &str = "agentql";
const API_KEY_ENV: &str = "AGENTQL_API_KEY";
const DEFAULT_URL: &str = "https://api.agentql.com/v1/query-data";

/// AgentQL — natural-language selectors for web scraping. Users supply a URL
/// and optional prompt; AgentQL returns a structured extraction.
#[derive(Clone)]
pub struct AgentQlScraper {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
    prompt: String,
}

impl AgentQlScraper {
    pub fn from_env() -> Result<Self, WebError> {
        Ok(Self {
            client: reqwest::Client::new(),
            endpoint: DEFAULT_URL.to_string(),
            api_key: load_credential(API_KEY_ENV)?,
            prompt: "Extract main article text, title, and outbound links".into(),
        })
    }

    #[must_use]
    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = prompt.into();
        self
    }
}

#[derive(Debug, Deserialize)]
struct AgentQlResponse {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    text: String,
    #[serde(default)]
    links: Vec<String>,
}

#[async_trait]
impl Scraper for AgentQlScraper {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn scrape(&self, url: &str) -> Result<ScrapedPage, WebError> {
        let body = json!({ "url": url, "prompt": self.prompt });
        let response = self
            .client
            .post(&self.endpoint)
            .header("X-API-Key", &self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(WebError::Upstream {
                provider: PROVIDER.into(),
                status: status.as_u16(),
                body,
            });
        }
        let parsed: AgentQlResponse = response
            .json()
            .await
            .map_err(|e| WebError::HtmlParse(e.to_string()))?;
        Ok(ScrapedPage {
            url: url.to_string(),
            title: parsed.title,
            text: parsed.text,
            links: parsed.links,
        })
    }
}
