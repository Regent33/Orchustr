use or_core::{DynState, RetryPolicy, TokenBudget};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepOutcome {
    ToolCall {
        tool_name: String,
        args: serde_json::Value,
        step_index: u32,
    },
    FinalAnswer {
        answer: String,
        state: DynState,
    },
    StepLimitReached {
        last_state: DynState,
        steps_taken: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SentinelConfig {
    pub max_steps: u32,
    pub step_budget: TokenBudget,
    pub tool_retry: RetryPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlanStep {
    pub step_index: u32,
    pub description: String,
}
