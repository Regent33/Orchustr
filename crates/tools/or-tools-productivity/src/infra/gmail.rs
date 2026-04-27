use super::shared::{load_credential, transport};
use crate::domain::contracts::EmailClient;
use crate::domain::entities::Email;
use crate::domain::errors::ProductivityError;
use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::Deserialize;
use serde_json::{Value, json};

const PROVIDER: &str = "gmail";
const TOKEN_ENV: &str = "GMAIL_ACCESS_TOKEN";
const BASE_URL: &str = "https://gmail.googleapis.com/gmail/v1/users/me";

pub struct GmailClient {
    client: reqwest::Client,
    access_token: String,
    base_url: String,
}

impl GmailClient {
    pub fn from_env() -> Result<Self, ProductivityError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: BASE_URL.into(),
            access_token: load_credential(TOKEN_ENV)?,
        })
    }

    pub fn with_config(
        client: reqwest::Client,
        base_url: impl Into<String>,
        access_token: impl Into<String>,
    ) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            access_token: access_token.into(),
        }
    }

    fn auth(&self) -> String {
        format!("Bearer {}", self.access_token)
    }
}

#[derive(Deserialize)]
struct MessageListResponse {
    #[serde(default)]
    messages: Vec<MessageRef>,
}
#[derive(Deserialize)]
struct MessageRef {
    id: String,
}
#[derive(Deserialize)]
struct MessagePayload {
    headers: Vec<Header>,
}
#[derive(Deserialize)]
struct Header {
    name: String,
    value: String,
}
#[derive(Deserialize)]
struct MessageDetail {
    id: String,
    payload: MessagePayload,
    #[serde(default)]
    snippet: String,
}

fn header_val(headers: &[Header], name: &str) -> String {
    headers
        .iter()
        .find(|h| h.name.eq_ignore_ascii_case(name))
        .map(|h| h.value.clone())
        .unwrap_or_default()
}

#[async_trait]
impl EmailClient for GmailClient {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn list(&self, query: Value) -> Result<Vec<Email>, ProductivityError> {
        let q = query.get("q").and_then(|v| v.as_str()).unwrap_or("");
        let max = query
            .get("max_results")
            .and_then(|v| v.as_u64())
            .unwrap_or(10)
            .to_string();
        let url = super::shared::build_url(
            &format!("{}/messages", self.base_url),
            &[("q", q), ("maxResults", &max)],
        )?;
        let resp = self
            .client
            .get(url)
            .header("Authorization", self.auth())
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
        let list: MessageListResponse = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        let mut emails = Vec::new();
        for r in list.messages.into_iter().take(5) {
            let url2 = format!("{}/messages/{}", self.base_url, r.id);
            let r2 = self
                .client
                .get(&url2)
                .header("Authorization", self.auth())
                .send()
                .await
                .map_err(transport)?;
            if !r2.status().is_success() {
                continue;
            }
            if let Ok(detail) = r2.json::<MessageDetail>().await {
                let headers = &detail.payload.headers;
                emails.push(Email {
                    id: detail.id,
                    from: header_val(headers, "From"),
                    to: vec![header_val(headers, "To")],
                    subject: header_val(headers, "Subject"),
                    body: detail.snippet,
                    timestamp: None,
                });
            }
        }
        Ok(emails)
    }

    async fn send_email(&self, email: Email) -> Result<String, ProductivityError> {
        let raw = format!(
            "From: {}\r\nTo: {}\r\nSubject: {}\r\n\r\n{}",
            email.from,
            email.to.join(","),
            email.subject,
            email.body
        );
        let encoded = STANDARD.encode(raw.as_bytes());
        let url = format!("{}/messages/send", self.base_url);
        let body = json!({ "raw": encoded });
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
        let v: Value = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(v.get("id")
            .and_then(|i| i.as_str())
            .unwrap_or_default()
            .into())
    }
}
