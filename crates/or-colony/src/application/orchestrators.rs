use crate::domain::contracts::ColonyAgentTrait;
use crate::domain::entities::ColonyResult;
use crate::domain::errors::ColonyError;
use crate::infra::adapters::{record_message, result_from_parts, seed_message};
use crate::infra::implementations::ColonyRoster;
use or_core::DynState;

#[derive(Clone, Default)]
pub struct ColonyOrchestrator {
    roster: ColonyRoster,
}

impl ColonyOrchestrator {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_member<A>(mut self, name: &str, role: &str, agent: A) -> Result<Self, ColonyError>
    where
        A: ColonyAgentTrait,
    {
        self.roster.add_member(name, role, agent)?;
        Ok(self)
    }

    pub async fn coordinate(
        &self,
        mut initial_state: DynState,
    ) -> Result<ColonyResult, ColonyError> {
        let span = tracing::info_span!(
            "colony.coordinate",
            otel.name = "colony.coordinate",
            status = tracing::field::Empty
        );
        let _guard = span.enter();
        let result = async {
            if self.roster.is_empty() {
                return Err(ColonyError::EmptyColony);
            }
            let mut transcript = vec![seed_message(&initial_state)?];
            for worker in self.roster.workers() {
                let reply = worker
                    .agent
                    .respond(
                        initial_state.clone(),
                        transcript.clone(),
                        worker.member.clone(),
                    )
                    .await?;
                record_message(&mut initial_state, &reply)?;
                transcript.push(reply);
            }
            result_from_parts(initial_state, transcript)
        }
        .await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}
