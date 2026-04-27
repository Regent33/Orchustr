use super::shared::{load_credential, transport};
use crate::domain::contracts::ProjectTracker;
use crate::domain::entities::Issue;
use crate::domain::errors::ProductivityError;
use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::Deserialize;
use serde_json::{Value, json};

const PROVIDER: &str = "jira";
const BASE_URL_ENV: &str = "JIRA_BASE_URL";
const EMAIL_ENV: &str = "JIRA_EMAIL";
const API_TOKEN_ENV: &str = "JIRA_API_TOKEN";

pub struct JiraTracker {
    client: reqwest::Client,
    base_url: String,
    auth_header: String,
}

impl JiraTracker {
    pub fn from_env() -> Result<Self, ProductivityError> {
        let base_url = load_credential(BASE_URL_ENV)?;
        let email = load_credential(EMAIL_ENV)?;
        let token = load_credential(API_TOKEN_ENV)?;
        let auth_header = format!("Basic {}", STANDARD.encode(format!("{email}:{token}")));
        Ok(Self {
            client: reqwest::Client::new(),
            base_url,
            auth_header,
        })
    }

    pub fn with_config(
        client: reqwest::Client,
        base_url: impl Into<String>,
        auth_header: impl Into<String>,
    ) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            auth_header: auth_header.into(),
        }
    }
}

#[derive(Deserialize)]
struct JiraSearch {
    issues: Vec<JiraIssue>,
}
#[derive(Deserialize)]
struct JiraIssue {
    id: String,
    fields: JiraFields,
}
#[derive(Deserialize)]
struct JiraFields {
    summary: String,
    #[serde(default)]
    description: Option<String>,
    status: JiraStatus,
    #[serde(default)]
    assignee: Option<JiraUser>,
    #[serde(default)]
    labels: Vec<String>,
}
#[derive(Deserialize)]
struct JiraStatus {
    name: String,
}
#[derive(Deserialize)]
struct JiraUser {
    #[serde(rename = "displayName")]
    display_name: String,
}
#[derive(Deserialize)]
struct JiraCreated {
    id: String,
}

#[async_trait]
impl ProjectTracker for JiraTracker {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn list_issues(&self, query: Value) -> Result<Vec<Issue>, ProductivityError> {
        let jql = query
            .get("jql")
            .and_then(|v| v.as_str())
            .unwrap_or("ORDER BY created DESC");
        let url = format!("{}/rest/api/3/search", self.base_url);
        let body = json!({ "jql": jql, "maxResults": 50 });
        let resp = self
            .client
            .post(&url)
            .header("Authorization", &self.auth_header)
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
        let search: JiraSearch = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(search
            .issues
            .into_iter()
            .map(|i| Issue {
                id: i.id,
                title: i.fields.summary,
                description: i.fields.description,
                status: i.fields.status.name,
                assignee: i.fields.assignee.map(|a| a.display_name),
                labels: i.fields.labels,
            })
            .collect())
    }

    async fn create_issue(&self, issue: Issue) -> Result<String, ProductivityError> {
        let url = format!("{}/rest/api/3/issue", self.base_url);
        let body = json!({
            "fields": {
                "summary": issue.title,
                "description": issue.description.unwrap_or_default(),
                "issuetype": { "name": "Task" },
                "labels": issue.labels
            }
        });
        let resp = self
            .client
            .post(&url)
            .header("Authorization", &self.auth_header)
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
        let created: JiraCreated = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(created.id)
    }
}
