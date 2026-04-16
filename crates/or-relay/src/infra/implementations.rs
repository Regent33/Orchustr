use crate::domain::entities::RelayBranchMetadata;
use crate::domain::errors::RelayError;
use futures::stream::{FuturesUnordered, StreamExt};
use or_core::OrchState;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type BranchFuture<T> = Pin<Box<dyn Future<Output = Result<T, RelayError>> + Send + 'static>>;
type BranchHandler<T> = Arc<dyn Fn(T) -> BranchFuture<T> + Send + Sync + 'static>;

#[derive(Clone)]
struct RelayBranch<T: OrchState> {
    metadata: RelayBranchMetadata,
    handler: BranchHandler<T>,
}

#[derive(Clone)]
pub struct RelayBuilder<T: OrchState> {
    branches: Vec<RelayBranch<T>>,
}

impl<T: OrchState> Default for RelayBuilder<T> {
    fn default() -> Self {
        Self {
            branches: Vec::new(),
        }
    }
}

impl<T: OrchState> RelayBuilder<T> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn add_branch<F, Fut>(mut self, name: &str, handler: F) -> Self
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, RelayError>> + Send + 'static,
    {
        self.branches.push(RelayBranch {
            metadata: RelayBranchMetadata {
                name: name.to_owned(),
            },
            handler: Arc::new(move |state| Box::pin(handler(state))),
        });
        self
    }

    pub fn build(self) -> Result<RelayPlan<T>, RelayError> {
        validate_branches(&self.branches)?;
        Ok(RelayPlan {
            branches: self.branches,
        })
    }
}

#[derive(Clone)]
pub struct RelayPlan<T: OrchState> {
    branches: Vec<RelayBranch<T>>,
}

#[derive(Debug, Clone, Default)]
pub struct RelayExecutor;

impl RelayExecutor {
    pub async fn execute<T: OrchState>(
        &self,
        plan: &RelayPlan<T>,
        initial_state: T,
    ) -> Result<T, RelayError> {
        let mut futures = FuturesUnordered::new();
        for branch in &plan.branches {
            let name = branch.metadata.name.clone();
            let handler = branch.handler.clone();
            let state = initial_state.clone();
            futures.push(async move { (name, handler(state).await) });
        }

        let mut patches = Vec::new();
        while let Some((name, result)) = futures.next().await {
            patches.push((name, result?));
        }
        patches.sort_by(|left, right| left.0.cmp(&right.0));

        let mut state = initial_state;
        for (_, patch) in patches {
            state = T::merge(&state, patch);
        }
        Ok(state)
    }
}

fn validate_branches<T: OrchState>(branches: &[RelayBranch<T>]) -> Result<(), RelayError> {
    if branches.is_empty() {
        return Err(RelayError::EmptyPlan);
    }

    let mut names = std::collections::BTreeSet::new();
    for branch in branches {
        let name = branch.metadata.name.trim();
        if name.is_empty() {
            return Err(RelayError::BlankBranchName);
        }
        if !names.insert(name.to_owned()) {
            return Err(RelayError::DuplicateBranch(name.to_owned()));
        }
    }
    Ok(())
}
