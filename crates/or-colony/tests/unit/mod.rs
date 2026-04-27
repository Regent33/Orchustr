use or_colony::domain::contracts::{ColonyAgentTrait, ColonyFuture};
use or_colony::{ColonyError, ColonyMember, ColonyMessage, ColonyOrchestrator};
use or_core::DynState;

#[derive(Clone)]
struct EchoAgent(&'static str);

impl ColonyAgentTrait for EchoAgent {
    fn respond(
        &self,
        _state: DynState,
        inbox: Vec<ColonyMessage>,
        member: ColonyMember,
    ) -> ColonyFuture {
        let suffix = self.0.to_owned();
        Box::pin(async move {
            Ok(ColonyMessage {
                from: member.name,
                to: "colony".to_owned(),
                content: format!(
                    "{}: {}",
                    suffix,
                    inbox
                        .last()
                        .map(|msg| msg.content.clone())
                        .unwrap_or_default()
                ),
            })
        })
    }
}

#[tokio::test]
async fn add_member_accepts_unique_members() {
    let orchestrator = ColonyOrchestrator::new()
        .add_member("planner", "planner", EchoAgent("plan"))
        .unwrap();
    let result = orchestrator.coordinate(task_state()).await.unwrap();
    assert!(result.summary.contains("planner"));
}

#[tokio::test]
async fn add_member_rejects_duplicates() {
    let result = ColonyOrchestrator::new()
        .add_member("planner", "planner", EchoAgent("plan"))
        .and_then(|orchestrator| {
            orchestrator.add_member("planner", "reviewer", EchoAgent("review"))
        });
    assert!(matches!(
        result,
        Err(ColonyError::DuplicateMember(name)) if name == "planner"
    ));
}

#[tokio::test]
async fn coordinate_passes_messages_and_aggregates_results() {
    let orchestrator = ColonyOrchestrator::new()
        .add_member("planner", "planner", EchoAgent("plan"))
        .unwrap()
        .add_member("reviewer", "reviewer", EchoAgent("review"))
        .unwrap();
    let result = orchestrator.coordinate(task_state()).await.unwrap();
    assert!(result.summary.contains("planner -> colony"));
    assert!(result.summary.contains("reviewer -> colony"));
}

#[tokio::test]
async fn coordinate_rejects_empty_roster() {
    let result = ColonyOrchestrator::new().coordinate(task_state()).await;
    assert_eq!(result, Err(ColonyError::EmptyColony));
}

#[tokio::test]
async fn coordinate_parallel_runs_each_member_against_seed_only() {
    // Regression for the audit's "multi-agent" overpromise: in
    // `coordinate_parallel` each member receives only the seed message,
    // so neither member sees the other's reply in its inbox. The
    // EchoAgent suffixes the last inbox message — both should echo the
    // seed, not each other.
    let orchestrator = ColonyOrchestrator::new()
        .add_member("planner", "planner", EchoAgent("plan"))
        .unwrap()
        .add_member("reviewer", "reviewer", EchoAgent("review"))
        .unwrap();
    let result = orchestrator
        .coordinate_parallel(task_state())
        .await
        .unwrap();

    let planner_reply = result
        .transcript
        .iter()
        .find(|m| m.from == "planner")
        .expect("planner reply must be present");
    let reviewer_reply = result
        .transcript
        .iter()
        .find(|m| m.from == "reviewer")
        .expect("reviewer reply must be present");

    // Both replies should reference the seed task content, not the
    // other member's output.
    assert!(
        planner_reply.content.contains("Coordinate a result"),
        "planner saw {planner_reply:?}"
    );
    assert!(
        reviewer_reply.content.contains("Coordinate a result"),
        "reviewer saw {reviewer_reply:?}"
    );
    // Neither reply should contain the other's distinguishing prefix.
    assert!(!planner_reply.content.contains("review:"));
    assert!(!reviewer_reply.content.contains("plan:"));

    // Roster ordering is preserved in the transcript regardless of
    // which task completed first.
    let from_order: Vec<_> = result.transcript.iter().map(|m| m.from.as_str()).collect();
    assert_eq!(from_order, vec!["user", "planner", "reviewer"]);
}

#[tokio::test]
async fn coordinate_parallel_rejects_empty_roster() {
    let result = ColonyOrchestrator::new()
        .coordinate_parallel(task_state())
        .await;
    assert_eq!(result, Err(ColonyError::EmptyColony));
}

fn task_state() -> DynState {
    let mut state = DynState::new();
    state.insert("task".to_owned(), serde_json::json!("Coordinate a result"));
    state
}
