use crate::domain::contracts::ColonyAgentTrait;
use crate::domain::entities::ColonyResult;
use crate::domain::errors::ColonyError;
use crate::infra::adapters::{record_message, result_from_parts, seed_message};
use crate::infra::implementations::ColonyRoster;
use futures::future::try_join_all;
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

    /// Runs colony members **sequentially**, threading each member's
    /// reply into the next member's transcript before invoking it.
    ///
    /// This is the right shape for cascading hand-off workflows
    /// (Researcher → Writer → Editor) where each member sees the
    /// previous members' output. For independent fan-out (each
    /// member sees only the seed task), use
    /// [`coordinate_parallel`][Self::coordinate_parallel] instead.
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

    /// Runs all colony members **concurrently** with the same seed
    /// transcript and merges their replies into the final state.
    ///
    /// Each member's `respond` future is polled together via
    /// `futures::try_join_all`; if any member errors the whole call
    /// short-circuits with that error. Replies are recorded into the
    /// transcript in roster order (not completion order) so the
    /// resulting `ColonyResult::transcript` is deterministic.
    ///
    /// Use this for independent-perspective workflows (e.g.
    /// "ask three reviewers in parallel"). Use
    /// [`coordinate`][Self::coordinate] when later members must see
    /// earlier members' output.
    pub async fn coordinate_parallel(
        &self,
        mut initial_state: DynState,
    ) -> Result<ColonyResult, ColonyError> {
        let span = tracing::info_span!(
            "colony.coordinate_parallel",
            otel.name = "colony.coordinate_parallel",
            members = self.roster.workers().len(),
            status = tracing::field::Empty
        );
        let _guard = span.enter();
        let result = async {
            if self.roster.is_empty() {
                return Err(ColonyError::EmptyColony);
            }
            let seed = seed_message(&initial_state)?;
            let initial_transcript = vec![seed.clone()];
            // Fan out: every worker sees the same initial state and
            // the same single-message transcript (the seed).
            let futures = self.roster.workers().iter().map(|worker| {
                let agent = worker.agent.clone();
                let state = initial_state.clone();
                let transcript = initial_transcript.clone();
                let member = worker.member.clone();
                async move { agent.respond(state, transcript, member).await }
            });
            let replies = try_join_all(futures).await?;

            // Fan in: order is roster-order (deterministic) regardless
            // of which task completed first.
            let mut transcript = initial_transcript;
            for reply in replies {
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
