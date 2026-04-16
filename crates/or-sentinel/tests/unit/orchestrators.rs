use or_core::DynState;
use or_sentinel::domain::contracts::{PlanExecuteAgentTrait, SentinelAgentTrait};
use or_sentinel::{SentinelConfig, SentinelError, SentinelOrchestrator, StepOutcome};

struct OkSentinel;
struct ErrSentinel;
struct OkPlanner;
struct ErrPlanner;

impl SentinelAgentTrait for OkSentinel {
    async fn run(
        &self,
        initial_state: DynState,
        _config: SentinelConfig,
    ) -> Result<StepOutcome, SentinelError> {
        Ok(StepOutcome::FinalAnswer {
            answer: "ok".to_owned(),
            state: initial_state,
        })
    }
    async fn step(&self, _state: DynState, _step_index: u32) -> Result<StepOutcome, SentinelError> {
        unreachable!()
    }
}

impl SentinelAgentTrait for ErrSentinel {
    async fn run(
        &self,
        _initial_state: DynState,
        _config: SentinelConfig,
    ) -> Result<StepOutcome, SentinelError> {
        Err(SentinelError::InvalidState("boom".to_owned()))
    }
    async fn step(&self, _state: DynState, _step_index: u32) -> Result<StepOutcome, SentinelError> {
        unreachable!()
    }
}

impl PlanExecuteAgentTrait for OkPlanner {
    async fn run(
        &self,
        initial_state: DynState,
        _config: SentinelConfig,
    ) -> Result<StepOutcome, SentinelError> {
        Ok(StepOutcome::FinalAnswer {
            answer: "planned".to_owned(),
            state: initial_state,
        })
    }
    async fn plan(
        &self,
        _initial_state: DynState,
    ) -> Result<Vec<or_sentinel::PlanStep>, SentinelError> {
        Ok(Vec::new())
    }
}

impl PlanExecuteAgentTrait for ErrPlanner {
    async fn run(
        &self,
        _initial_state: DynState,
        _config: SentinelConfig,
    ) -> Result<StepOutcome, SentinelError> {
        Err(SentinelError::InvalidResponse("bad plan".to_owned()))
    }
    async fn plan(
        &self,
        _initial_state: DynState,
    ) -> Result<Vec<or_sentinel::PlanStep>, SentinelError> {
        Ok(Vec::new())
    }
}

#[tokio::test]
async fn orchestrator_runs_agent_happy_path() {
    let result = SentinelOrchestrator
        .run_agent(&OkSentinel, DynState::new(), config())
        .await
        .unwrap();
    assert!(matches!(result, StepOutcome::FinalAnswer { answer, .. } if answer == "ok"));
}

#[tokio::test]
async fn orchestrator_runs_agent_failure_path() {
    let result = SentinelOrchestrator
        .run_agent(&ErrSentinel, DynState::new(), config())
        .await;
    assert_eq!(result, Err(SentinelError::InvalidState("boom".to_owned())));
}

#[tokio::test]
async fn orchestrator_runs_planned_agent_happy_path() {
    let result = SentinelOrchestrator
        .run_planned_agent(&OkPlanner, DynState::new(), config())
        .await
        .unwrap();
    assert!(matches!(result, StepOutcome::FinalAnswer { answer, .. } if answer == "planned"));
}

#[tokio::test]
async fn orchestrator_runs_planned_agent_failure_path() {
    let result = SentinelOrchestrator
        .run_planned_agent(&ErrPlanner, DynState::new(), config())
        .await;
    assert_eq!(
        result,
        Err(SentinelError::InvalidResponse("bad plan".to_owned()))
    );
}

fn config() -> SentinelConfig {
    SentinelConfig {
        max_steps: 1,
        step_budget: or_core::TokenBudget {
            max_context_tokens: 8_192,
            max_completion_tokens: 512,
        },
        tool_retry: or_core::RetryPolicy::no_retry(),
    }
}
