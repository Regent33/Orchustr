use crate::domain::contracts::{PlanExecuteAgentTrait, SentinelAgentTrait};
use crate::domain::entities::{SentinelConfig, StepOutcome};
use crate::domain::errors::SentinelError;
use or_core::DynState;

#[derive(Debug, Clone, Default)]
pub struct SentinelOrchestrator;

impl SentinelOrchestrator {
    pub async fn run_agent<A: SentinelAgentTrait>(
        &self,
        agent: &A,
        initial_state: DynState,
        config: SentinelConfig,
    ) -> Result<StepOutcome, SentinelError> {
        let span = tracing::info_span!(
            "sentinel.run_agent",
            otel.name = "sentinel.run_agent",
            status = tracing::field::Empty
        );
        let _guard = span.enter();
        let result = agent.run(initial_state, config).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }

    pub async fn run_planned_agent<A: PlanExecuteAgentTrait>(
        &self,
        agent: &A,
        initial_state: DynState,
        config: SentinelConfig,
    ) -> Result<StepOutcome, SentinelError> {
        let span = tracing::info_span!(
            "sentinel.run_planned_agent",
            otel.name = "sentinel.run_planned_agent",
            status = tracing::field::Empty
        );
        let _guard = span.enter();
        let result = agent.run(initial_state, config).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}
