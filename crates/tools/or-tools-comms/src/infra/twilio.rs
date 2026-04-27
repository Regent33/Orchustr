use super::shared::{load_credential, transport};
use crate::domain::contracts::MessageSender;
use crate::domain::entities::{Channel, Message, SendResult};
use crate::domain::errors::CommsError;
use async_trait::async_trait;
use serde::Deserialize;

const PROVIDER: &str = "sms";
const ACCOUNT_SID_ENV: &str = "TWILIO_ACCOUNT_SID";
const AUTH_TOKEN_ENV: &str = "TWILIO_AUTH_TOKEN";
const FROM_ENV: &str = "TWILIO_FROM";
const BASE_URL: &str = "https://api.twilio.com/2010-04-01/Accounts";

pub struct TwilioSender {
    client: reqwest::Client,
    account_sid: String,
    auth_token: String,
    from: String,
}

impl TwilioSender {
    pub fn from_env() -> Result<Self, CommsError> {
        Ok(Self {
            client: reqwest::Client::new(),
            account_sid: load_credential(ACCOUNT_SID_ENV)?,
            auth_token: load_credential(AUTH_TOKEN_ENV)?,
            from: load_credential(FROM_ENV)?,
        })
    }

    pub fn with_config(
        client: reqwest::Client,
        account_sid: impl Into<String>,
        auth_token: impl Into<String>,
        from: impl Into<String>,
    ) -> Self {
        Self {
            client,
            account_sid: account_sid.into(),
            auth_token: auth_token.into(),
            from: from.into(),
        }
    }
}

#[derive(Deserialize)]
struct TwilioResponse {
    sid: String,
    status: String,
}

#[async_trait]
impl MessageSender for TwilioSender {
    fn channel(&self) -> &'static str {
        PROVIDER
    }

    async fn send(&self, msg: Message) -> Result<SendResult, CommsError> {
        let url = format!("{}/{}/Messages.json", BASE_URL, self.account_sid);
        let from = msg.from.as_deref().unwrap_or(&self.from).to_string();
        let body = {
            let mut form = url::form_urlencoded::Serializer::new(String::new());
            form.append_pair("To", &msg.to);
            form.append_pair("From", &from);
            form.append_pair("Body", &msg.body);
            form.finish()
        };
        let resp = self
            .client
            .post(&url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
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
        let parsed: TwilioResponse = resp
            .json()
            .await
            .map_err(|e| CommsError::Transport(e.to_string()))?;
        Ok(SendResult {
            message_id: parsed.sid,
            channel: Channel::Sms,
            status: parsed.status,
        })
    }
}
