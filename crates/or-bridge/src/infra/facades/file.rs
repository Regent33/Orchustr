//! Bridge entry point for `or-tools-file` (filesystem + external data
//! sources).

use super::helpers::{
    block_on, get_str, invocation, json_value, required_str, unsupported, unsupported_provider,
};
use crate::domain::errors::BridgeError;
use or_tools_file::infra::{
    arxiv::ArxivSource, financial::FinancialDatasetsSource, gdrive::GoogleDriveStore,
    json_toolkit::JsonToolkit, local_fs::LocalFileSystem,
};
use or_tools_file::{DataSource, FileOrchestrator, FileStore};
use serde_json::{Value, json};
use std::sync::Arc;

pub(crate) fn invoke(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    match operation {
        "read" | "write" | "list" | "delete" => {
            let provider_name = payload
                .get("provider")
                .and_then(Value::as_str)
                .unwrap_or("local");
            let store = build_file_store(provider_name, payload.get("config"))?;
            let orchestrator = FileOrchestrator::new(store.clone());
            match operation {
                "read" => {
                    let path = required_str(&payload, "path", "or-tools-file", operation)?;
                    block_on("or-tools-file", operation, orchestrator.read(path))
                        .and_then(json_value)
                }
                "write" => {
                    let path = required_str(&payload, "path", "or-tools-file", operation)?;
                    let content = required_str(&payload, "content", "or-tools-file", operation)?;
                    block_on("or-tools-file", operation, store.write(path, content))?;
                    Ok(json!({ "status": "ok" }))
                }
                "list" => {
                    let path = required_str(&payload, "path", "or-tools-file", operation)?;
                    block_on("or-tools-file", operation, store.list(path)).and_then(json_value)
                }
                "delete" => {
                    let path = required_str(&payload, "path", "or-tools-file", operation)?;
                    block_on("or-tools-file", operation, store.delete(path))?;
                    Ok(json!({ "status": "ok" }))
                }
                _ => unreachable!(),
            }
        }
        "fetch" => {
            let provider_name = required_str(&payload, "provider", "or-tools-file", operation)?;
            let query = payload.get("query").cloned().unwrap_or(Value::Null);
            let source = build_data_source(provider_name, payload.get("config"))?;
            block_on("or-tools-file", operation, source.fetch(query))
        }
        _ => Err(unsupported("or-tools-file", operation)),
    }
}

fn build_file_store(
    provider: &str,
    config: Option<&Value>,
) -> Result<Arc<dyn FileStore>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let store: Arc<dyn FileStore> = match provider {
        "local" => Arc::new(LocalFileSystem),
        "gdrive" => Arc::new(
            if let Some(access_token) = cfg.and_then(|v| get_str(v, "access_token")) {
                GoogleDriveStore::with_token(client, access_token)
            } else {
                GoogleDriveStore::from_env()
                    .map_err(|error| invocation("or-tools-file", "store", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-file", other)),
    };
    Ok(store)
}

fn build_data_source(
    provider: &str,
    config: Option<&Value>,
) -> Result<Arc<dyn DataSource>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let source: Arc<dyn DataSource> = match provider {
        "json" => Arc::new(JsonToolkit),
        "arxiv" => Arc::new(
            if let Some(endpoint) = cfg.and_then(|v| get_str(v, "endpoint")) {
                ArxivSource::with_endpoint(client, endpoint)
            } else {
                ArxivSource::new()
            },
        ),
        "financial" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (
                cfg.and_then(|v| get_str(v, "endpoint")),
                cfg.and_then(|v| get_str(v, "api_key")),
            ) {
                FinancialDatasetsSource::with_config(client, endpoint, api_key)
            } else {
                FinancialDatasetsSource::from_env()
                    .map_err(|error| invocation("or-tools-file", "fetch", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-file", other)),
    };
    Ok(source)
}
