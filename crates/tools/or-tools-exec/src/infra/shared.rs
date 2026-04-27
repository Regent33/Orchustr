use crate::domain::errors::ExecError;
use reqwest::Response;

pub(crate) async fn decode<T: serde::de::DeserializeOwned>(
    provider: &'static str,
    resp: Response,
) -> Result<T, ExecError> {
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(ExecError::Upstream {
            provider: provider.into(),
            status: status.as_u16(),
            body,
        });
    }
    resp.json::<T>()
        .await
        .map_err(|e| ExecError::Transport(e.to_string()))
}

pub(crate) fn transport(err: reqwest::Error) -> ExecError {
    ExecError::Transport(err.to_string())
}

pub(crate) fn load_credential(env_var: &'static str) -> Result<String, ExecError> {
    std::env::var(env_var).map_err(|_| ExecError::MissingCredential(env_var.into()))
}
