use super::shared::{load_credential, transport};
use crate::domain::contracts::MessageSender;
use crate::domain::entities::{Channel, Message, SendResult};
use crate::domain::errors::CommsError;
use async_trait::async_trait;
use serde::Deserialize;

const PROVIDER: &str = "facebook";
const TOKEN_ENV: &str = "FACEBOOK_PAGE_ACCESS_TOKEN";
const BASE_URL: &str = "https://graph.facebook.com/v18.0";

pub struct FacebookSender {
    client: reqwest::Client,
    page_token: String,
    base_url: String,
}

impl FacebookSender {
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
struct FbResponse {
    id: String,
}

#[async_trait]
impl MessageSender for FacebookSender {
    fn channel(&self) -> &'static str {
        PROVIDER
    }

    async fn send(&self, msg: Message) -> Result<SendResult, CommsError> {
        let url = format!("{}/{}/feed", self.base_url, msg.to);
        let body = {
            let mut form = url::form_urlencoded::Serializer::new(String::new());
            form.append_pair("message", &msg.body);
            form.append_pair("access_token", &self.page_token);
            form.finish()
        };
        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
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
        let parsed: FbResponse = resp
            .json()
            .await
            .map_err(|e| CommsError::Transport(e.to_string()))?;
        Ok(SendResult {
            message_id: parsed.id,
            channel: Channel::Facebook,
            status: "posted".into(),
        })
    }
}
