use or_core::OrchState;
use or_relay::{RelayBuilder, RelayError, RelayExecutor, RelayOrchestrator};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RelayState {
    left: Option<String>,
    right: Option<String>,
}

impl OrchState for RelayState {
    fn merge(current: &Self, patch: Self) -> Self {
        Self {
            left: patch.left.or_else(|| current.left.clone()),
            right: patch.right.or_else(|| current.right.clone()),
        }
    }
}

#[tokio::test]
async fn execute_parallel_merges_branch_patches() {
    let plan = RelayBuilder::new()
        .add_branch("left", |_state: RelayState| async move {
            Ok(RelayState {
                left: Some("a".to_owned()),
                right: None,
            })
        })
        .add_branch("right", |_state: RelayState| async move {
            Ok(RelayState {
                left: None,
                right: Some("b".to_owned()),
            })
        })
        .build()
        .unwrap();
    let result = RelayOrchestrator
        .execute_parallel(
            &RelayExecutor,
            &plan,
            RelayState {
                left: None,
                right: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(result.left, Some("a".to_owned()));
    assert_eq!(result.right, Some("b".to_owned()));
}

#[tokio::test]
async fn execute_parallel_surfaces_branch_failures() {
    let plan = RelayBuilder::new()
        .add_branch("broken", |_state: RelayState| async move {
            Err(RelayError::BranchExecution("boom".to_owned()))
        })
        .build()
        .unwrap();
    let result = RelayOrchestrator
        .execute_parallel(
            &RelayExecutor,
            &plan,
            RelayState {
                left: None,
                right: None,
            },
        )
        .await;
    assert_eq!(result, Err(RelayError::BranchExecution("boom".to_owned())));
}
