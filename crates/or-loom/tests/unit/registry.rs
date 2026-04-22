use or_core::DynState;
use or_loom::{LoomError, LoomOrchestrator, NodeRegistry, NodeResult};
use or_schema::{EdgeSpec, GraphSpec, NodeSpec};
use serde_json::json;
use std::sync::{Arc, Mutex};

fn simple_spec(handler_one: &str, handler_two: &str) -> GraphSpec {
    GraphSpec {
        name: "registry-simple".to_owned(),
        version: "0.1.0".to_owned(),
        entry: "start".to_owned(),
        exits: vec!["finish".to_owned()],
        nodes: vec![
            NodeSpec {
                id: "start".to_owned(),
                handler: handler_one.to_owned(),
                metadata: json!({}),
            },
            NodeSpec {
                id: "finish".to_owned(),
                handler: handler_two.to_owned(),
                metadata: json!({}),
            },
        ],
        edges: vec![EdgeSpec {
            from: "start".to_owned(),
            to: "finish".to_owned(),
            condition: None,
        }],
    }
}

#[tokio::test]
async fn node_registry_compile_simple_graph() {
    let order = Arc::new(Mutex::new(Vec::<String>::new()));
    let mut registry = NodeRegistry::new();

    registry.register("tests::start", {
        let order = Arc::clone(&order);
        move |mut state: DynState| {
            let order = Arc::clone(&order);
            async move {
                order.lock().expect("order lock").push("start".to_owned());
                state.insert("step".to_owned(), json!("planned"));
                NodeResult::advance(state)
            }
        }
    });
    registry.register("tests::finish", {
        let order = Arc::clone(&order);
        move |mut state: DynState| {
            let order = Arc::clone(&order);
            async move {
                order.lock().expect("order lock").push("finish".to_owned());
                state.insert("status".to_owned(), json!("done"));
                NodeResult::advance(state)
            }
        }
    });

    let graph = registry
        .compile(&simple_spec("tests::start", "tests::finish"))
        .expect("graph compilation should succeed");
    let result = LoomOrchestrator
        .execute_graph(&graph, DynState::new())
        .await
        .expect("graph execution should succeed");

    assert_eq!(
        order.lock().expect("order lock").clone(),
        vec!["start".to_owned(), "finish".to_owned()]
    );
    assert_eq!(result.get("step"), Some(&json!("planned")));
    assert_eq!(result.get("status"), Some(&json!("done")));
}

#[test]
fn node_registry_unknown_handler_returns_error() {
    let registry = NodeRegistry::new();
    let result = registry.compile(&simple_spec("tests::missing", "tests::finish"));

    assert_eq!(
        result.err(),
        Some(LoomError::UnknownHandler("tests::missing".to_owned()))
    );
}

#[tokio::test]
async fn node_registry_conditional_edges_fall_back_to_default() {
    let order = Arc::new(Mutex::new(Vec::<String>::new()));
    let mut registry = NodeRegistry::new();

    registry.register("tests::think", {
        let order = Arc::clone(&order);
        move |mut state: DynState| {
            let order = Arc::clone(&order);
            async move {
                order.lock().expect("order lock").push("think".to_owned());
                state.insert("retry".to_owned(), json!(false));
                NodeResult::advance(state)
            }
        }
    });
    registry.register("tests::done", {
        let order = Arc::clone(&order);
        move |mut state: DynState| {
            let order = Arc::clone(&order);
            async move {
                order.lock().expect("order lock").push("done".to_owned());
                state.insert("status".to_owned(), json!("done"));
                NodeResult::advance(state)
            }
        }
    });
    registry.register("tests::retry", {
        let order = Arc::clone(&order);
        move |mut state: DynState| {
            let order = Arc::clone(&order);
            async move {
                order.lock().expect("order lock").push("retry".to_owned());
                state.insert("status".to_owned(), json!("retry"));
                NodeResult::advance(state)
            }
        }
    });
    registry.register_condition("tests::needs_retry", |state: &DynState| {
        Ok(state.get("retry") == Some(&json!(true)))
    });

    let spec = GraphSpec {
        name: "conditional-default".to_owned(),
        version: "0.1.0".to_owned(),
        entry: "think".to_owned(),
        exits: vec!["done".to_owned()],
        nodes: vec![
            NodeSpec {
                id: "think".to_owned(),
                handler: "tests::think".to_owned(),
                metadata: json!({}),
            },
            NodeSpec {
                id: "retry".to_owned(),
                handler: "tests::retry".to_owned(),
                metadata: json!({}),
            },
            NodeSpec {
                id: "done".to_owned(),
                handler: "tests::done".to_owned(),
                metadata: json!({}),
            },
        ],
        edges: vec![
            EdgeSpec {
                from: "think".to_owned(),
                to: "retry".to_owned(),
                condition: Some("tests::needs_retry".to_owned()),
            },
            EdgeSpec {
                from: "think".to_owned(),
                to: "done".to_owned(),
                condition: None,
            },
        ],
    };

    let graph = registry.compile(&spec).expect("graph compilation should succeed");
    let result = LoomOrchestrator
        .execute_graph(&graph, DynState::new())
        .await
        .expect("graph execution should succeed");

    assert_eq!(
        order.lock().expect("order lock").clone(),
        vec!["think".to_owned(), "done".to_owned()]
    );
    assert_eq!(result.get("status"), Some(&json!("done")));
}
