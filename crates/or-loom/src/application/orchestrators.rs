use crate::domain::errors::LoomError;
use crate::infra::implementations::ExecutionGraph;
use or_core::OrchState;

#[derive(Debug, Clone, Default)]
pub struct LoomOrchestrator;

impl LoomOrchestrator {
    pub async fn execute_graph<T: OrchState>(
        &self,
        graph: &ExecutionGraph<T>,
        initial_state: T,
    ) -> Result<T, LoomError> {
        let span = tracing::info_span!(
            "loom.execute_graph",
            otel.name = "loom.execute_graph",
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = graph.execute(initial_state).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }

    pub async fn resume_graph<T: OrchState>(
        &self,
        graph: &ExecutionGraph<T>,
        start_at: &str,
        state: T,
    ) -> Result<T, LoomError> {
        let span = tracing::info_span!(
            "loom.resume_graph",
            otel.name = "loom.resume_graph",
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = graph.execute_from(start_at, state).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}
