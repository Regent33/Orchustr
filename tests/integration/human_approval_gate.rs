use or_checkpoint::CheckpointGate;
use or_core::{DynState, InMemoryPersistenceBackend};

#[tokio::test]
async fn checkpoint_gate_round_trips_approval_state() {
    let gate = CheckpointGate::new("approval-graph", InMemoryPersistenceBackend::new());
    let mut state = DynState::new();
    state.insert("approval_required".into(), serde_json::json!(true));
    state.insert("request_id".into(), serde_json::json!("req-123"));

    gate.pause("await-human", "resume-node", &state)
        .await
        .expect("state should save");

    let checkpoint = gate
        .resume::<DynState>("await-human")
        .await
        .expect("state should load");

    assert_eq!(checkpoint.checkpoint_id, "await-human");
    assert_eq!(checkpoint.resume_from, "resume-node");
    assert_eq!(checkpoint.state["approval_required"], true);
    assert_eq!(checkpoint.state["request_id"], "req-123");
}
