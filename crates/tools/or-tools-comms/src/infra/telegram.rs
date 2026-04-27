use super::shared::{load_credential, transport};
use crate::domain::contracts::MessageSender;
use crate::domain::entities::{Channel, Message, SendResult};
use crate::domain::errors::CommsError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

const PROVIDER: &str = "telegram";
const BOT_TOKEN_ENV: &str = "TELEGRAM_BOT_TOKEN";
const BASE_URL: &str = "https://api.telegram.org/bot";

pub struct TelegramSender {
    client: reqwest::Client,
    bot_token: String,
    base_url: String,
}

impl TelegramSender {
    pub fn from_env() -> Result<Self, CommsError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: BASE_URL.into(),
            bot_token: load_credential(BOT_TOKEN_ENV)?,
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
}

#[derive(Deserialize)]
struct TgResult {
    message_id: i64,
}
#[derive(Deserialize)]
struct TgResponse {
    result: TgResult,
}

#[async_trait]
impl MessageSender for TelegramSender {
    fn channel(&self) -> &'static str {
        PROVIDER
    }

    async fn send(&self, msg: Message) -> Result<SendResult, CommsError> {
        let url = format!("{}{}/sendMessage", self.base_url, self.bot_token);
        let body = json!({ "chat_id": msg.to, "text": msg.body });
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
        let parsed: TgResponse = resp
            .json()
            .await
            .map_err(|e| CommsError::Transport(e.to_string()))?;
        Ok(SendResult {
            message_id: parsed.result.message_id.to_string(),
            channel: Channel::Telegram,
            status: "sent".into(),
        })
    }
}
