use crate::domain::errors::PipelineError;
use crate::infra::implementations::Pipeline;
use or_core::OrchState;

#[derive(Debug, Clone, Default)]
pub struct PipelineOrchestrator;

impl PipelineOrchestrator {
    pub async fn execute_pipeline<T: OrchState>(
        &self,
        pipeline: &Pipeline<T>,
        initial_state: T,
    ) -> Result<T, PipelineError> {
        let span = tracing::info_span!(
            "pipeline.execute_pipeline",
            otel.name = "pipeline.execute_pipeline",
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = pipeline.execute(initial_state).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}
