use super::shared::{build_url, load_credential, transport};
use crate::domain::contracts::CalendarClient;
use crate::domain::entities::CalendarEvent;
use crate::domain::errors::ProductivityError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

const PROVIDER: &str = "gcalendar";
const TOKEN_ENV: &str = "GOOGLE_CALENDAR_ACCESS_TOKEN";
const BASE_URL: &str = "https://www.googleapis.com/calendar/v3/calendars/primary";

pub struct GoogleCalendarClient {
    client: reqwest::Client,
    access_token: String,
    base_url: String,
}

impl GoogleCalendarClient {
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
struct EventList {
    #[serde(default)]
    items: Vec<GEvent>,
}
#[derive(Deserialize)]
struct GEvent {
    id: String,
    #[serde(default)]
    summary: String,
    #[serde(default)]
    description: Option<String>,
    start: GDateTime,
    end: GDateTime,
    #[serde(default)]
    attendees: Vec<GAttendee>,
}
#[derive(Deserialize)]
struct GDateTime {
    #[serde(rename = "dateTime", default)]
    date_time: String,
}
#[derive(Deserialize)]
struct GAttendee {
    email: String,
}

#[async_trait]
impl CalendarClient for GoogleCalendarClient {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn list_events(&self, query: Value) -> Result<Vec<CalendarEvent>, ProductivityError> {
        let time_min = query.get("time_min").and_then(|v| v.as_str()).unwrap_or("");
        let time_max = query.get("time_max").and_then(|v| v.as_str()).unwrap_or("");
        let url = build_url(
            &format!("{}/events", self.base_url),
            &[
                ("timeMin", time_min),
                ("timeMax", time_max),
                ("singleEvents", "true"),
            ],
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
        let list: EventList = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(list
            .items
            .into_iter()
            .map(|e| CalendarEvent {
                id: e.id,
                title: e.summary,
                description: e.description,
                start: e.start.date_time,
                end: e.end.date_time,
                attendees: e.attendees.into_iter().map(|a| a.email).collect(),
            })
            .collect())
    }

    async fn create_event(&self, event: CalendarEvent) -> Result<String, ProductivityError> {
        let url = format!("{}/events", self.base_url);
        let body = json!({
            "summary": event.title,
            "description": event.description,
            "start": { "dateTime": event.start },
            "end": { "dateTime": event.end },
            "attendees": event.attendees.iter().map(|e| json!({ "email": e })).collect::<Vec<_>>()
        });
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
