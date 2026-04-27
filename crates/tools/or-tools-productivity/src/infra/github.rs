use super::shared::{build_url, load_credential, transport};
use crate::domain::contracts::ProjectTracker;
use crate::domain::entities::Issue;
use crate::domain::errors::ProductivityError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

const PROVIDER: &str = "github";
const TOKEN_ENV: &str = "GITHUB_TOKEN";
const OWNER_ENV: &str = "GITHUB_OWNER";
const REPO_ENV: &str = "GITHUB_REPO";
const BASE_URL: &str = "https://api.github.com/repos";

pub struct GitHubTracker {
    client: reqwest::Client,
    token: String,
    owner: String,
    repo: String,
    base_url: String,
}

impl GitHubTracker {
    pub fn from_env() -> Result<Self, ProductivityError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: BASE_URL.into(),
            token: load_credential(TOKEN_ENV)?,
            owner: load_credential(OWNER_ENV)?,
            repo: load_credential(REPO_ENV)?,
        })
    }

    pub fn with_config(
        client: reqwest::Client,
        base_url: impl Into<String>,
        token: impl Into<String>,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            token: token.into(),
            owner: owner.into(),
            repo: repo.into(),
        }
    }
}

#[derive(Deserialize)]
struct GhIssue {
    #[serde(rename = "number")]
    id: u64,
    title: String,
    #[serde(default)]
    body: Option<String>,
    state: String,
    #[serde(default)]
    assignee: Option<GhUser>,
    #[serde(default)]
    labels: Vec<GhLabel>,
}
#[derive(Deserialize)]
struct GhUser {
    login: String,
}
#[derive(Deserialize)]
struct GhLabel {
    name: String,
}
#[derive(Deserialize)]
struct GhCreated {
    number: u64,
}

#[async_trait]
impl ProjectTracker for GitHubTracker {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn list_issues(&self, query: Value) -> Result<Vec<Issue>, ProductivityError> {
        let state = query
            .get("state")
            .and_then(|v| v.as_str())
            .unwrap_or("open");
        let url = build_url(
            &format!("{}/{}/{}/issues", self.base_url, self.owner, self.repo),
            &[("state", state), ("per_page", "50")],
        )?;
        let resp = self
            .client
            .get(url)
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", "orchustr")
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
        let issues: Vec<GhIssue> = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(issues
            .into_iter()
            .map(|i| Issue {
                id: i.id.to_string(),
                title: i.title,
                description: i.body,
                status: i.state,
                assignee: i.assignee.map(|a| a.login),
                labels: i.labels.into_iter().map(|l| l.name).collect(),
            })
            .collect())
    }

    async fn create_issue(&self, issue: Issue) -> Result<String, ProductivityError> {
        let url = format!("{}/{}/{}/issues", self.base_url, self.owner, self.repo);
        let body = json!({
            "title": issue.title,
            "body": issue.description.unwrap_or_default(),
            "labels": issue.labels
        });
        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", "orchustr")
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
        let created: GhCreated = resp
            .json()
            .await
            .map_err(|e| ProductivityError::Transport(e.to_string()))?;
        Ok(created.number.to_string())
    }
}
