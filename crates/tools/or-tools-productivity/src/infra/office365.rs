use super::shared::{load_credential, transport};
use crate::domain::contracts::{CalendarClient, EmailClient};
use crate::domain::entities::{CalendarEvent, Email};
use crate::domain::errors::ProductivityError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

const PROVIDER: &str = "office365";
const TOKEN_ENV: &str = "OFFICE365_ACCESS_TOKEN";
const BASE_URL: &str = "https://graph.microsoft.com/v1.0/me";

struct Office365Client {
    client: reqwest::Client,
    access_token: String,
    base_url: String,
}

impl Office365Client {
    fn from_env() -> Result<Self, ProductivityError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: BASE_URL.into(),
            access_token: load_credential(TOKEN_ENV)?,
        })
    }

    fn with_config(
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

// ── Email ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct GraphMessages {
    value: Vec<GraphMessage>,
}
#[derive(Deserialize)]
struct GraphMessage {
    id: String,
    subject: String,
    #[serde(rename = "bodyPreview", default)]
    body_preview: String,
    from: GraphFrom,
    #[serde(rename = "receivedDateTime", default)]
    received: String,
    #[serde(rename = "toRecipients", default)]
    to_recipients: Vec<GraphRecipient>,
}
#[derive(Deserialize)]
struct GraphFrom {
    #[serde(rename = "emailAddress")]
    email_address: GraphEmail,
}
#[derive(Deserialize)]
struct GraphRecipient {
    #[serde(rename = "emailAddress")]
    email_address: GraphEmail,
}
#[derive(Deserialize)]
struct GraphEmail {
    address: String,
}

pub struct OutlookEmailClient(Office365Client);

impl OutlookEmailClient {
    pub fn from_env() -> Result<Self, ProductivityError> {
        Ok(Self(Office365Client::from_env()?))
    }
    pub fn with_config(
        client: reqwest::Client,
        base_url: impl Into<String>,
        access_token: impl Into<String>,
    ) -> Self {
        Self(Office365Client::with_config(client, base_url, access_token))
    }
}

#[async_trait]
impl EmailClient for OutlookEmailClient {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn list(&self, query: Value) -> Result<Vec<Email>, ProductivityError> {
        let top = query
            .get("top")
            .and_then(|v| v.as_u64())
            .unwrap_or(20)
            .to_string();
        let url = format!(
            "{}{}messages?$top={}",
            self.0.base_url,
            if self.0.base_url.ends_with('/') {
                ""
            } else {
                "/"
            },
            top
        );
        let resp = self
            .0
            .client
            .get(&url)
            .header("Authorization", self.0.auth())
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
        let msgs: GraphMessages = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(msgs
            .value
            .into_iter()
            .map(|m| Email {
                id: m.id,
                from: m.from.email_address.address,
                to: m
                    .to_recipients
                    .into_iter()
                    .map(|r| r.email_address.address)
                    .collect(),
                subject: m.subject,
                body: m.body_preview,
                timestamp: Some(m.received),
            })
            .collect())
    }

    async fn send_email(&self, email: Email) -> Result<String, ProductivityError> {
        let url = format!("{}/sendMail", self.0.base_url);
        let body = json!({
            "message": {
                "subject": email.subject,
                "body": { "contentType": "Text", "content": email.body },
                "toRecipients": email.to.iter().map(|e| json!({ "emailAddress": { "address": e } })).collect::<Vec<_>>()
            }
        });
        let resp = self
            .0
            .client
            .post(&url)
            .header("Authorization", self.0.auth())
            .json(&body)
            .send()
            .await
            .map_err(transport)?;
        let status = resp.status().as_u16();
        if !(200..300).contains(&status) {
            return Err(ProductivityError::Upstream {
                provider: PROVIDER.into(),
                status,
                body: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(email.id)
    }
}

// ── Calendar ──────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct GraphEvents {
    value: Vec<GraphEvent>,
}
#[derive(Deserialize)]
struct GraphEvent {
    id: String,
    subject: String,
    #[serde(rename = "bodyPreview", default)]
    body_preview: String,
    start: GraphDatetime,
    end: GraphDatetime,
    #[serde(default)]
    attendees: Vec<GraphAttendee>,
}
#[derive(Deserialize)]
struct GraphDatetime {
    #[serde(rename = "dateTime")]
    date_time: String,
}
#[derive(Deserialize)]
struct GraphAttendee {
    #[serde(rename = "emailAddress")]
    email_address: GraphEmail,
}
#[derive(Deserialize)]
struct GraphCreated {
    id: String,
}

pub struct OutlookCalendarClient(Office365Client);

impl OutlookCalendarClient {
    pub fn from_env() -> Result<Self, ProductivityError> {
        Ok(Self(Office365Client::from_env()?))
    }
    pub fn with_config(
        client: reqwest::Client,
        base_url: impl Into<String>,
        access_token: impl Into<String>,
    ) -> Self {
        Self(Office365Client::with_config(client, base_url, access_token))
    }
}

#[async_trait]
impl CalendarClient for OutlookCalendarClient {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn list_events(&self, query: Value) -> Result<Vec<CalendarEvent>, ProductivityError> {
        let top = query
            .get("top")
            .and_then(|v| v.as_u64())
            .unwrap_or(20)
            .to_string();
        let url = format!("{}/events?$top={}", self.0.base_url, top);
        let resp = self
            .0
            .client
            .get(&url)
            .header("Authorization", self.0.auth())
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
        let events: GraphEvents = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(events
            .value
            .into_iter()
            .map(|e| CalendarEvent {
                id: e.id,
                title: e.subject,
                description: Some(e.body_preview),
                start: e.start.date_time,
                end: e.end.date_time,
                attendees: e
                    .attendees
                    .into_iter()
                    .map(|a| a.email_address.address)
                    .collect(),
            })
            .collect())
    }

    async fn create_event(&self, event: CalendarEvent) -> Result<String, ProductivityError> {
        let url = format!("{}/events", self.0.base_url);
        let body = json!({
            "subject": event.title,
            "body": { "contentType": "Text", "content": event.description.unwrap_or_default() },
            "start": { "dateTime": event.start, "timeZone": "UTC" },
            "end": { "dateTime": event.end, "timeZone": "UTC" },
            "attendees": event.attendees.iter().map(|e| json!({ "emailAddress": { "address": e }, "type": "required" })).collect::<Vec<_>>()
        });
        let resp = self
            .0
            .client
            .post(&url)
            .header("Authorization", self.0.auth())
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
        let created: GraphCreated = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(created.id)
    }
}
