use crate::domain::entities::SentinelConfig;
use crate::domain::errors::SentinelError;
use or_core::CoreOrchestrator;
use or_forge::{ForgeError, ForgeRegistry};
use or_loom::LoomError;

/// Classifies a `ForgeError` as retriable or terminal.
///
/// Terminal errors (`UnknownTool`, `InvalidArguments`, `DuplicateTool`)
/// are deterministic — the same call will fail the same way next time —
/// so retrying just burns the attempt budget. `Invocation` is treated
/// as potentially transient (network/upstream/rate-limit) and is
/// allowed to retry.
fn is_retriable(error: &ForgeError) -> bool {
    matches!(error, ForgeError::Invocation(_))
}

pub(crate) async fn invoke_with_retry(
    registry: &ForgeRegistry,
    tool_name: &str,
    args: serde_json::Value,
    config: &SentinelConfig,
) -> Result<serde_json::Value, SentinelError> {
    let core = CoreOrchestrator::new();
    let max_attempts = config.tool_retry.max_attempts.max(1);
    let mut last_error: Option<ForgeError> = None;
    for attempt in 1..=max_attempts {
        match registry.invoke(tool_name, args.clone()).await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if !is_retriable(&error) {
                    tracing::debug!(
                        target: "or_sentinel",
                        tool = tool_name,
                        attempt,
                        reason = %error,
                        "tool error is terminal; not retrying"
                    );
                    return Err(SentinelError::Forge(error.to_string()));
                }
                if attempt == max_attempts {
                    return Err(SentinelError::Forge(error.to_string()));
                }
                tracing::debug!(
                    target: "or_sentinel",
                    tool = tool_name,
                    attempt,
                    reason = %error,
                    "tool error is retriable; scheduling next attempt"
                );
                last_error = Some(error);
                let delay = core
                    .next_retry_delay(&config.tool_retry, attempt)
                    .map_err(SentinelError::from)?;
                tokio::time::sleep(delay).await;
            }
        }
    }
    // Loop only exits via early return on success/failure; this branch
    // remains as a safety net in case `max_attempts` somehow yields zero
    // iterations.
    Err(SentinelError::Forge(
        last_error
            .map(|error| error.to_string())
            .unwrap_or_else(|| "tool retry exhausted".to_owned()),
    ))
}

pub(crate) fn node_error(node: &str, error: SentinelError) -> LoomError {
    LoomError::NodeExecution {
        node: format!("sentinel::{node}"),
        message: error.to_string(),
    }
}
