use crate::domain::contracts::ConduitProvider;
use crate::domain::entities::{CompletionMessage, CompletionResponse};
use crate::domain::errors::ConduitError;
use crate::infra::adapters::gemini::{gemini_payload, parse_gemini_response};
use crate::infra::http::required_env;
use or_core::{RetryPolicy, TokenBudget};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;
use std::fmt;
use std::time::Duration;

/// Google Gemini API conduit.
/// Uses the `generateContent` endpoint directly.
#[derive(Clone)]
pub struct GeminiConduit {
    api_key: String,
    base_url: String,
    http_client: Client,
    model: String,
    retry_policy: RetryPolicy,
    token_budget: TokenBudget,
    timeout: Duration,
}

impl fmt::Debug for GeminiConduit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GeminiConduit")
            .field("model", &self.model)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}

impl GeminiConduit {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        Ok(Self {
            api_key: api_key.into(),
            base_url: "https://generativelanguage.googleapis.com".to_owned(),
            http_client: Client::new(),
            model: model.into(),
            retry_policy: RetryPolicy::default_llm(),
            token_budget: TokenBudget {
                max_context_tokens: 1_000_000,
                max_completion_tokens: 8_192,
            },
            timeout: Duration::from_secs(60),
        })
    }

    pub fn from_env() -> Result<Self, ConduitError> {
        Self::new(required_env("GEMINI_API_KEY")?, required_env("GEMINI_MODEL")?)
    }

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
}

impl ConduitProvider for GeminiConduit {
    async fn complete_messages(
        &self,
        messages: Vec<CompletionMessage>,
    ) -> Result<CompletionResponse, ConduitError> {
        let payload = gemini_payload(&messages)?;
        let path = format!(
            "/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
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
        let body: Value = response
            .json()
            .await
            .map_err(|e| ConduitError::Serialization(e.to_string()))?;
        parse_gemini_response(&body)
    }
}
