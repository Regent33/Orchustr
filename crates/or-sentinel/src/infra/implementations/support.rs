use crate::domain::entities::SentinelConfig;
use crate::domain::errors::SentinelError;
use or_core::CoreOrchestrator;
use or_forge::ForgeRegistry;
use or_loom::LoomError;

pub(crate) async fn invoke_with_retry(
    registry: &ForgeRegistry,
    tool_name: &str,
    args: serde_json::Value,
    config: &SentinelConfig,
) -> Result<serde_json::Value, SentinelError> {
    let core = CoreOrchestrator::new();
    for attempt in 1..=config.tool_retry.max_attempts.max(1) {
        match registry.invoke(tool_name, args.clone()).await {
            Ok(result) => return Ok(result),
            Err(error) if attempt == config.tool_retry.max_attempts.max(1) => {
                return Err(SentinelError::Forge(error.to_string()));
            }
            Err(_) => {
                let delay = core
                    .next_retry_delay(&config.tool_retry, attempt)
                    .map_err(|error| SentinelError::Core(error.to_string()))?;
                tokio::time::sleep(delay).await;
            }
        }
    }
    Err(SentinelError::Forge("tool retry exhausted".to_owned()))
}

pub(crate) fn node_error(node: &str, error: SentinelError) -> LoomError {
    LoomError::NodeExecution {
        node: format!("sentinel::{node}"),
        message: error.to_string(),
    }
}
