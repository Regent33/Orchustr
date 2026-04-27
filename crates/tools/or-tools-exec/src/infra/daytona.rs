use super::shared::{decode, load_credential, transport};
use crate::domain::contracts::CodeExecutor;
use crate::domain::entities::{ExecRequest, ExecResult, Language};
use crate::domain::errors::ExecError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

const PROVIDER: &str = "daytona";
const API_KEY_ENV: &str = "DAYTONA_API_KEY";
const SERVER_URL_ENV: &str = "DAYTONA_SERVER_URL";

#[derive(Clone)]
pub struct DaytonaExecutor {
    client: reqwest::Client,
    server_url: String,
    api_key: String,
}

impl DaytonaExecutor {
    pub fn from_env() -> Result<Self, ExecError> {
        Ok(Self {
            client: reqwest::Client::new(),
            server_url: load_credential(SERVER_URL_ENV)?,
            api_key: load_credential(API_KEY_ENV)?,
        })
    }
}

#[derive(Deserialize)]
struct DaytonaRunResult {
    output: String,
    #[serde(default)]
    exit_code: i32,
}

#[async_trait]
impl CodeExecutor for DaytonaExecutor {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    fn supports(&self, _lang: Language) -> bool {
        true
    }

    async fn execute(&self, req: ExecRequest) -> Result<ExecResult, ExecError> {
        let body = json!({
            "command": req.code,
            "language": req.language.as_str(),
            "timeout": req.timeout_ms / 1000,
        });
        let url = format!("{}/workspace/exec", self.server_url.trim_end_matches('/'));
        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(transport)?;
        let parsed: DaytonaRunResult = decode(PROVIDER, resp).await?;
        Ok(ExecResult {
            stdout: parsed.output,
            stderr: String::new(),
            exit_code: parsed.exit_code,
            duration_ms: 0,
        })
    }
}
