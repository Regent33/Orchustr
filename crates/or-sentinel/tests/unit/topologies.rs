use or_conduit::{
    CompletionMessage, CompletionResponse, ConduitProvider, FinishReason, MessageRole,
};
use or_core::{DynState, RetryPolicy, TokenBudget, TokenUsage};
use or_forge::{ForgeRegistry, ForgeTool};
use or_sentinel::domain::contracts::SentinelAgentTrait;
use or_sentinel::{
    ReActTopology, ReflectionTopology, SentinelAgent, SentinelAgentBuilder, SentinelConfig,
    StepOutcome,
};
use schemars::schema::RootSchema;
use std::collections::VecDeque;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Clone)]
struct SequenceProvider {
    responses: Arc<Mutex<VecDeque<String>>>,
    calls: Arc<AtomicUsize>,
}

impl SequenceProvider {
    fn new(responses: &[&str]) -> Self {
        Self {
            responses: Arc::new(Mutex::new(
                responses.iter().map(|value| (*value).to_owned()).collect(),
            )),
            calls: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn call_count(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}

impl ConduitProvider for SequenceProvider {
    async fn complete_messages(
        &self,
        _messages: Vec<CompletionMessage>,
    ) -> Result<CompletionResponse, or_conduit::ConduitError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
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
async fn topology_react_matches_legacy() {
    let provider = SequenceProvider::new(&[]);
    let legacy = SentinelAgent::new(provider.clone(), registry()).unwrap();
    let builder = SentinelAgentBuilder::new()
        .topology(ReActTopology::default())
        .conduit(provider)
        .tool_registry(registry())
        .build()
        .unwrap();

    assert_eq!(legacy.graph_inspection(), builder.graph_inspection());
}

#[tokio::test]
async fn topology_reflection_max_iterations() {
    let provider = SequenceProvider::new(&["revise", "revise"]);
    let agent = SentinelAgentBuilder::new()
        .topology(ReflectionTopology::new(2))
        .conduit(provider.clone())
        .tool_registry(registry())
        .build()
        .unwrap();

    let result = agent
        .run(initial_state("Draft an answer"), config(1))
        .await
        .unwrap();

    match result {
        StepOutcome::FinalAnswer { state, .. } => {
            assert_eq!(provider.call_count(), 2);
            assert_eq!(state["reflection_iterations"], serde_json::json!(2));
        }
        other => panic!("unexpected outcome: {other:?}"),
    }
}

#[tokio::test]
async fn topology_plan_execute_step_ordering() {
    let provider = SequenceProvider::new(&[
        "{\"steps\":[\"step one\",\"step two\",\"step three\"]}",
        "{\"type\":\"final_answer\",\"answer\":\"note one\"}",
        "{\"type\":\"final_answer\",\"answer\":\"note two\"}",
        "{\"type\":\"final_answer\",\"answer\":\"note three\"}",
    ]);
    let agent = SentinelAgentBuilder::new()
        .topology(or_sentinel::PlanExecuteTopology::default())
        .conduit(provider)
        .tool_registry(registry())
        .build()
        .unwrap();

    let result = agent
        .run(initial_state("Execute the plan"), config(1))
        .await
        .unwrap();

    match result {
        StepOutcome::FinalAnswer { state, .. } => {
            assert_eq!(
                state["plan_execution_order"],
                serde_json::json!(["step one", "step two", "step three"])
            );
        }
        other => panic!("unexpected outcome: {other:?}"),
    }
}

fn initial_state(prompt: &str) -> DynState {
    let mut state = DynState::new();
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
    RootSchema {
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
