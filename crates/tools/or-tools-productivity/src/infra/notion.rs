use super::shared::{load_credential, transport};
use crate::domain::contracts::KnowledgeBase;
use crate::domain::entities::Page;
use crate::domain::errors::ProductivityError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

const PROVIDER: &str = "notion";
const API_KEY_ENV: &str = "NOTION_API_KEY";
const DB_ID_ENV: &str = "NOTION_DATABASE_ID";
const BASE_URL: &str = "https://api.notion.com/v1";
const NOTION_VERSION: &str = "2022-06-28";

pub struct NotionBase {
    client: reqwest::Client,
    api_key: String,
    database_id: String,
    base_url: String,
}

impl NotionBase {
    pub fn from_env() -> Result<Self, ProductivityError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: BASE_URL.into(),
            api_key: load_credential(API_KEY_ENV)?,
            database_id: load_credential(DB_ID_ENV)?,
        })
    }

    pub fn with_config(
        client: reqwest::Client,
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        database_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            api_key: api_key.into(),
            database_id: database_id.into(),
        }
    }

    fn auth(&self) -> String {
        format!("Bearer {}", self.api_key)
    }
}

#[derive(Deserialize)]
struct NotionResults {
    results: Vec<NotionPage>,
}
#[derive(Deserialize)]
struct NotionPage {
    id: String,
    #[serde(default)]
    url: Option<String>,
    properties: Value,
}
#[derive(Deserialize)]
struct NotionCreated {
    id: String,
}

fn extract_title(props: &Value) -> String {
    props
        .get("Name")
        .or_else(|| props.get("title"))
        .and_then(|p| p.get("title"))
        .and_then(|t| t.as_array())
        .and_then(|a| a.first())
        .and_then(|e| e.get("plain_text"))
        .and_then(|s| s.as_str())
        .unwrap_or_default()
        .to_string()
}

#[async_trait]
impl KnowledgeBase for NotionBase {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn search(&self, query: Value) -> Result<Vec<Page>, ProductivityError> {
        let filter = query.get("filter").cloned().unwrap_or(json!({}));
        let url = format!("{}/databases/{}/query", self.base_url, self.database_id);
        let body = json!({ "filter": filter, "page_size": 20 });
        let resp = self
            .client
            .post(&url)
            .header("Authorization", self.auth())
            .header("Notion-Version", NOTION_VERSION)
            .json(&body)
            .send()
            .await
            .map_err(transport)?;
        let status = resp.status();
        if !status.is_success() {
            return Err(ProductivityError::Upstream {
                provider: PROVIDER.into(),
                status: status.as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        let results: NotionResults = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(results
            .results
            .into_iter()
            .map(|p| Page {
                id: p.id,
                title: extract_title(&p.properties),
                content: String::new(),
                url: p.url,
            })
            .collect())
    }

    async fn create_page(&self, page: Page) -> Result<String, ProductivityError> {
        let url = format!("{}/pages", self.base_url);
        let body = json!({
            "parent": { "database_id": self.database_id },
            "properties": {
                "Name": { "title": [{ "text": { "content": page.title } }] }
            },
            "children": [{
                "object": "block",
                "type": "paragraph",
                "paragraph": { "rich_text": [{ "text": { "content": page.content } }] }
            }]
        });
        let resp = self
            .client
            .post(&url)
            .header("Authorization", self.auth())
            .header("Notion-Version", NOTION_VERSION)
            .json(&body)
            .send()
            .await
            .map_err(transport)?;
        let status = resp.status();
        if !status.is_success() {
            return Err(ProductivityError::Upstream {
                provider: PROVIDER.into(),
                status: status.as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        let created: NotionCreated = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(created.id)
    }
}
