//! Typed step-scoped context for sentinel topology nodes.
//!
//! Replaces the previous practice of stuffing five `__sentinel_*` keys
//! (config, step index, pending tool call, completed tool call, final
//! answer) into `DynState` and stripping them again every step. The
//! context now lives in a `tokio::task_local!` for the duration of one
//! `step_once` invocation; nodes read configuration from it and write
//! their outputs through it. `DynState` only carries data the user
//! actually cares about (messages, plan notes, etc.).

use crate::domain::entities::SentinelConfig;
use crate::domain::errors::SentinelError;
use std::sync::{Arc, Mutex};

/// Mutable outputs a topology node may surface during a single step:
/// either a final answer or a completed tool call. `pending_tool_call`
/// is the inter-node hand-off used by ReAct (`think` → `act`).
#[derive(Debug, Default)]
struct StepOutputs {
    final_answer: Option<String>,
    last_tool_call: Option<(String, serde_json::Value)>,
    pending_tool_call: Option<(String, serde_json::Value)>,
}

/// Read-only inputs (`config`, `step_index`) and mutable outputs
/// shared across all nodes within a single `step_once` call.
#[derive(Debug)]
pub(crate) struct SentinelStepContext {
    config: SentinelConfig,
    step_index: u32,
    outputs: Mutex<StepOutputs>,
}

impl SentinelStepContext {
    pub(crate) fn new(config: SentinelConfig, step_index: u32) -> Arc<Self> {
        Arc::new(Self {
            config,
            step_index,
            outputs: Mutex::new(StepOutputs::default()),
        })
    }

    pub(crate) fn config(&self) -> SentinelConfig {
        self.config.clone()
    }

    #[allow(dead_code)] // reserved for future topology features
    pub(crate) fn step_index(&self) -> u32 {
        self.step_index
    }

    pub(crate) fn set_final_answer(&self, answer: String) {
        self.outputs
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .final_answer = Some(answer);
    }

    pub(crate) fn take_final_answer(&self) -> Option<String> {
        self.outputs
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .final_answer
            .take()
    }

    pub(crate) fn set_pending_tool_call(&self, tool_name: String, args: serde_json::Value) {
        self.outputs
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .pending_tool_call = Some((tool_name, args));
    }

    pub(crate) fn take_pending_tool_call(
        &self,
    ) -> Result<(String, serde_json::Value), SentinelError> {
        self.outputs
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .pending_tool_call
            .take()
            .ok_or_else(|| SentinelError::InvalidState("pending tool call missing".to_owned()))
    }

    pub(crate) fn set_last_tool_call(&self, tool_name: String, args: serde_json::Value) {
        let mut outputs = self.outputs.lock().unwrap_or_else(|p| p.into_inner());
        outputs.last_tool_call = Some((tool_name, args));
        outputs.pending_tool_call = None;
    }

    pub(crate) fn take_last_tool_call(&self) -> Result<(String, serde_json::Value), SentinelError> {
        self.outputs
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .last_tool_call
            .take()
            .ok_or_else(|| SentinelError::InvalidState("completed tool call missing".to_owned()))
    }
}

tokio::task_local! {
    pub(crate) static SENTINEL_CTX: Arc<SentinelStepContext>;
}

/// Returns the active step context, mapped through `f`. Topology
/// nodes call this from inside the task-local scope established by
/// `SentinelAgent::step_once`. Returns `SentinelError::InvalidState`
/// if called outside that scope (which only happens if a user
/// invokes a bound topology graph directly without going through the
/// sentinel runtime).
pub(crate) fn with_context<R>(
    f: impl FnOnce(&SentinelStepContext) -> R,
) -> Result<R, SentinelError> {
    SENTINEL_CTX.try_with(|ctx| f(ctx)).map_err(|_| {
        SentinelError::InvalidState(
            "sentinel step context not set — \
                 topologies must run via SentinelAgent::run / step_once"
                .to_owned(),
        )
    })
}
