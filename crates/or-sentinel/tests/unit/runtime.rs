use or_conduit::{
    CompletionMessage, CompletionResponse, ConduitProvider, FinishReason, MessageRole,
};
use or_core::{RetryPolicy, TokenBudget, TokenUsage};
use or_forge::{ForgeRegistry, ForgeTool};
use or_sentinel::domain::contracts::{PlanExecuteAgentTrait, SentinelAgentTrait};
use or_sentinel::{PlanExecuteAgent, SentinelAgent, SentinelConfig, StepOutcome};
use schemars::schema::RootSchema;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct SequenceProvider {
    responses: Arc<Mutex<VecDeque<String>>>,
}

impl SequenceProvider {
    fn new(responses: &[&str]) -> Self {
        Self {
            responses: Arc::new(Mutex::new(
                responses.iter().map(|value| (*value).to_owned()).collect(),
            )),
        }
    }
}

impl ConduitProvider for SequenceProvider {
    async fn complete_messages(
        &self,
        _messages: Vec<CompletionMessage>,
    ) -> Result<CompletionResponse, or_conduit::ConduitError> {
        let text = self
            .responses
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or_else(|| "{\"type\":\"final_answer\",\"answer\":\"done\"}".to_owned());
        Ok(CompletionResponse {
            text,
            usage: TokenUsage::default(),
            finish_reason: FinishReason::Stop,
        })
    }
}

#[tokio::test]
async fn sentinel_react_loop_invokes_tool_and_returns_final_answer() {
    let agent = SentinelAgent::new(
        SequenceProvider::new(&[
            "{\"type\":\"tool_call\",\"tool_name\":\"echo\",\"args\":{\"value\":1}}",
            "{\"type\":\"final_answer\",\"answer\":\"done\"}",
        ]),
        registry(),
    )
    .unwrap();
    let result = agent
        .run(initial_state("Use the tool"), config(2))
        .await
        .unwrap();
    match result {
        StepOutcome::FinalAnswer { answer, state } => {
            assert_eq!(answer, "done");
            assert!(
                serde_json::to_string(&state["messages"])
                    .unwrap()
                    .contains("\"tool\"")
            );
            // Regression for audit #4: sentinel-internal control data
            // (config, step index, pending/last tool call, final answer)
            // must no longer leak into the user-facing DynState.
            for key in state.keys() {
                assert!(
                    !key.starts_with("__sentinel_"),
                    "sentinel-internal key leaked into user state: {key}"
                );
            }
        }
        other => panic!("unexpected outcome: {other:?}"),
    }
}

#[tokio::test]
async fn sentinel_returns_step_limit_reached_when_loop_does_not_finish() {
    let agent = SentinelAgent::new(
        SequenceProvider::new(&["{\"type\":\"tool_call\",\"tool_name\":\"echo\",\"args\":{}}"]),
        registry(),
    )
    .unwrap();
    let result = agent.run(initial_state("Loop"), config(1)).await.unwrap();
    assert!(matches!(
        result,
        StepOutcome::StepLimitReached { steps_taken: 1, .. }
    ));
}

#[tokio::test]
async fn plan_execute_agent_generates_plan_and_collects_notes() {
    let agent = PlanExecuteAgent::new(
        SequenceProvider::new(&[
            "{\"steps\":[\"look up context\",\"summarize result\"]}",
            "{\"type\":\"final_answer\",\"answer\":\"note one\"}",
            "{\"type\":\"final_answer\",\"answer\":\"note two\"}",
        ]),
        registry(),
    )
    .unwrap();
    let result = agent
        .run(initial_state("Make a plan"), config(2))
        .await
        .unwrap();
    match result {
        StepOutcome::FinalAnswer { answer, state } => {
            assert!(answer.contains("note one"));
            assert!(state.contains_key("plan"));
        }
        other => panic!("unexpected outcome: {other:?}"),
    }
}

fn initial_state(prompt: &str) -> or_core::DynState {
    let mut state = or_core::DynState::new();
    state.insert(
        "messages".to_owned(),
        serde_json::to_value(vec![CompletionMessage::single_text(
            MessageRole::User,
            prompt,
        )])
        .unwrap(),
    );
    state
}

fn config(max_steps: u32) -> SentinelConfig {
    SentinelConfig {
        max_steps,
        step_budget: TokenBudget {
            max_context_tokens: 32_000,
            max_completion_tokens: 1_024,
        },
        tool_retry: RetryPolicy::no_retry(),
    }
}

fn registry() -> ForgeRegistry {
    let mut registry = ForgeRegistry::new();
    registry
        .register(
            ForgeTool {
                name: "echo".to_owned(),
                description: "Echoes input".to_owned(),
                input_schema: schema_object(),
            },
            |args| async move { Ok(serde_json::json!({ "echo": args })) },
        )
        .unwrap();
    registry
}

fn schema_object() -> RootSchema {
    schemars::schema::RootSchema {
        meta_schema: None,
        schema: schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
                schemars::schema::InstanceType::Object,
            ))),
            ..Default::default()
        },
        definitions: Default::default(),
    }
}
