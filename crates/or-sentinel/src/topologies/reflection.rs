use crate::domain::errors::SentinelError;
use crate::infra::adapters::context::with_context;
use crate::infra::adapters::state::messages_from_state;
use crate::infra::implementations::support::node_error;
use crate::topology::LoopTopology;
use or_conduit::{CompletionMessage, ConduitProvider, ContentPart, MessageRole};
use or_core::DynState;
use or_forge::ForgeRegistry;
use or_loom::{GraphBuilder, NodeResult};
use serde::Deserialize;

const DRAFT_KEY: &str = "__sentinel_reflection_draft";
const FEEDBACK_KEY: &str = "__sentinel_reflection_feedback";
const ITERATIONS_KEY: &str = "reflection_iterations";

/// A reflection loop topology for `or-sentinel`.
///
/// This built-in topology alternates between drafting and critique passes until
/// the critique accepts the result or the configured reflection limit is
/// reached. Before this topology existed, users had to fork the fixed sentinel
/// loop to add self-critique behavior.
#[derive(Debug, Clone)]
pub struct ReflectionTopology {
    max_reflections: u8,
}

impl ReflectionTopology {
    /// Creates a reflection topology with the given revision limit.
    #[must_use]
    pub fn new(max_reflections: u8) -> Self {
        Self { max_reflections }
    }

    pub(crate) fn max_reflections(&self) -> u8 {
        self.max_reflections
    }
}

impl Default for ReflectionTopology {
    fn default() -> Self {
        Self::new(2)
    }
}

impl LoopTopology for ReflectionTopology {
    fn build(&self) -> GraphBuilder<DynState> {
        GraphBuilder::new()
            .add_placeholder_node("draft")
            .add_placeholder_node("critique")
            .add_placeholder_node("exit")
            .add_edge("draft", "critique")
            .add_edge("critique", "draft")
            .add_edge("critique", "exit")
            .set_entry("draft")
            .set_exit("exit")
    }

    fn name(&self) -> &'static str {
        "reflection"
    }

    fn bind<P>(
        &self,
        builder: GraphBuilder<DynState>,
        provider: P,
        registry: ForgeRegistry,
    ) -> GraphBuilder<DynState>
    where
        P: ConduitProvider + Clone + Send + Sync + 'static,
    {
        bind_reflection(builder, self, provider, registry)
    }
}

pub(crate) fn bind_reflection<P>(
    builder: GraphBuilder<DynState>,
    topology: &ReflectionTopology,
    provider: P,
    _registry: ForgeRegistry,
) -> GraphBuilder<DynState>
where
    P: ConduitProvider + Clone + Send + Sync + 'static,
{
    let max_reflections = topology.max_reflections();
    builder
        .bind_node("draft", |state: DynState| async move {
            let updated_draft = draft_from_state(&state)?;
            let mut state = state;
            state.insert(DRAFT_KEY.to_owned(), serde_json::json!(updated_draft));
            state.remove(FEEDBACK_KEY);
            NodeResult::advance(state)
        })
        .bind_node("critique", move |state: DynState| {
            let provider = provider.clone();
            async move {
                let draft = state
                    .get(DRAFT_KEY)
                    .and_then(|value| value.as_str())
                    .map(ToOwned::to_owned)
                    .ok_or_else(|| {
                        node_error(
                            "critique",
                            SentinelError::InvalidState("draft missing".to_owned()),
                        )
                    })?;
                let iterations = state
                    .get(ITERATIONS_KEY)
                    .and_then(|value| value.as_u64())
                    .unwrap_or(0);
                let mut state = state;
                state.insert(ITERATIONS_KEY.to_owned(), serde_json::json!(iterations));

                if iterations >= u64::from(max_reflections) {
                    with_context(|ctx| ctx.set_final_answer(draft))
                        .map_err(|error| node_error("critique", error))?;
                    return NodeResult::branch(state, "exit");
                }

                let response = provider
                    .complete_messages(vec![CompletionMessage::single_text(
                        MessageRole::User,
                        format!(
                            "Critique this draft. Reply with JSON {{\"decision\":\"accept\"}} or \
{{\"decision\":\"revise\",\"feedback\":\"...\"}}.\n\n{draft}"
                        ),
                    )])
                    .await
                    .map_err(|error| {
                        node_error("critique", SentinelError::Conduit(error.to_string()))
                    })?;

                match parse_critique(&response.text)? {
                    CritiqueDecision::Accept { answer } => {
                        with_context(|ctx| ctx.set_final_answer(answer.unwrap_or(draft)))
                            .map_err(|error| node_error("critique", error))?;
                        NodeResult::branch(state, "exit")
                    }
                    CritiqueDecision::Revise { feedback } => {
                        state.insert(ITERATIONS_KEY.to_owned(), serde_json::json!(iterations + 1));
                        state.insert(FEEDBACK_KEY.to_owned(), serde_json::json!(feedback));
                        NodeResult::branch(state, "draft")
                    }
                }
            }
        })
        .bind_node("exit", |state: DynState| async move {
            NodeResult::advance(state)
        })
}

fn draft_from_state(state: &DynState) -> Result<String, or_loom::LoomError> {
    if let Some(draft) = state.get(DRAFT_KEY).and_then(|value| value.as_str()) {
        if let Some(feedback) = state.get(FEEDBACK_KEY).and_then(|value| value.as_str()) {
            return Ok(format!("{draft}\nRevision: {feedback}"));
        }
        return Ok(draft.to_owned());
    }

    let prompt = messages_from_state(state)
        .map_err(|error| node_error("draft", error))?
        .into_iter()
        .flat_map(|message| message.content)
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    Ok(format!("Draft: {prompt}"))
}

#[derive(Debug, Deserialize)]
struct CritiquePayload {
    decision: String,
    answer: Option<String>,
    feedback: Option<String>,
}

enum CritiqueDecision {
    Accept { answer: Option<String> },
    Revise { feedback: String },
}

fn parse_critique(raw: &str) -> Result<CritiqueDecision, or_loom::LoomError> {
    let trimmed = raw.trim();
    if let Ok(payload) = serde_json::from_str::<CritiquePayload>(trimmed) {
        return match payload.decision.as_str() {
            "accept" => Ok(CritiqueDecision::Accept {
                answer: payload.answer,
            }),
            "revise" => Ok(CritiqueDecision::Revise {
                feedback: payload.feedback.unwrap_or_else(|| "revise".to_owned()),
            }),
            other => Err(node_error(
                "critique",
                SentinelError::InvalidResponse(format!("unknown critique decision: {other}")),
            )),
        };
    }

    if trimmed.eq_ignore_ascii_case("accept") {
        return Ok(CritiqueDecision::Accept { answer: None });
    }
    if trimmed.is_empty() {
        return Err(node_error(
            "critique",
            SentinelError::InvalidResponse("critique response was empty".to_owned()),
        ));
    }
    Ok(CritiqueDecision::Revise {
        feedback: trimmed.to_owned(),
    })
}
