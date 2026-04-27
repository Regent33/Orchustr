use super::shared::{build_url, load_credential, transport};
use crate::domain::contracts::ProjectTracker;
use crate::domain::entities::Issue;
use crate::domain::errors::ProductivityError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

const PROVIDER: &str = "clickup";
const API_KEY_ENV: &str = "CLICKUP_API_KEY";
const LIST_ID_ENV: &str = "CLICKUP_LIST_ID";
const BASE_URL: &str = "https://api.clickup.com/api/v2";

pub struct ClickUpTracker {
    client: reqwest::Client,
    api_key: String,
    list_id: String,
    base_url: String,
}

impl ClickUpTracker {
    pub fn from_env() -> Result<Self, ProductivityError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: BASE_URL.into(),
            api_key: load_credential(API_KEY_ENV)?,
            list_id: load_credential(LIST_ID_ENV)?,
        })
    }

    pub fn with_config(
        client: reqwest::Client,
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        list_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            api_key: api_key.into(),
            list_id: list_id.into(),
        }
    }
}

#[derive(Deserialize)]
struct TaskList {
    tasks: Vec<CuTask>,
}
#[derive(Deserialize)]
struct CuTask {
    id: String,
    name: String,
    #[serde(default)]
    description: Option<String>,
    status: CuStatus,
    #[serde(default)]
    assignees: Vec<CuUser>,
    #[serde(default)]
    tags: Vec<CuTag>,
}
#[derive(Deserialize)]
struct CuStatus {
    status: String,
}
#[derive(Deserialize)]
struct CuUser {
    username: String,
}
#[derive(Deserialize)]
struct CuTag {
    name: String,
}
#[derive(Deserialize)]
struct CuCreated {
    id: String,
}

#[async_trait]
impl ProjectTracker for ClickUpTracker {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn list_issues(&self, _query: Value) -> Result<Vec<Issue>, ProductivityError> {
        let url = build_url(
            &format!("{}/list/{}/task", self.base_url, self.list_id),
            &[("page", "0")],
        )?;
        let resp = self
            .client
            .get(url)
            .header("Authorization", &self.api_key)
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
        let list: TaskList = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(list
            .tasks
            .into_iter()
            .map(|t| Issue {
                id: t.id,
                title: t.name,
                description: t.description,
                status: t.status.status,
                assignee: t.assignees.into_iter().next().map(|u| u.username),
                labels: t.tags.into_iter().map(|tg| tg.name).collect(),
            })
            .collect())
    }

    async fn create_issue(&self, issue: Issue) -> Result<String, ProductivityError> {
        let url = format!("{}/list/{}/task", self.base_url, self.list_id);
        let body = json!({
            "name": issue.title,
            "description": issue.description.unwrap_or_default(),
            "tags": issue.labels
        });
        let resp = self
            .client
            .post(&url)
            .header("Authorization", &self.api_key)
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
        let created: CuCreated = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(created.id)
    }
}
