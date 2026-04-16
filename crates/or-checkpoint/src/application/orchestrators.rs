use crate::domain::entities::CheckpointRecord;
use crate::domain::errors::CheckpointError;
use crate::infra::implementations::CheckpointGate;
use or_core::{OrchState, PersistenceBackend};

#[derive(Debug, Clone, Default)]
pub struct CheckpointOrchestrator;

impl CheckpointOrchestrator {
    pub async fn pause<T: OrchState, B: PersistenceBackend>(
        &self,
        gate: &CheckpointGate<B>,
        checkpoint_id: &str,
        resume_from: &str,
        state: &T,
    ) -> Result<(), CheckpointError> {
        let span = tracing::info_span!(
            "checkpoint.pause",
            otel.name = "checkpoint.pause",
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = gate.pause(checkpoint_id, resume_from, state).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }

    pub async fn resume<T: OrchState, B: PersistenceBackend>(
        &self,
        gate: &CheckpointGate<B>,
        checkpoint_id: &str,
    ) -> Result<CheckpointRecord<T>, CheckpointError> {
        let span = tracing::info_span!(
            "checkpoint.resume",
            otel.name = "checkpoint.resume",
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = gate.resume(checkpoint_id).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}
