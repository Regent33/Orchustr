//! Bridge entry point for `or-tools-exec` (local + sandboxed code
//! execution providers).

use super::helpers::{
    block_on, from_field, get_str, invocation, json_value, unsupported, unsupported_provider,
};
use crate::domain::errors::BridgeError;
use or_tools_exec::infra::{
    bearly::BearlyExecutor, daytona::DaytonaExecutor, e2b::E2BExecutor, python::PythonExecutor,
    shell::ShellExecutor,
};
use or_tools_exec::{CodeExecutor, ExecOrchestrator, ExecRequest};
use serde_json::Value;
use std::sync::Arc;

pub(crate) fn invoke(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    if operation != "execute" {
        return Err(unsupported("or-tools-exec", operation));
    }
    let request: ExecRequest = from_field(&payload, "request", "or-tools-exec", operation)?;
    let providers = payload
        .get("providers")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec!["python".into(), "shell".into()]);
    let config = payload.get("config").and_then(Value::as_object);
    let executors = providers
        .into_iter()
        .map(|provider| build_exec_executor(&provider, config.and_then(|cfg| cfg.get(&provider))))
        .collect::<Result<Vec<_>, _>>()?;
    let orchestrator = ExecOrchestrator::new(executors);
    block_on("or-tools-exec", operation, orchestrator.execute(request)).and_then(json_value)
}

fn build_exec_executor(
    provider: &str,
    config: Option<&Value>,
) -> Result<Arc<dyn CodeExecutor>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let executor: Arc<dyn CodeExecutor> = match provider {
        "python" => Arc::new(PythonExecutor),
        "shell" => Arc::new(ShellExecutor),
        "e2b" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (
                cfg.and_then(|v| get_str(v, "endpoint")),
                cfg.and_then(|v| get_str(v, "api_key")),
            ) {
                E2BExecutor::with_config(client, endpoint, api_key)
            } else {
                E2BExecutor::from_env()
                    .map_err(|error| invocation("or-tools-exec", "execute", error))?
            },
        ),
        "bearly" => Arc::new(
            if let (Some(endpoint), Some(api_key)) = (
                cfg.and_then(|v| get_str(v, "endpoint")),
                cfg.and_then(|v| get_str(v, "api_key")),
            ) {
                BearlyExecutor::with_config(client, endpoint, api_key)
            } else {
                BearlyExecutor::from_env()
                    .map_err(|error| invocation("or-tools-exec", "execute", error))?
            },
        ),
        "daytona" => Arc::new(if cfg.is_some() {
            return Err(BridgeError::InvalidInput(
                "daytona currently uses environment-based connection setup only".into(),
            ));
        } else {
            DaytonaExecutor::from_env()
                .map_err(|error| invocation("or-tools-exec", "execute", error))?
        }),
        other => return Err(unsupported_provider("or-tools-exec", other)),
    };
    Ok(executor)
}
