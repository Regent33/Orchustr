use or_checkpoint::{CheckpointError, CheckpointGate, CheckpointOrchestrator};
use or_core::{InMemoryPersistenceBackend, OrchState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CheckpointState {
    approved: bool,
}

impl OrchState for CheckpointState {}

#[tokio::test]
async fn pause_and_resume_round_trip_state() {
    let gate = CheckpointGate::new("graph-a", InMemoryPersistenceBackend::new());
    CheckpointOrchestrator
        .pause(
            &gate,
            "approval-1",
            "publish",
            &CheckpointState { approved: false },
        )
        .await
        .unwrap();
    let record = CheckpointOrchestrator
        .resume::<CheckpointState, _>(&gate, "approval-1")
        .await
        .unwrap();
    assert_eq!(record.resume_from, "publish");
    assert!(!record.state.approved);
}

#[tokio::test]
async fn resume_reports_missing_checkpoints() {
    let gate = CheckpointGate::new("graph-a", InMemoryPersistenceBackend::new());
    let result = CheckpointOrchestrator
        .resume::<CheckpointState, _>(&gate, "missing")
        .await;
    assert_eq!(
        result,
        Err(CheckpointError::MissingCheckpoint("missing".to_owned()))
    );
}
