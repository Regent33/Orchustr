use super::shared::{load_credential, transport};
use crate::domain::contracts::MessageSender;
use crate::domain::entities::{Channel, Message, SendResult};
use crate::domain::errors::CommsError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

const PROVIDER: &str = "whatsapp";
const TOKEN_ENV: &str = "WHATSAPP_ACCESS_TOKEN";
const PHONE_ID_ENV: &str = "WHATSAPP_PHONE_NUMBER_ID";
const BASE_URL: &str = "https://graph.facebook.com/v18.0";

pub struct WhatsAppSender {
    client: reqwest::Client,
    access_token: String,
    phone_id: String,
    base_url: String,
}

impl WhatsAppSender {
    pub fn from_env() -> Result<Self, CommsError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: BASE_URL.into(),
            access_token: load_credential(TOKEN_ENV)?,
            phone_id: load_credential(PHONE_ID_ENV)?,
        })
    }

    pub fn with_config(
        client: reqwest::Client,
        base_url: impl Into<String>,
        access_token: impl Into<String>,
        phone_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            access_token: access_token.into(),
            phone_id: phone_id.into(),
        }
    }
}

#[derive(Deserialize)]
struct WaMessage {
    id: String,
}
#[derive(Deserialize)]
struct WaResponse {
    messages: Vec<WaMessage>,
}

#[async_trait]
impl MessageSender for WhatsAppSender {
    fn channel(&self) -> &'static str {
        PROVIDER
    }

    async fn send(&self, msg: Message) -> Result<SendResult, CommsError> {
        let url = format!("{}/{}/messages", self.base_url, self.phone_id);
        let body = json!({
            "messaging_product": "whatsapp",
            "to": msg.to,
            "type": "text",
            "text": { "body": msg.body }
        });
        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
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
        let parsed: WaResponse = resp
            .json()
            .await
            .map_err(|e| CommsError::Transport(e.to_string()))?;
        let id = parsed
            .messages
            .into_iter()
            .next()
            .map(|m| m.id)
            .unwrap_or_default();
        Ok(SendResult {
            message_id: id,
            channel: Channel::WhatsApp,
            status: "sent".into(),
        })
    }
}
