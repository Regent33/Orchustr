use crate::domain::contracts::ConduitProvider;
use crate::domain::entities::{CompletionMessage, CompletionResponse};
use crate::domain::errors::ConduitError;
use crate::infra::adapters::anthropic::{anthropic_payload, parse_anthropic_response};
use crate::infra::http::{HttpConduit, required_env};
use or_core::{RetryPolicy, TokenBudget};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::fmt;
use std::time::Duration;

#[derive(Clone)]
pub struct AnthropicConduit {
    api_key: String,
    base_url: String,
    http_client: Client,
    model: String,
    retry_policy: RetryPolicy,
    token_budget: TokenBudget,
    timeout: Duration,
}

impl fmt::Debug for AnthropicConduit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnthropicConduit")
            .field("base_url", &self.base_url)
            .field("model", &self.model)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}

impl AnthropicConduit {
    #[must_use]
    pub fn with_retry(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    #[must_use]
    pub fn with_budget(mut self, budget: TokenBudget) -> Self {
        self.token_budget = budget;
        self
    }

    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        Ok(Self {
            api_key: api_key.into(),
            base_url: "https://api.anthropic.com".to_owned(),
            http_client: Client::new(),
            model: model.into(),
            retry_policy: RetryPolicy::default_llm(),
            token_budget: TokenBudget {
                max_context_tokens: 200_000,
                max_completion_tokens: 4_096,
            },
            timeout: Duration::from_secs(60),
        })
    }

    pub fn from_env() -> Result<Self, ConduitError> {
        Self::new(
            required_env("ANTHROPIC_API_KEY")?,
            required_env("ANTHROPIC_MODEL")?,
        )
    }

    fn headers(&self) -> Result<HeaderMap, ConduitError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(&self.api_key)
                .map_err(|e| ConduitError::AuthenticationFailed(e.to_string()))?,
        );
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        Ok(headers)
    }
}

impl ConduitProvider for AnthropicConduit {
    async fn complete_messages(
        &self,
        messages: Vec<CompletionMessage>,
    ) -> Result<CompletionResponse, ConduitError> {
        self.complete(
            "/v1/messages",
            anthropic_payload(
                &self.model,
                &messages,
                self.token_budget.max_completion_tokens,
            )?,
            &messages,
            self.headers()?,
            parse_anthropic_response,
        )
        .await
    }
}

impl HttpConduit for AnthropicConduit {
    fn base_url(&self) -> &str { &self.base_url }
    fn client(&self) -> &Client { &self.http_client }
    fn retry_policy(&self) -> &RetryPolicy { &self.retry_policy }
    fn token_budget(&self) -> &TokenBudget { &self.token_budget }
    fn timeout(&self) -> Duration { self.timeout }
}
