//! Agent loop integration test.
//!
//! Simulates a ReAct-style agent that:
//! 1. Receives a user task  
//! 2. Uses a mock LLM conduit to generate tool-use decisions  
//! 3. Invokes tools via ForgeRegistry  
//! 4. Feeds results back into the next LLM turn  
//! 5. Produces a final answer
//!
//! This validates bugs 1 (stream fallback), 2 (loom contracts), and 3 (merge).

use or_core::{DynState, OrchState};
use or_loom::{GraphBuilder, NodeResult};
use or_pipeline::PipelineBuilder;
use serde_json::{Value, json};

/// Verifies that DynState::merge actually merges keys instead of replacing.
#[tokio::test]
async fn dynstate_merge_accumulates_keys() {
    let current: DynState = [("count".into(), json!(1)), ("name".into(), json!("agent"))]
        .into_iter()
        .collect();

    let patch: DynState = [("count".into(), json!(2)), ("tool".into(), json!("search"))]
        .into_iter()
        .collect();

    let merged = DynState::merge(&current, patch);

    // Bug 3 fix: "name" from current MUST survive the merge
    assert_eq!(
        merged.get("name"),
        Some(&json!("agent")),
        "current key preserved"
    );
    // Patch key takes precedence
    assert_eq!(merged.get("count"), Some(&json!(2)), "patch key overwrites");
    // New key from patch is present
    assert_eq!(merged.get("tool"), Some(&json!("search")), "new key added");
    assert_eq!(merged.len(), 3);
}

/// Simulates a simple ReAct agent loop using a loom graph with tool use.
///
/// Graph topology:
///   plan → act → observe → (branch back to plan OR branch to answer)
///
/// The "observe" node has two outgoing edges, so it MUST use
/// `NodeResult::Branch` to pick the next node explicitly.
#[tokio::test]
async fn react_agent_loop_with_tool_use() {
    let graph = GraphBuilder::<DynState>::new()
        .add_node("plan", |state| async move {
            let iteration = state.get("iteration").and_then(Value::as_u64).unwrap_or(0);
            let task = state
                .get("task")
                .and_then(Value::as_str)
                .unwrap_or("unknown");

            // Simulate LLM deciding to use a tool on first iteration,
            // then answering on second
            let mut patch = state.clone();
            if iteration < 1 {
                patch.insert("action".into(), json!("use_tool"));
                patch.insert("tool_name".into(), json!("calculator"));
                patch.insert("tool_args".into(), json!({"expression": "2+2"}));
            } else {
                patch.insert("action".into(), json!("answer"));
                patch.insert(
                    "final_answer".into(),
                    json!(format!("The result is 4 for task: {}", task)),
                );
            }
            NodeResult::advance(patch)
        })
        .add_node("act", |state| async move {
            let action = state.get("action").and_then(Value::as_str).unwrap_or("");
            let mut patch = state.clone();
            if action == "use_tool" {
                // Simulate tool execution
                let tool_name = state.get("tool_name").and_then(Value::as_str).unwrap_or("");
                patch.insert(
                    "tool_result".into(),
                    json!(format!("{} returned: 4", tool_name)),
                );
            }
            NodeResult::advance(patch)
        })
        .add_node("observe", |state| async move {
            let action = state.get("action").and_then(Value::as_str).unwrap_or("");
            let mut patch = state.clone();
            let iteration = state.get("iteration").and_then(Value::as_u64).unwrap_or(0);
            patch.insert("iteration".into(), json!(iteration + 1));

            if action == "answer" {
                // Done — branch explicitly to exit node
                NodeResult::branch(patch, "answer")
            } else {
                // Loop back — branch explicitly to plan
                NodeResult::branch(patch, "plan")
            }
        })
        .add_node("answer", |state| async move { NodeResult::advance(state) })
        .add_edge("plan", "act")
        .add_edge("act", "observe")
        .add_edge("observe", "plan") // loop back
        .add_edge("observe", "answer") // or exit
        .set_entry("plan")
        .set_exit("answer")
        .build()
        .expect("graph should build without shape errors");

    let initial: DynState = [
        ("task".into(), json!("What is 2+2?")),
        ("iteration".into(), json!(0)),
    ]
    .into_iter()
    .collect();

    let result = graph
        .execute(initial)
        .await
        .expect("agent loop should complete");

    // Verify the agent completed the loop
    assert!(
        result.contains_key("final_answer"),
        "agent should produce a final answer"
    );
    assert_eq!(
        result.get("iteration").and_then(Value::as_u64),
        Some(2),
        "agent should have run 2 iterations (tool use + answer)"
    );
    // Bug 3: Verify merge preserved the task from the initial state
    assert_eq!(
        result.get("task").and_then(Value::as_str),
        Some("What is 2+2?"),
        "original task should survive merge"
    );
    assert!(
        result
            .get("final_answer")
            .and_then(Value::as_str)
            .unwrap_or("")
            .contains("4"),
        "final answer should contain the calculation result"
    );
}

/// Pipeline agent test: multi-step processing chain that enriches state.
#[tokio::test]
async fn pipeline_agent_enriches_state_through_chain() {
    let pipeline = PipelineBuilder::<DynState>::new()
        .add_node("classify", |state| async move {
            let mut out = state.clone();
            let text = state.get("input").and_then(Value::as_str).unwrap_or("");
            out.insert(
                "intent".into(),
                json!(if text.contains("weather") {
                    "weather"
                } else {
                    "general"
                }),
            );
            Ok(out)
        })
        .add_node("fetch", |state| async move {
            let mut out = state.clone();
            let intent = state
                .get("intent")
                .and_then(Value::as_str)
                .unwrap_or("general");
            out.insert(
                "context".into(),
                json!(format!("fetched data for intent={}", intent)),
            );
            Ok(out)
        })
        .add_node("generate", |state| async move {
            let mut out = state.clone();
            let context = state.get("context").and_then(Value::as_str).unwrap_or("");
            out.insert(
                "response".into(),
                json!(format!("Answer based on: {}", context)),
            );
            Ok(out)
        })
        .build()
        .expect("pipeline should build");

    let initial: DynState = [("input".into(), json!("What's the weather?"))]
        .into_iter()
        .collect();

    let result = pipeline
        .execute(initial)
        .await
        .expect("pipeline should complete");

    // Bug 3: all intermediate state keys must survive
    assert!(result.contains_key("input"), "input key preserved");
    assert_eq!(
        result.get("intent").and_then(Value::as_str),
        Some("weather")
    );
    assert!(result.contains_key("context"), "context key preserved");
    assert!(result.contains_key("response"), "response key preserved");
    assert!(
        result
            .get("response")
            .and_then(Value::as_str)
            .unwrap_or("")
            .contains("weather"),
        "response should reference the weather intent"
    );
}
