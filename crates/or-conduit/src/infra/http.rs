use crate::domain::entities::{CompletionMessage, CompletionResponse};
use crate::domain::errors::ConduitError;
use crate::infra::adapters::openai_compat::estimate_prompt_tokens;
use or_core::{CoreOrchestrator, RetryPolicy, TokenBudget};
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde_json::Value;
use std::time::Duration;

pub(crate) trait HttpConduit {
    fn base_url(&self) -> &str;
    fn client(&self) -> &Client;
    fn retry_policy(&self) -> &RetryPolicy;
    fn token_budget(&self) -> &TokenBudget;
    fn timeout(&self) -> Duration {
        Duration::from_secs(60)
    }

    async fn complete(
        &self,
        path: &str,
        payload: Value,
        messages: &[CompletionMessage],
        headers: HeaderMap,
        parser: fn(&Value) -> Result<CompletionResponse, ConduitError>,
    ) -> Result<CompletionResponse, ConduitError> {
        let prompt_tokens = estimate_prompt_tokens(messages);
        CoreOrchestrator::new()
            .enforce_completion_budget(self.token_budget(), prompt_tokens)
            .map_err(|_| ConduitError::BudgetExceeded {
                requested: prompt_tokens.saturating_add(self.token_budget().max_completion_tokens),
                budget: self.token_budget().max_context_tokens,
            })?;
        let url = format!("{}{}", self.base_url(), path);
        let body = send_json(
            self.client(),
            &url,
            payload,
            headers,
            self.retry_policy(),
            self.timeout(),
        )
        .await?;
        parser(&body)
    }
}

async fn send_json(
    client: &Client,
    url: &str,
    payload: Value,
    headers: HeaderMap,
    retry_policy: &RetryPolicy,
    timeout: Duration,
) -> Result<Value, ConduitError> {
    let orchestrator = CoreOrchestrator::new();
    for attempt in 1..=retry_policy.max_attempts.max(1) {
        let response = client
            .post(url)
            .headers(headers.clone())
            .timeout(timeout)
            .json(&payload)
            .send()
            .await?;
        if response.status().is_success() {
            return response
                .json::<Value>()
                .await
                .map_err(|error| ConduitError::Serialization(error.to_string()));
        }
        let status = response.status().as_u16();
        let retry_after_ms = response
            .headers()
            .get("retry-after")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
            .map(|seconds| seconds.saturating_mul(1000))
            .unwrap_or_default();
        let body: String = response.text().await.unwrap_or_default();
        if (status == 429 || (500..=599).contains(&status)) && attempt < retry_policy.max_attempts {
            let delay = if retry_after_ms > 0 {
                retry_after_ms
            } else {
                orchestrator
                    .next_retry_delay(retry_policy, attempt)
                    .map_or(0, |value| value.as_millis() as u64)
            };
            tokio::time::sleep(Duration::from_millis(delay)).await;
            continue;
        }
        return if status == 429 {
            Err(ConduitError::RateLimited { retry_after_ms })
        } else {
            Err(ConduitError::Api { status, body })
        };
    }
    Err(ConduitError::Api {
        status: 500,
        body: "retry policy exhausted".to_owned(),
    })
}

pub(crate) fn required_env(name: &str) -> Result<String, ConduitError> {
    std::env::var(name).map_err(|_| ConduitError::MissingEnvironmentVariable(name.to_owned()))
}

/// Builds a Bearer-token `Authorization` header set.
/// Returns an error instead of silently dropping auth if the key is invalid.
pub(crate) fn bearer_headers(api_key: &str) -> Result<HeaderMap, ConduitError> {
    let mut headers = HeaderMap::new();
    let value = HeaderValue::from_str(&format!("Bearer {api_key}"))
        .map_err(|e| ConduitError::AuthenticationFailed(e.to_string()))?;
    headers.insert(AUTHORIZATION, value);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    Ok(headers)
}
