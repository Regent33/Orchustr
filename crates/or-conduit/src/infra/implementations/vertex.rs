use crate::domain::contracts::ConduitProvider;
use crate::domain::entities::{CompletionMessage, CompletionResponse};
use crate::domain::errors::ConduitError;
use crate::infra::adapters::vertex::parse_vertex_response;
use crate::infra::adapters::gemini::gemini_payload;
use crate::infra::http::{bearer_headers, required_env};
use or_core::{RetryPolicy, TokenBudget};
use reqwest::Client;
use serde_json::Value;
use std::fmt;
use std::time::Duration;

/// Google Vertex AI conduit using bring-your-own-token auth.
/// The caller must supply an OAuth2 access token.
#[derive(Clone)]
pub struct VertexConduit {
    access_token: String,
    project_id: String,
    location: String,
    http_client: Client,
    model: String,
    retry_policy: RetryPolicy,
    token_budget: TokenBudget,
    timeout: Duration,
}

impl fmt::Debug for VertexConduit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VertexConduit")
            .field("project_id", &self.project_id)
            .field("model", &self.model)
            .field("access_token", &"[REDACTED]")
            .finish()
    }
}

impl VertexConduit {
    pub fn new(
        access_token: impl Into<String>,
        project_id: impl Into<String>,
        location: impl Into<String>,
        model: impl Into<String>,
    ) -> Result<Self, ConduitError> {
        Ok(Self {
            access_token: access_token.into(),
            project_id: project_id.into(),
            location: location.into(),
            http_client: Client::new(),
            model: model.into(),
            retry_policy: RetryPolicy::default_llm(),
            token_budget: TokenBudget { max_context_tokens: 1_000_000, max_completion_tokens: 8_192 },
            timeout: Duration::from_secs(60),
        })
    }

    pub fn from_env() -> Result<Self, ConduitError> {
        Self::new(
            required_env("VERTEX_ACCESS_TOKEN")?,
            required_env("VERTEX_PROJECT_ID")?,
            required_env("VERTEX_LOCATION").unwrap_or_else(|_| "us-central1".to_owned()),
            required_env("VERTEX_MODEL")?,
        )
    }

    #[must_use] pub fn with_retry(mut self, p: RetryPolicy) -> Self { self.retry_policy = p; self }
    #[must_use] pub fn with_budget(mut self, b: TokenBudget) -> Self { self.token_budget = b; self }
    #[must_use] pub fn with_timeout(mut self, t: Duration) -> Self { self.timeout = t; self }
}

impl ConduitProvider for VertexConduit {
    async fn complete_messages(&self, messages: Vec<CompletionMessage>) -> Result<CompletionResponse, ConduitError> {
        let payload = gemini_payload(&messages)?;
        let headers = bearer_headers(&self.access_token)?;
        let url = format!(
            "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:generateContent",
            self.location, self.project_id, self.location, self.model
        );
        let response = self.http_client
            .post(&url)
            .headers(headers)
            .timeout(self.timeout)
            .json(&payload)
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(ConduitError::Api { status, body });
        }
        let body: Value = response.json().await
            .map_err(|e| ConduitError::Serialization(e.to_string()))?;
        parse_vertex_response(&body)
    }
}
