use super::shared::{decode, load_credential, transport};
use crate::domain::contracts::CodeExecutor;
use crate::domain::entities::{ExecRequest, ExecResult, Language};
use crate::domain::errors::ExecError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

const PROVIDER: &str = "e2b";
const API_KEY_ENV: &str = "E2B_API_KEY";
const DEFAULT_URL: &str = "https://api.e2b.dev/sandboxes/run";

#[derive(Clone)]
pub struct E2BExecutor {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
}

impl E2BExecutor {
    pub fn from_env() -> Result<Self, ExecError> {
        Ok(Self {
            client: reqwest::Client::new(),
            endpoint: DEFAULT_URL.to_string(),
            api_key: load_credential(API_KEY_ENV)?,
        })
    }

    #[must_use]
    pub fn with_config(
        client: reqwest::Client,
        endpoint: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        Self {
            client,
            endpoint: endpoint.into(),
            api_key: api_key.into(),
        }
    }
}

#[derive(Deserialize)]
struct E2BResponse {
    stdout: String,
    stderr: String,
    exit_code: i32,
    #[serde(default)]
    duration_ms: u64,
}

#[async_trait]
impl CodeExecutor for E2BExecutor {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    fn supports(&self, _lang: Language) -> bool {
        true
    }

    async fn execute(&self, req: ExecRequest) -> Result<ExecResult, ExecError> {
        let body = json!({
            "code": req.code,
            "language": req.language.as_str(),
            "timeout_ms": req.timeout_ms,
        });
        let resp = self
            .client
            .post(&self.endpoint)
            .header("X-API-Key", &self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(transport)?;
        let parsed: E2BResponse = decode(PROVIDER, resp).await?;
        Ok(ExecResult {
            stdout: parsed.stdout,
            stderr: parsed.stderr,
            exit_code: parsed.exit_code,
            duration_ms: parsed.duration_ms,
        })
    }
}
