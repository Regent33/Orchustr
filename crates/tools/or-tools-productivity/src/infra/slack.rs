use super::shared::{load_credential, transport};
use crate::domain::contracts::TeamMessenger;
use crate::domain::entities::Page;
use crate::domain::errors::ProductivityError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

const PROVIDER: &str = "slack";
const TOKEN_ENV: &str = "SLACK_BOT_TOKEN";
const BASE_URL: &str = "https://slack.com/api";

pub struct SlackMessenger {
    client: reqwest::Client,
    bot_token: String,
    base_url: String,
}

impl SlackMessenger {
    pub fn from_env() -> Result<Self, ProductivityError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: BASE_URL.into(),
            bot_token: load_credential(TOKEN_ENV)?,
        })
    }

    pub fn with_config(
        client: reqwest::Client,
        base_url: impl Into<String>,
        bot_token: impl Into<String>,
    ) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            bot_token: bot_token.into(),
        }
    }

    fn auth(&self) -> String {
        format!("Bearer {}", self.bot_token)
    }
}

#[derive(Deserialize)]
struct PostMessageResponse {
    ok: bool,
    #[serde(default)]
    ts: String,
    #[serde(default)]
    error: String,
}

#[derive(Deserialize)]
struct SearchResponse {
    ok: bool,
    #[serde(default)]
    messages: SearchMessages,
}
#[derive(Deserialize, Default)]
struct SearchMessages {
    #[serde(default)]
    matches: Vec<SearchMatch>,
}
#[derive(Deserialize)]
struct SearchMatch {
    #[serde(default)]
    text: String,
    #[serde(default)]
    permalink: String,
}

#[async_trait]
impl TeamMessenger for SlackMessenger {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn post(&self, channel: &str, text: &str) -> Result<String, ProductivityError> {
        let url = format!("{}/chat.postMessage", self.base_url);
        let body = json!({ "channel": channel, "text": text });
        let resp = self
            .client
            .post(&url)
            .header("Authorization", self.auth())
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
        let parsed: PostMessageResponse = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        if !parsed.ok {
            return Err(ProductivityError::Upstream {
                provider: PROVIDER.into(),
                status: 200,
                body: parsed.error,
            });
        }
        Ok(parsed.ts)
    }

    async fn search_messages(&self, query: Value) -> Result<Vec<Page>, ProductivityError> {
        let q = query.get("q").and_then(|v| v.as_str()).unwrap_or("");
        let url = format!("{}/search.messages", self.base_url);
        let body = json!({ "query": q });
        let resp = self
            .client
            .post(&url)
            .header("Authorization", self.auth())
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
        let parsed: SearchResponse = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        if !parsed.ok {
            return Ok(Vec::new());
        }
        Ok(parsed
            .messages
            .matches
            .into_iter()
            .map(|m| Page {
                id: m.permalink.clone(),
                title: m.text.chars().take(60).collect(),
                content: m.text,
                url: Some(m.permalink),
            })
            .collect())
    }
}
