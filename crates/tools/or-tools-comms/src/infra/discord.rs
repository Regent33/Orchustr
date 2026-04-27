use super::shared::{load_credential, transport};
use crate::domain::contracts::MessageSender;
use crate::domain::entities::{Channel, Message, SendResult};
use crate::domain::errors::CommsError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

const PROVIDER: &str = "discord";
const TOKEN_ENV: &str = "DISCORD_BOT_TOKEN";
const BASE_URL: &str = "https://discord.com/api/v10";

pub struct DiscordSender {
    client: reqwest::Client,
    bot_token: String,
    base_url: String,
}

impl DiscordSender {
    pub fn from_env() -> Result<Self, CommsError> {
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
}

#[derive(Deserialize)]
struct DiscordMessage {
    id: String,
}

#[async_trait]
impl MessageSender for DiscordSender {
    fn channel(&self) -> &'static str {
        PROVIDER
    }

    async fn send(&self, msg: Message) -> Result<SendResult, CommsError> {
        let url = format!("{}/channels/{}/messages", self.base_url, msg.to);
        let body = json!({ "content": msg.body });
        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bot {}", self.bot_token))
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
        let parsed: DiscordMessage = resp
            .json()
            .await
            .map_err(|e| CommsError::Transport(e.to_string()))?;
        Ok(SendResult {
            message_id: parsed.id,
            channel: Channel::Discord,
            status: "sent".into(),
        })
    }
}
