use or_compass::CompassRouterBuilder;
use or_core::DynState;
use or_loom::{GraphBuilder, NodeResult};

#[tokio::test]
async fn graph_routes_state_through_compass_selected_path() {
    let router = CompassRouterBuilder::<DynState>::new()
        .add_route("priority", |state| state["priority"] == "high")
        .add_route("standard", |_| true)
        .set_default("standard")
        .build()
        .expect("router should build");

    let graph = GraphBuilder::<DynState>::new()
        .add_node("route", move |state: DynState| {
            let router = router.clone();
            async move {
                let selection = router.select(&state).expect("route should resolve");
                NodeResult::branch(state, selection.route)
            }
        })
        .add_node("priority", |mut state: DynState| async move {
            state.insert("path".into(), serde_json::json!("priority"));
            NodeResult::branch(state, "done")
        })
        .add_node("standard", |mut state: DynState| async move {
            state.insert("path".into(), serde_json::json!("standard"));
            NodeResult::branch(state, "done")
        })
        .add_node("done", |mut state: DynState| async move {
            state.insert("visited_done".into(), serde_json::json!(true));
            NodeResult::advance(state)
        })
        .add_edge("route", "priority")
        .add_edge("route", "standard")
        .add_edge("priority", "done")
        .add_edge("standard", "done")
        .set_entry("route")
        .set_exit("done")
        .build()
        .expect("graph should build");

    let mut state = DynState::new();
    state.insert("priority".into(), serde_json::json!("high"));

    let result = graph.execute(state).await.expect("graph should execute");
    assert_eq!(result["path"], "priority");
    assert_eq!(result["visited_done"], true);
}
