use crate::domain::contracts::ConduitProvider;
use crate::domain::entities::{CompletionMessage, CompletionResponse};
use crate::domain::errors::ConduitError;
use crate::infra::adapters::openai_compat::{
    openai_chat_payload, openai_payload, parse_openai_chat_response, parse_openai_response,
};
use crate::infra::http::{HttpConduit, bearer_headers, required_env};
use or_core::{RetryPolicy, TokenBudget};
use reqwest::Client;
use std::fmt;
use std::time::Duration;

/// Generic conduit for any OpenAI-compatible API.
/// Covers: OpenAI, OpenRouter, Together AI, Groq, Fireworks AI,
/// DeepSeek, Mistral, xAI/Grok, Nvidia, Ollama.
#[derive(Clone)]
pub struct OpenAiCompatConduit {
    api_key: String,
    base_url: String,
    http_client: Client,
    model: String,
    retry_policy: RetryPolicy,
    token_budget: TokenBudget,
    api_path: String,
    use_chat_format: bool,
    timeout: Duration,
}

impl fmt::Debug for OpenAiCompatConduit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenAiCompatConduit")
            .field("base_url", &self.base_url)
            .field("model", &self.model)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}

/// Builder configuration for an OpenAI-compatible provider.
pub struct OpenAiCompatConfig {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
    pub api_path: String,
    pub use_chat_format: bool,
    pub max_context_tokens: u32,
    pub max_completion_tokens: u32,
}

impl OpenAiCompatConduit {
    pub fn from_config(config: OpenAiCompatConfig) -> Result<Self, ConduitError> {
        Ok(Self {
            api_key: config.api_key,
            base_url: config.base_url,
            http_client: Client::new(),
            model: config.model,
            retry_policy: RetryPolicy::default_llm(),
            token_budget: TokenBudget {
                max_context_tokens: config.max_context_tokens,
                max_completion_tokens: config.max_completion_tokens,
            },
            api_path: config.api_path,
            use_chat_format: config.use_chat_format,
            timeout: Duration::from_secs(60),
        })
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

impl ConduitProvider for OpenAiCompatConduit {
    async fn complete_messages(
        &self,
        messages: Vec<CompletionMessage>,
    ) -> Result<CompletionResponse, ConduitError> {
        let (payload, parser) = if self.use_chat_format {
            (
                openai_chat_payload(
                    &self.model,
                    &messages,
                    self.token_budget.max_completion_tokens,
                )?,
                parse_openai_chat_response as fn(&_) -> _,
            )
        } else {
            (
                openai_payload(
                    &self.model,
                    &messages,
                    self.token_budget.max_completion_tokens,
                )?,
                parse_openai_response as fn(&_) -> _,
            )
        };
        self.complete(
            &self.api_path,
            payload,
            &messages,
            bearer_headers(&self.api_key)?,
            parser,
        )
        .await
    }
}

impl HttpConduit for OpenAiCompatConduit {
    fn base_url(&self) -> &str { &self.base_url }
    fn client(&self) -> &Client { &self.http_client }
    fn retry_policy(&self) -> &RetryPolicy { &self.retry_policy }
    fn token_budget(&self) -> &TokenBudget { &self.token_budget }
    fn timeout(&self) -> Duration { self.timeout }
}

// ── Factory constructors for specific providers ─────────────────────

fn chat_config(
    api_key: String,
    model: String,
    base_url: &str,
    ctx: u32,
    comp: u32,
) -> OpenAiCompatConfig {
    OpenAiCompatConfig {
        api_key,
        model,
        base_url: base_url.to_owned(),
        api_path: "/v1/chat/completions".to_owned(),
        use_chat_format: true,
        max_context_tokens: ctx,
        max_completion_tokens: comp,
    }
}

impl OpenAiCompatConduit {
    /// OpenAI Responses API (native format).
    pub fn openai(api_key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        Self::from_config(OpenAiCompatConfig {
            api_key: api_key.into(),
            model: model.into(),
            base_url: "https://api.openai.com".to_owned(),
            api_path: "/v1/responses".to_owned(),
            use_chat_format: false,
            max_context_tokens: 128_000,
            max_completion_tokens: 4_096,
        })
    }

    pub fn openai_from_env() -> Result<Self, ConduitError> {
        Self::openai(required_env("OPENAI_API_KEY")?, required_env("OPENAI_MODEL")?)
    }

    pub fn openrouter(key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        Self::from_config(chat_config(key.into(), model.into(), "https://openrouter.ai/api", 128_000, 4_096))
    }

    pub fn together(key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        Self::from_config(chat_config(key.into(), model.into(), "https://api.together.xyz", 128_000, 4_096))
    }

    pub fn groq(key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        Self::from_config(chat_config(key.into(), model.into(), "https://api.groq.com/openai", 128_000, 4_096))
    }

    pub fn fireworks(key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        Self::from_config(chat_config(key.into(), model.into(), "https://api.fireworks.ai/inference", 128_000, 4_096))
    }

    pub fn deepseek(key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        Self::from_config(chat_config(key.into(), model.into(), "https://api.deepseek.com", 128_000, 4_096))
    }

    pub fn mistral(key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        Self::from_config(chat_config(key.into(), model.into(), "https://api.mistral.ai", 128_000, 4_096))
    }

    pub fn xai(key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        Self::from_config(chat_config(key.into(), model.into(), "https://api.x.ai", 128_000, 4_096))
    }

    pub fn nvidia(key: impl Into<String>, model: impl Into<String>) -> Result<Self, ConduitError> {
        Self::from_config(chat_config(key.into(), model.into(), "https://integrate.api.nvidia.com", 128_000, 4_096))
    }

    pub fn ollama(model: impl Into<String>) -> Result<Self, ConduitError> {
        Self::from_config(OpenAiCompatConfig {
            api_key: "ollama".to_owned(),
            model: model.into(),
            base_url: "http://localhost:11434".to_owned(),
            api_path: "/v1/chat/completions".to_owned(),
            use_chat_format: true,
            max_context_tokens: 128_000,
            max_completion_tokens: 4_096,
        })
    }
}
