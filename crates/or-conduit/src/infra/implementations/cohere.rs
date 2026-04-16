use crate::domain::contracts::ConduitProvider;
use crate::domain::entities::{CompletionMessage, CompletionResponse};
use crate::domain::errors::ConduitError;
use crate::infra::adapters::cohere::{cohere_payload, parse_cohere_response};
use crate::infra::http::{HttpConduit, bearer_headers, required_env};
use or_core::{RetryPolicy, TokenBudget};
use reqwest::Client;
use std::fmt;
use std::time::Duration;

#[derive(Clone)]
pub struct CohereConduit {
    api_key: String,
    base_url: String,
    http_client: Client,
    model: String,
    retry_policy: RetryPolicy,
    token_budget: TokenBudget,
    timeout: Duration,
}

impl fmt::Debug for CohereConduit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CohereConduit")
            .field("model", &self.model)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}

impl CohereConduit {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        Ok(Self {
            api_key: api_key.into(),
            base_url: "https://api.cohere.com".to_owned(),
            http_client: Client::new(),
            model: model.into(),
            retry_policy: RetryPolicy::default_llm(),
            token_budget: TokenBudget { max_context_tokens: 128_000, max_completion_tokens: 4_096 },
            timeout: Duration::from_secs(60),
        })
    }

    pub fn from_env() -> Result<Self, ConduitError> {
        Self::new(required_env("COHERE_API_KEY")?, required_env("COHERE_MODEL")?)
    }

    #[must_use] pub fn with_retry(mut self, p: RetryPolicy) -> Self { self.retry_policy = p; self }
    #[must_use] pub fn with_budget(mut self, b: TokenBudget) -> Self { self.token_budget = b; self }
    #[must_use] pub fn with_timeout(mut self, t: Duration) -> Self { self.timeout = t; self }
}

impl ConduitProvider for CohereConduit {
    async fn complete_messages(&self, messages: Vec<CompletionMessage>) -> Result<CompletionResponse, ConduitError> {
        self.complete("/v2/chat", cohere_payload(&self.model, &messages)?, &messages, bearer_headers(&self.api_key)?, parse_cohere_response).await
    }
}

impl HttpConduit for CohereConduit {
    fn base_url(&self) -> &str { &self.base_url }
    fn client(&self) -> &Client { &self.http_client }
    fn retry_policy(&self) -> &RetryPolicy { &self.retry_policy }
    fn token_budget(&self) -> &TokenBudget { &self.token_budget }
    fn timeout(&self) -> Duration { self.timeout }
}
