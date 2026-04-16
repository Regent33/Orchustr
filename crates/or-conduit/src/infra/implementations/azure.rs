use crate::domain::contracts::ConduitProvider;
use crate::domain::entities::{CompletionMessage, CompletionResponse};
use crate::domain::errors::ConduitError;
use crate::infra::adapters::openai_compat::{openai_chat_payload, parse_openai_chat_response};
use crate::infra::http::{HttpConduit, required_env};
use or_core::{RetryPolicy, TokenBudget};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::fmt;
use std::time::Duration;

/// Azure OpenAI conduit.
/// Uses Azure-specific URL patterns and `api-key` header auth.
#[derive(Clone)]
pub struct AzureConduit {
    api_key: String,
    /// Full resource URL: `https://{resource}.openai.azure.com`
    base_url: String,
    http_client: Client,
    deployment: String,
    api_version: String,
    retry_policy: RetryPolicy,
    token_budget: TokenBudget,
    timeout: Duration,
}

impl fmt::Debug for AzureConduit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AzureConduit")
            .field("base_url", &self.base_url)
            .field("deployment", &self.deployment)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}

impl AzureConduit {
    pub fn new(
        api_key: impl Into<String>,
        base_url: impl Into<String>,
        deployment: impl Into<String>,
        api_version: impl Into<String>,
    ) -> Result<Self, ConduitError> {
        Ok(Self {
            api_key: api_key.into(),
            base_url: base_url.into(),
            http_client: Client::new(),
            deployment: deployment.into(),
            api_version: api_version.into(),
            retry_policy: RetryPolicy::default_llm(),
            token_budget: TokenBudget { max_context_tokens: 128_000, max_completion_tokens: 4_096 },
            timeout: Duration::from_secs(60),
        })
    }

    pub fn from_env() -> Result<Self, ConduitError> {
        Self::new(
            required_env("AZURE_OPENAI_API_KEY")?,
            required_env("AZURE_OPENAI_ENDPOINT")?,
            required_env("AZURE_OPENAI_DEPLOYMENT")?,
            required_env("AZURE_OPENAI_API_VERSION")
                .unwrap_or_else(|_| "2024-06-01".to_owned()),
        )
    }

    #[must_use] pub fn with_retry(mut self, p: RetryPolicy) -> Self { self.retry_policy = p; self }
    #[must_use] pub fn with_budget(mut self, b: TokenBudget) -> Self { self.token_budget = b; self }
    #[must_use] pub fn with_timeout(mut self, t: Duration) -> Self { self.timeout = t; self }

    fn headers(&self) -> Result<HeaderMap, ConduitError> {
        let mut h = HeaderMap::new();
        h.insert(
            "api-key",
            HeaderValue::from_str(&self.api_key)
                .map_err(|e| ConduitError::AuthenticationFailed(e.to_string()))?,
        );
        h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        Ok(h)
    }

    fn api_path(&self) -> String {
        format!(
            "/openai/deployments/{}/chat/completions?api-version={}",
            self.deployment, self.api_version
        )
    }
}

impl ConduitProvider for AzureConduit {
    async fn complete_messages(&self, messages: Vec<CompletionMessage>) -> Result<CompletionResponse, ConduitError> {
        // Azure uses the deployment name, not model field in payload
        let payload = openai_chat_payload(&self.deployment, &messages, self.token_budget.max_completion_tokens)?;
        self.complete(&self.api_path(), payload, &messages, self.headers()?, parse_openai_chat_response).await
    }
}

impl HttpConduit for AzureConduit {
    fn base_url(&self) -> &str { &self.base_url }
    fn client(&self) -> &Client { &self.http_client }
    fn retry_policy(&self) -> &RetryPolicy { &self.retry_policy }
    fn token_budget(&self) -> &TokenBudget { &self.token_budget }
    fn timeout(&self) -> Duration { self.timeout }
}
