use super::shared::{build_url, load_credential, transport};
use crate::domain::contracts::ProjectTracker;
use crate::domain::entities::Issue;
use crate::domain::errors::ProductivityError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;

const PROVIDER: &str = "trello";
const API_KEY_ENV: &str = "TRELLO_API_KEY";
const TOKEN_ENV: &str = "TRELLO_TOKEN";
const LIST_ID_ENV: &str = "TRELLO_LIST_ID";
const BASE_URL: &str = "https://api.trello.com/1";

pub struct TrelloTracker {
    client: reqwest::Client,
    api_key: String,
    token: String,
    list_id: String,
    base_url: String,
}

impl TrelloTracker {
    pub fn from_env() -> Result<Self, ProductivityError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: BASE_URL.into(),
            api_key: load_credential(API_KEY_ENV)?,
            token: load_credential(TOKEN_ENV)?,
            list_id: load_credential(LIST_ID_ENV)?,
        })
    }

    pub fn with_config(
        client: reqwest::Client,
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        token: impl Into<String>,
        list_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            api_key: api_key.into(),
            token: token.into(),
            list_id: list_id.into(),
        }
    }
}

#[derive(Deserialize)]
struct TrelloCard {
    id: String,
    name: String,
    #[serde(default)]
    desc: String,
}

#[async_trait]
impl ProjectTracker for TrelloTracker {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn list_issues(&self, _query: Value) -> Result<Vec<Issue>, ProductivityError> {
        let url = build_url(
            &format!("{}/lists/{}/cards", self.base_url, self.list_id),
            &[("key", &self.api_key), ("token", &self.token)],
        )?;
        let resp = self.client.get(url).send().await.map_err(transport)?;
        let status = resp.status();
        if !status.is_success() {
            return Err(ProductivityError::Upstream {
                provider: PROVIDER.into(),
                status: status.as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        let cards: Vec<TrelloCard> = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(cards
            .into_iter()
            .map(|c| Issue {
                id: c.id,
                title: c.name,
                description: if c.desc.is_empty() {
                    None
                } else {
                    Some(c.desc)
                },
                status: "open".into(),
                assignee: None,
                labels: Vec::new(),
            })
            .collect())
    }

    async fn create_issue(&self, issue: Issue) -> Result<String, ProductivityError> {
        let url = build_url(
            &format!("{}/cards", self.base_url),
            &[
                ("key", &self.api_key),
                ("token", &self.token),
                ("idList", &self.list_id),
                ("name", &issue.title),
                ("desc", issue.description.as_deref().unwrap_or("")),
            ],
        )?;
        let resp = self.client.post(url).send().await.map_err(transport)?;
        let status = resp.status();
        if !status.is_success() {
            return Err(ProductivityError::Upstream {
                provider: PROVIDER.into(),
                status: status.as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        let card: TrelloCard = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(card.id)
    }
}
