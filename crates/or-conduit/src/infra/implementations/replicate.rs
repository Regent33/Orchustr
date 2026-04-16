use crate::domain::contracts::ConduitProvider;
use crate::domain::entities::{CompletionMessage, CompletionResponse};
use crate::domain::errors::ConduitError;
use crate::infra::adapters::replicate::{
    parse_replicate_response, replicate_is_pending, replicate_payload, replicate_poll_url,
};
use crate::infra::http::{bearer_headers, required_env};
use or_core::{RetryPolicy, TokenBudget};
use reqwest::Client;
use serde_json::Value;
use std::fmt;
use std::time::Duration;

#[derive(Clone)]
pub struct ReplicateConduit {
    api_key: String,
    base_url: String,
    http_client: Client,
    model: String,
    retry_policy: RetryPolicy,
    token_budget: TokenBudget,
    timeout: Duration,
    poll_interval: Duration,
}

impl fmt::Debug for ReplicateConduit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReplicateConduit")
            .field("model", &self.model)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}

impl ReplicateConduit {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        Ok(Self {
            api_key: api_key.into(),
            base_url: "https://api.replicate.com".to_owned(),
            http_client: Client::new(),
            model: model.into(),
            retry_policy: RetryPolicy::default_llm(),
            token_budget: TokenBudget { max_context_tokens: 128_000, max_completion_tokens: 4_096 },
            timeout: Duration::from_secs(300),
            poll_interval: Duration::from_secs(2),
        })
    }

    pub fn from_env() -> Result<Self, ConduitError> {
        Self::new(required_env("REPLICATE_API_KEY")?, required_env("REPLICATE_MODEL")?)
    }

    #[must_use] pub fn with_retry(mut self, p: RetryPolicy) -> Self { self.retry_policy = p; self }
    #[must_use] pub fn with_budget(mut self, b: TokenBudget) -> Self { self.token_budget = b; self }
    #[must_use] pub fn with_timeout(mut self, t: Duration) -> Self { self.timeout = t; self }
}

impl ConduitProvider for ReplicateConduit {
    async fn complete_messages(&self, messages: Vec<CompletionMessage>) -> Result<CompletionResponse, ConduitError> {
        let payload = replicate_payload(&messages)?;
        let headers = bearer_headers(&self.api_key)?;
        let url = format!(
            "{}/v1/models/{}/predictions",
            self.base_url, self.model
        );
        let response = self.http_client
            .post(&url)
            .headers(headers.clone())
            .timeout(self.timeout)
            .json(&payload)
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(ConduitError::Api { status, body });
        }
        let mut body: Value = response
            .json()
            .await
            .map_err(|e| ConduitError::Serialization(e.to_string()))?;

        // Poll until completion
        let deadline = tokio::time::Instant::now() + self.timeout;
        while replicate_is_pending(&body) {
            if tokio::time::Instant::now() > deadline {
                return Err(ConduitError::Timeout);
            }
            let poll_url = replicate_poll_url(&body)
                .ok_or_else(|| ConduitError::Serialization("missing poll URL".to_owned()))?;
            tokio::time::sleep(self.poll_interval).await;
            let poll_resp = self.http_client
                .get(&poll_url)
                .headers(headers.clone())
                .timeout(Duration::from_secs(30))
                .send()
                .await?;
            body = poll_resp
                .json()
                .await
                .map_err(|e| ConduitError::Serialization(e.to_string()))?;
        }
        parse_replicate_response(&body)
    }
}
