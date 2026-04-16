use crate::domain::errors::RelayError;
use crate::infra::implementations::{RelayExecutor, RelayPlan};
use or_core::OrchState;

#[derive(Debug, Clone, Default)]
pub struct RelayOrchestrator;

impl RelayOrchestrator {
    pub async fn execute_parallel<T: OrchState>(
        &self,
        executor: &RelayExecutor,
        plan: &RelayPlan<T>,
        initial_state: T,
    ) -> Result<T, RelayError> {
        let span = tracing::info_span!(
            "relay.execute_parallel",
            otel.name = "relay.execute_parallel",
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = executor.execute(plan, initial_state).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}
