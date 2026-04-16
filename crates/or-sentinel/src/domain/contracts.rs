#![allow(async_fn_in_trait)]

use crate::domain::entities::{PlanStep, SentinelConfig, StepOutcome};
use crate::domain::errors::SentinelError;
use or_core::DynState;

#[cfg_attr(test, mockall::automock)]
pub trait SentinelAgentTrait: Send + Sync + 'static {
    async fn run(
        &self,
        initial_state: DynState,
        config: SentinelConfig,
    ) -> Result<StepOutcome, SentinelError>;

    async fn step(&self, state: DynState, step_index: u32) -> Result<StepOutcome, SentinelError>;
}

#[cfg_attr(test, mockall::automock)]
pub trait PlanExecuteAgentTrait: Send + Sync + 'static {
    async fn run(
        &self,
        initial_state: DynState,
        config: SentinelConfig,
    ) -> Result<StepOutcome, SentinelError>;

    async fn plan(&self, initial_state: DynState) -> Result<Vec<PlanStep>, SentinelError>;
}
