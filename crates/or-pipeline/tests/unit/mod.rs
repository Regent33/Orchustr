use or_core::OrchState;
use or_pipeline::{PipelineBuilder, PipelineError, PipelineOrchestrator};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PipelineState {
    text: String,
    summary: String,
}

impl OrchState for PipelineState {}

#[tokio::test]
async fn builder_rejects_duplicate_node_names() {
    let result = PipelineBuilder::<PipelineState>::new()
        .add_node("fetch", |state| async move { Ok(state) })
        .add_node("fetch", |state| async move { Ok(state) })
        .build();
    assert!(matches!(
        result,
        Err(PipelineError::DuplicateNode(name)) if name == "fetch"
    ));
}

#[tokio::test]
async fn execute_pipeline_runs_nodes_in_sequence() {
    let pipeline = PipelineBuilder::new()
        .add_node("fetch", |mut state: PipelineState| async move {
            state.text = "fetched".to_owned();
            Ok(state)
        })
        .add_node("summarize", |mut state: PipelineState| async move {
            state.summary = format!("summary: {}", state.text);
            Ok(state)
        })
        .build()
        .unwrap();
    let result = PipelineOrchestrator
        .execute_pipeline(
            &pipeline,
            PipelineState {
                text: String::new(),
                summary: String::new(),
            },
        )
        .await
        .unwrap();
    assert_eq!(result.summary, "summary: fetched");
}

#[tokio::test]
async fn execute_pipeline_surfaces_node_failures() {
    let pipeline = PipelineBuilder::new()
        .add_node("broken", |_state: PipelineState| async move {
            Err(PipelineError::NodeExecution("boom".to_owned()))
        })
        .build()
        .unwrap();
    let result = PipelineOrchestrator
        .execute_pipeline(
            &pipeline,
            PipelineState {
                text: String::new(),
                summary: String::new(),
            },
        )
        .await;
    assert_eq!(result, Err(PipelineError::NodeExecution("boom".to_owned())));
}
