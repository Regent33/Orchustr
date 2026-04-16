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

fn task_state() -> DynState {
    let mut state = DynState::new();
    state.insert("task".to_owned(), serde_json::json!("Coordinate a result"));
    state
}
