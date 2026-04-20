use crate::domain::contracts::ConduitProvider;
use crate::domain::entities::{CompletionMessage, CompletionResponse};
use crate::domain::errors::ConduitError;
use crate::infra::adapters::bedrock::{bedrock_claude_payload, parse_bedrock_claude_response};
use crate::infra::http::{bearer_headers, required_env};
use or_core::{RetryPolicy, TokenBudget};
use reqwest::Client;
use serde_json::Value;
use std::fmt;
use std::time::Duration;

/// AWS Bedrock conduit using bring-your-own-token auth.
/// The caller must supply a pre-signed session token or temporary credentials.
#[derive(Clone)]
pub struct BedrockConduit {
    /// Session token or temporary bearer token
    auth_token: String,
    base_url: String,
    http_client: Client,
    model_id: String,
    retry_policy: RetryPolicy,
    token_budget: TokenBudget,
    timeout: Duration,
}

impl fmt::Debug for BedrockConduit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BedrockConduit")
            .field("model_id", &self.model_id)
            .field("auth_token", &"[REDACTED]")
            .finish()
    }
}

impl BedrockConduit {
    pub fn new(
        auth_token: impl Into<String>,
        region: impl Into<String>,
        model_id: impl Into<String>,
    ) -> Result<Self, ConduitError> {
        let region_str: String = region.into();
        Ok(Self {
            auth_token: auth_token.into(),
            base_url: format!("https://bedrock-runtime.{region_str}.amazonaws.com"),
            http_client: Client::new(),
            model_id: model_id.into(),
            retry_policy: RetryPolicy::default_llm(),
            token_budget: TokenBudget {
                max_context_tokens: 200_000,
                max_completion_tokens: 4_096,
            },
            timeout: Duration::from_secs(120),
        })
    }

    pub fn from_env() -> Result<Self, ConduitError> {
        Self::new(
            required_env("AWS_SESSION_TOKEN")?,
            required_env("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_owned()),
            required_env("BEDROCK_MODEL_ID")?,
        )
    }

    #[must_use]
    pub fn with_retry(mut self, p: RetryPolicy) -> Self {
        self.retry_policy = p;
        self
    }
    #[must_use]
    pub fn with_budget(mut self, b: TokenBudget) -> Self {
        self.token_budget = b;
        self
    }
    #[must_use]
    pub fn with_timeout(mut self, t: Duration) -> Self {
        self.timeout = t;
        self
    }
}

impl ConduitProvider for BedrockConduit {
    async fn complete_messages(
        &self,
        messages: Vec<CompletionMessage>,
    ) -> Result<CompletionResponse, ConduitError> {
        let payload = bedrock_claude_payload(&messages, self.token_budget.max_completion_tokens)?;
        let headers = bearer_headers(&self.auth_token)?;
        let url = format!("{}/model/{}/invoke", self.base_url, self.model_id);
        let response = self
            .http_client
            .post(url)
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
        let body: Value = response
            .json()
            .await
            .map_err(|e| ConduitError::Serialization(e.to_string()))?;
        parse_bedrock_claude_response(&body)
    }
}
