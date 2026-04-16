use or_beacon::PromptBuilder;
use or_core::DynState;
use or_pipeline::{PipelineBuilder, PipelineOrchestrator};

#[tokio::test]
async fn pipeline_executes_beacon_rendering_across_nodes() {
    let prompt = PromptBuilder::new()
        .template("Summarize: {{text}}")
        .build()
        .expect("prompt should build");

    let pipeline = PipelineBuilder::<DynState>::new()
        .add_node("fetch", |mut state: DynState| async move {
            state.insert("text".into(), serde_json::json!("orchustr integration"));
            Ok(state)
        })
        .add_node("render", move |mut state: DynState| {
            let prompt = prompt.clone();
            async move {
                let rendered = prompt.render(&state).expect("template should render");
                state.insert("rendered".into(), serde_json::json!(rendered));
                Ok(state)
            }
        })
        .build()
        .expect("pipeline should build");

    let result = PipelineOrchestrator
        .execute_pipeline(&pipeline, DynState::new())
        .await
        .expect("pipeline should execute");

    assert_eq!(result["text"], "orchustr integration");
    assert_eq!(result["rendered"], "Summarize: orchustr integration");
}
