//! Graph execution stress and edge case tests.

use or_core::OrchState;
use or_loom::{GraphBuilder, LoomError, LoomOrchestrator, NodeResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CountState {
    steps: u32,
}

impl OrchState for CountState {}

#[tokio::test]
async fn linear_graph_executes_all_nodes() {
    let graph = GraphBuilder::new()
        .add_node("a", |mut s: CountState| async move {
            s.steps += 1;
            NodeResult::advance(s)
        })
        .add_node("b", |mut s: CountState| async move {
            s.steps += 1;
            NodeResult::advance(s)
        })
        .add_node("c", |mut s: CountState| async move {
            s.steps += 1;
            NodeResult::advance(s)
        })
        .add_edge("a", "b")
        .add_edge("b", "c")
        .set_entry("a")
        .set_exit("c")
        .build()
        .unwrap();
    let result = LoomOrchestrator
        .execute_graph(&graph, CountState { steps: 0 })
        .await
        .unwrap();
    assert_eq!(result.steps, 3);
}

#[tokio::test]
async fn empty_graph_is_rejected() {
    let result = GraphBuilder::<CountState>::new()
        .set_entry("a")
        .set_exit("a")
        .build();
    assert!(matches!(result, Err(LoomError::EmptyGraph)));
}

#[tokio::test]
async fn missing_entry_is_rejected() {
    let result = GraphBuilder::new()
        .add_node("a", |s: CountState| async move { NodeResult::advance(s) })
        .set_exit("a")
        .build();
    assert!(matches!(result, Err(LoomError::MissingEntry)));
}

#[tokio::test]
async fn missing_exit_is_rejected() {
    let result = GraphBuilder::new()
        .add_node("a", |s: CountState| async move { NodeResult::advance(s) })
        .set_entry("a")
        .build();
    assert!(matches!(result, Err(LoomError::MissingExit)));
}

#[tokio::test]
async fn invalid_branch_target_is_caught() {
    let graph = GraphBuilder::new()
        .add_node("start", |s: CountState| async move {
            NodeResult::branch(s, "nonexistent")
        })
        .add_node("end", |s: CountState| async move {
            NodeResult::advance(s)
        })
        .add_edge("start", "end")
        .set_entry("start")
        .set_exit("end")
        .build()
        .unwrap();
    let result = LoomOrchestrator
        .execute_graph(&graph, CountState { steps: 0 })
        .await;
    assert!(matches!(
        result,
        Err(LoomError::InvalidBranchTarget { .. })
    ));
}

#[tokio::test]
async fn node_execution_error_variant_is_usable() {
    // Verifies the new NodeExecution error variant works correctly
    let err = LoomError::NodeExecution {
        node: "sentinel::plan".into(),
        message: "model unavailable".into(),
    };
    assert!(err.to_string().contains("sentinel::plan"));
    assert!(err.to_string().contains("model unavailable"));
}
