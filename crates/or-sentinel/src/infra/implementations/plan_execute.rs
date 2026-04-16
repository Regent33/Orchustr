use crate::domain::contracts::PlanExecuteAgentTrait;
use crate::domain::entities::{PlanStep, SentinelConfig, StepOutcome};
use crate::domain::errors::SentinelError;
use crate::infra::adapters::parsing::parse_plan;
use crate::infra::adapters::state::{messages_from_state, write_messages};
use crate::infra::implementations::sentinel_agent::SentinelAgent;
use or_conduit::{CompletionMessage, ConduitProvider, ContentPart, MessageRole};
use or_core::DynState;
use or_forge::ForgeRegistry;

#[derive(Clone)]
pub struct PlanExecuteAgent<P> {
    planner: P,
    worker: SentinelAgent<P>,
}

impl<P> PlanExecuteAgent<P>
where
    P: ConduitProvider + Clone + Send + Sync + 'static,
{
    pub fn new(planner: P, registry: ForgeRegistry) -> Result<Self, SentinelError> {
        Ok(Self {
            worker: SentinelAgent::new(planner.clone(), registry)?,
            planner,
        })
    }
}

impl<P> PlanExecuteAgentTrait for PlanExecuteAgent<P>
where
    P: ConduitProvider + Clone + Send + Sync + 'static,
{
    async fn run(
        &self,
        mut initial_state: DynState,
        config: SentinelConfig,
    ) -> Result<StepOutcome, SentinelError> {
        let mut notes = Vec::new();
        let steps = self.plan(initial_state.clone()).await?;
        initial_state.insert(
            "plan".to_owned(),
            serde_json::to_value(&steps)
                .map_err(|error| SentinelError::Serialization(error.to_string()))?,
        );
        for step in steps {
            let mut messages = messages_from_state(&initial_state).unwrap_or_default();
            messages.push(CompletionMessage::single_text(
                MessageRole::User,
                format!(
                    "Execute plan step {}: {}",
                    step.step_index, step.description
                ),
            ));
            write_messages(&mut initial_state, &messages)?;
            let (outcome, next_state) = self
                .worker
                .step_once(initial_state, config.clone(), step.step_index)
                .await?;
            initial_state = next_state;
            if let StepOutcome::FinalAnswer { answer, .. } = outcome {
                notes.push(answer);
            }
        }
        Ok(StepOutcome::FinalAnswer {
            answer: if notes.is_empty() {
                "plan executed without a synthesized answer".to_owned()
            } else {
                notes.join("\n")
            },
            state: initial_state,
        })
    }

    async fn plan(&self, initial_state: DynState) -> Result<Vec<PlanStep>, SentinelError> {
        let objective = messages_from_state(&initial_state)?
            .into_iter()
            .flat_map(|message| message.content)
            .filter_map(|part| match part {
                ContentPart::Text { text } => Some(text),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        let response = self
            .planner
            .complete_text(&format!(
                "Return JSON {{\"steps\":[...]}} for this objective:\n{objective}"
            ))
            .await
            .map_err(|error| SentinelError::Conduit(error.to_string()))?;
        parse_plan(&response.text)
    }
}
