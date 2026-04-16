use crate::domain::contracts::ConduitProvider;
use crate::domain::entities::{CompletionMessage, CompletionResponse};
use crate::domain::errors::ConduitError;
use crate::infra::adapters::huggingface::{huggingface_payload, parse_huggingface_response};
use crate::infra::http::{HttpConduit, bearer_headers, required_env};
use or_core::{RetryPolicy, TokenBudget};
use reqwest::Client;
use std::fmt;
use std::time::Duration;

#[derive(Clone)]
pub struct HuggingFaceConduit {
    api_key: String,
    base_url: String,
    http_client: Client,
    model: String,
    retry_policy: RetryPolicy,
    token_budget: TokenBudget,
    timeout: Duration,
}

impl fmt::Debug for HuggingFaceConduit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HuggingFaceConduit")
            .field("model", &self.model)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}

impl HuggingFaceConduit {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        let model_str: String = model.into();
        Ok(Self {
            api_key: api_key.into(),
            base_url: format!("https://api-inference.huggingface.co/models/{model_str}"),
            http_client: Client::new(),
            model: model_str,
            retry_policy: RetryPolicy::default_llm(),
            token_budget: TokenBudget { max_context_tokens: 32_000, max_completion_tokens: 2_048 },
            timeout: Duration::from_secs(120),
        })
    }

    pub fn from_env() -> Result<Self, ConduitError> {
        Self::new(required_env("HF_API_KEY")?, required_env("HF_MODEL")?)
    }

    #[must_use] pub fn with_retry(mut self, p: RetryPolicy) -> Self { self.retry_policy = p; self }
    #[must_use] pub fn with_budget(mut self, b: TokenBudget) -> Self { self.token_budget = b; self }
    #[must_use] pub fn with_timeout(mut self, t: Duration) -> Self { self.timeout = t; self }
}

impl ConduitProvider for HuggingFaceConduit {
    async fn complete_messages(&self, messages: Vec<CompletionMessage>) -> Result<CompletionResponse, ConduitError> {
        // HF Inference: POST directly to base_url (model is in the URL)
        self.complete("", huggingface_payload(&messages, self.token_budget.max_completion_tokens)?, &messages, bearer_headers(&self.api_key)?, parse_huggingface_response).await
    }
}

impl HttpConduit for HuggingFaceConduit {
    fn base_url(&self) -> &str { &self.base_url }
    fn client(&self) -> &Client { &self.http_client }
    fn retry_policy(&self) -> &RetryPolicy { &self.retry_policy }
    fn token_budget(&self) -> &TokenBudget { &self.token_budget }
    fn timeout(&self) -> Duration { self.timeout }
}
