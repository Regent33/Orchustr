//! Stress tests: sequential graph chains, concurrent vector store, rapid builds.

use or_core::{InMemoryVectorStore, VectorStore, OrchState};
use or_loom::{GraphBuilder, LoomOrchestrator, NodeResult};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct StressState {
    counter: u32,
}

impl OrchState for StressState {}

#[tokio::test]
async fn sequential_rapid_graph_executions() {
    let graph = GraphBuilder::new()
        .add_node("inc", |mut s: StressState| async move {
            s.counter += 1;
            NodeResult::advance(s)
        })
        .set_entry("inc")
        .set_exit("inc")
        .build()
        .unwrap();

    for i in 0..50u32 {
        let result = LoomOrchestrator
            .execute_graph(&graph, StressState { counter: i })
            .await
            .unwrap();
        assert_eq!(result.counter, i + 1);
    }
}

#[tokio::test]
async fn concurrent_vector_store_upserts() {
    let store = InMemoryVectorStore::new();
    let mut handles = Vec::new();

    for i in 0..100 {
        let store = store.clone();
        handles.push(tokio::spawn(async move {
            store
                .upsert(
                    &format!("item-{i}"),
                    vec![i as f32; 3],
                    json!({"index": i}),
                )
                .await
                .unwrap();
        }));
    }

    futures::future::join_all(handles).await;

    let results = store.query(vec![50.0; 3], 5).await.unwrap();
    assert!(
        !results.is_empty(),
        "concurrent upserts should produce queryable entries"
    );
}

#[tokio::test]
async fn stress_20_node_linear_chain() {
    let mut builder = GraphBuilder::new();
    let nodes: Vec<String> = (0..20).map(|i| format!("n{i}")).collect();
    for name in &nodes {
        let name_clone = name.clone();
        builder = builder.add_node(&name_clone, |mut s: StressState| async move {
            s.counter += 1;
            NodeResult::advance(s)
        });
    }
    for pair in nodes.windows(2) {
        builder = builder.add_edge(&pair[0], &pair[1]);
    }
    builder = builder.set_entry(&nodes[0]).set_exit(nodes.last().unwrap());
    let graph = builder.build().unwrap();

    let result = LoomOrchestrator
        .execute_graph(&graph, StressState { counter: 0 })
        .await
        .unwrap();
    assert_eq!(result.counter, 20);
}

#[tokio::test]
async fn rapid_graph_build_and_execute_cycle() {
    // Rapidly build and execute many small graphs to test for leaks/panics
    for i in 0..100u32 {
        let graph = GraphBuilder::new()
            .add_node("only", |mut s: StressState| async move {
                s.counter *= 2;
                NodeResult::advance(s)
            })
            .set_entry("only")
            .set_exit("only")
            .build()
            .unwrap();
        let result = LoomOrchestrator
            .execute_graph(&graph, StressState { counter: i + 1 })
            .await
            .unwrap();
        assert_eq!(result.counter, (i + 1) * 2);
    }
}
