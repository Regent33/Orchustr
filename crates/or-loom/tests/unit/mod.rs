mod graph;
mod stress;

use or_checkpoint::{CheckpointGate, CheckpointOrchestrator};
use or_core::{InMemoryPersistenceBackend, OrchState};
use or_loom::{GraphBuilder, LoomError, LoomOrchestrator, NodeResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct GraphState {
    path: Vec<String>,
}

impl OrchState for GraphState {}

#[tokio::test]
async fn execute_graph_handles_explicit_branching() {
    let graph = GraphBuilder::new()
        .add_node("start", |mut state: GraphState| async move {
            state.path.push("start".to_owned());
            NodeResult::branch(state, "right")
        })
        .add_node("left", |mut state: GraphState| async move {
            state.path.push("left".to_owned());
            NodeResult::advance(state)
        })
        .add_node("right", |mut state: GraphState| async move {
            state.path.push("right".to_owned());
            NodeResult::advance(state)
        })
        .add_node("end", |mut state: GraphState| async move {
            state.path.push("end".to_owned());
            NodeResult::advance(state)
        })
        .add_edge("start", "left")
        .add_edge("start", "right")
        .add_edge("right", "end")
        .add_edge("left", "end")
        .set_entry("start")
        .set_exit("end")
        .build()
        .unwrap();
    let result = LoomOrchestrator
        .execute_graph(&graph, GraphState { path: Vec::new() })
        .await
        .unwrap();
    assert_eq!(
        result.path,
        vec!["start".to_owned(), "right".to_owned(), "end".to_owned()]
    );
}

#[tokio::test]
async fn execute_graph_supports_pause_and_resume() {
    let gate = CheckpointGate::new("graph-x", InMemoryPersistenceBackend::new());
    let graph = GraphBuilder::new()
        .add_node("draft", |mut state: GraphState| async move {
            state.path.push("draft".to_owned());
            NodeResult::advance(state)
        })
        .add_node("approval", {
            let gate = gate.clone();
            move |mut state: GraphState| {
                let gate = gate.clone();
                async move {
                    state.path.push("approval".to_owned());
                    CheckpointOrchestrator
                        .pause(&gate, "approval-1", "publish", &state)
                        .await
                        .map_err(|error| LoomError::InvalidBranchTarget {
                            from: "approval".to_owned(),
                            to: error.to_string(),
                        })?;
                    NodeResult::pause("approval-1", state)
                }
            }
        })
        .add_node("publish", |mut state: GraphState| async move {
            state.path.push("publish".to_owned());
            NodeResult::advance(state)
        })
        .add_edge("draft", "approval")
        .add_edge("approval", "publish")
        .set_entry("draft")
        .set_exit("publish")
        .build()
        .unwrap();
    let paused = LoomOrchestrator
        .execute_graph(&graph, GraphState { path: Vec::new() })
        .await;
    assert_eq!(
        paused,
        Err(LoomError::Paused {
            checkpoint_id: "approval-1".to_owned()
        })
    );
    let checkpoint = CheckpointOrchestrator
        .resume::<GraphState, _>(&gate, "approval-1")
        .await
        .unwrap();
    let resumed = LoomOrchestrator
        .resume_graph(&graph, &checkpoint.resume_from, checkpoint.state)
        .await
        .unwrap();
    assert_eq!(
        resumed.path,
        vec![
            "draft".to_owned(),
            "approval".to_owned(),
            "publish".to_owned()
        ]
    );
}
