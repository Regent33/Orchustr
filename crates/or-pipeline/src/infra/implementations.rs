use crate::domain::entities::PipelineNodeMetadata;
use crate::domain::errors::PipelineError;
use or_core::OrchState;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type NodeFuture<T> = Pin<Box<dyn Future<Output = Result<T, PipelineError>> + Send + 'static>>;
type NodeHandler<T> = Arc<dyn Fn(T) -> NodeFuture<T> + Send + Sync + 'static>;

#[derive(Clone)]
struct PipelineNode<T: OrchState> {
    metadata: PipelineNodeMetadata,
    handler: NodeHandler<T>,
}

/// Not serializable because it stores executable node closures.
#[derive(Clone)]
pub struct PipelineBuilder<T: OrchState> {
    nodes: Vec<PipelineNode<T>>,
}

impl<T: OrchState> Default for PipelineBuilder<T> {
    fn default() -> Self {
        Self { nodes: Vec::new() }
    }
}

impl<T: OrchState> PipelineBuilder<T> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn add_node<F, Fut>(mut self, name: &str, handler: F) -> Self
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, PipelineError>> + Send + 'static,
    {
        let node = PipelineNode {
            metadata: PipelineNodeMetadata {
                name: name.to_owned(),
            },
            handler: Arc::new(move |state| Box::pin(handler(state))),
        };
        self.nodes.push(node);
        self
    }

    pub fn build(self) -> Result<Pipeline<T>, PipelineError> {
        validate_nodes(&self.nodes)?;
        Ok(Pipeline { nodes: self.nodes })
    }
}

/// Not serializable because it stores executable node closures.
#[derive(Clone)]
pub struct Pipeline<T: OrchState> {
    nodes: Vec<PipelineNode<T>>,
}

impl<T: OrchState> Pipeline<T> {
    pub async fn execute(&self, initial_state: T) -> Result<T, PipelineError> {
        let mut state = initial_state;
        for node in &self.nodes {
            let patch = (node.handler)(state.clone()).await?;
            state = T::merge(&state, patch);
        }
        Ok(state)
    }

    #[must_use]
    pub fn node_names(&self) -> Vec<String> {
        self.nodes
            .iter()
            .map(|node| node.metadata.name.clone())
            .collect()
    }
}

fn validate_nodes<T: OrchState>(nodes: &[PipelineNode<T>]) -> Result<(), PipelineError> {
    if nodes.is_empty() {
        return Err(PipelineError::EmptyPipeline);
    }

    let mut seen = std::collections::BTreeSet::new();
    for node in nodes {
        let name = node.metadata.name.trim();
        if name.is_empty() {
            return Err(PipelineError::BlankNodeName);
        }
        if !seen.insert(name.to_owned()) {
            return Err(PipelineError::DuplicateNode(name.to_owned()));
        }
    }
    Ok(())
}
