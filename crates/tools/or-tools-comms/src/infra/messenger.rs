use super::shared::{load_credential, transport};
use crate::domain::contracts::MessageSender;
use crate::domain::entities::{Channel, Message, SendResult};
use crate::domain::errors::CommsError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

const PROVIDER: &str = "messenger";
const TOKEN_ENV: &str = "MESSENGER_PAGE_ACCESS_TOKEN";
const BASE_URL: &str = "https://graph.facebook.com/v18.0/me/messages";

pub struct MessengerSender {
    client: reqwest::Client,
    page_token: String,
    base_url: String,
}

impl MessengerSender {
    pub fn from_env() -> Result<Self, CommsError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: BASE_URL.into(),
            page_token: load_credential(TOKEN_ENV)?,
        })
    }

    pub fn with_config(
        client: reqwest::Client,
        base_url: impl Into<String>,
        page_token: impl Into<String>,
    ) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            page_token: page_token.into(),
        }
    }
}

#[derive(Deserialize)]
struct MsgResponse {
    message_id: String,
}

#[async_trait]
impl MessageSender for MessengerSender {
    fn channel(&self) -> &'static str {
        PROVIDER
    }

    async fn send(&self, msg: Message) -> Result<SendResult, CommsError> {
        let body = json!({
            "recipient": { "id": msg.to },
            "message": { "text": msg.body }
        });
        let url = format!("{}?access_token={}", self.base_url, self.page_token);
        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(transport)?;
        let status = resp.status().as_u16();
        if !(200..300).contains(&status) {
            return Err(CommsError::Upstream {
                provider: PROVIDER.into(),
                status,
                body: resp.text().await.unwrap_or_default(),
            });
        }
        let parsed: MsgResponse = resp
            .json()
            .await
            .map_err(|e| CommsError::Transport(e.to_string()))?;
        Ok(SendResult {
            message_id: parsed.message_id,
            channel: Channel::Messenger,
            status: "sent".into(),
        })
    }
}
