use super::shared::{decode, load_credential, transport};
use crate::domain::contracts::CodeExecutor;
use crate::domain::entities::{ExecRequest, ExecResult, Language};
use crate::domain::errors::ExecError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

const PROVIDER: &str = "bearly";
const API_KEY_ENV: &str = "BEARLY_API_KEY";
const DEFAULT_URL: &str = "https://production.bearly.ai/execute";

#[derive(Clone)]
pub struct BearlyExecutor {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
}

impl BearlyExecutor {
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
struct BearlyResponse {
    stdout: String,
    #[serde(default)]
    stderr: String,
    #[serde(default)]
    exit_code: i32,
}

#[async_trait]
impl CodeExecutor for BearlyExecutor {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    fn supports(&self, lang: Language) -> bool {
        matches!(lang, Language::Python)
    }

    async fn execute(&self, req: ExecRequest) -> Result<ExecResult, ExecError> {
        let body = json!({ "fileContents": req.code });
        let resp = self
            .client
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(transport)?;
        let parsed: BearlyResponse = decode(PROVIDER, resp).await?;
        Ok(ExecResult {
            stdout: parsed.stdout,
            stderr: parsed.stderr,
            exit_code: parsed.exit_code,
            duration_ms: 0,
        })
    }
}
