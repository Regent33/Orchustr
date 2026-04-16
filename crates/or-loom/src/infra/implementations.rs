use crate::domain::entities::NodeResult;
use crate::domain::errors::LoomError;
use crate::infra::adapters::validate_graph_shape;
use or_core::OrchState;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type NodeFuture<T> = Pin<Box<dyn Future<Output = Result<NodeResult<T>, LoomError>> + 'static>>;
type NodeHandler<T> = Arc<dyn Fn(T) -> NodeFuture<T> + Send + Sync + 'static>;

#[derive(Clone)]
struct GraphNode<T: OrchState> {
    handler: NodeHandler<T>,
}

#[derive(Clone)]
pub struct GraphBuilder<T: OrchState> {
    nodes: HashMap<String, GraphNode<T>>,
    edges: HashMap<String, Vec<String>>,
    entry: Option<String>,
    exit: Option<String>,
}

impl<T: OrchState> Default for GraphBuilder<T> {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            entry: None,
            exit: None,
        }
    }
}

impl<T: OrchState> GraphBuilder<T> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn add_node<F, Fut>(mut self, name: &str, handler: F) -> Self
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<NodeResult<T>, LoomError>> + 'static,
    {
        self.nodes.insert(
            name.to_owned(),
            GraphNode {
                handler: Arc::new(move |state| Box::pin(handler(state))),
            },
        );
        self
    }

    #[must_use]
    pub fn add_edge(mut self, from: &str, to: &str) -> Self {
        self.edges
            .entry(from.to_owned())
            .or_default()
            .push(to.to_owned());
        self
    }

    #[must_use]
    pub fn set_entry(mut self, name: &str) -> Self {
        self.entry = Some(name.to_owned());
        self
    }

    #[must_use]
    pub fn set_exit(mut self, name: &str) -> Self {
        self.exit = Some(name.to_owned());
        self
    }

    pub fn build(self) -> Result<ExecutionGraph<T>, LoomError> {
        validate_graph_shape(
            self.nodes.keys(),
            &self.edges,
            self.entry.as_deref(),
            self.exit.as_deref(),
        )?;
        Ok(ExecutionGraph {
            nodes: self.nodes,
            edges: self.edges,
            entry: self.entry.unwrap_or_default(),
            exit: self.exit.unwrap_or_default(),
            max_steps: 1024,
        })
    }
}

#[derive(Clone)]
pub struct ExecutionGraph<T: OrchState> {
    nodes: HashMap<String, GraphNode<T>>,
    edges: HashMap<String, Vec<String>>,
    entry: String,
    exit: String,
    max_steps: usize,
}

impl<T: OrchState> ExecutionGraph<T> {
    pub async fn execute(&self, initial_state: T) -> Result<T, LoomError> {
        self.execute_from(&self.entry, initial_state).await
    }

    pub async fn execute_from(&self, start_at: &str, initial_state: T) -> Result<T, LoomError> {
        let mut current = start_at.to_owned();
        let mut state = initial_state;
        for _ in 0..self.max_steps {
            let node = self
                .nodes
                .get(&current)
                .ok_or_else(|| LoomError::UnknownNode(current.clone()))?;
            match (node.handler)(state.clone()).await? {
                NodeResult::Advance(patch) => {
                    state = T::merge(&state, patch);
                    if current == self.exit {
                        return Ok(state);
                    }
                    current = self.default_next(&current)?;
                }
                NodeResult::Branch { state: patch, next } => {
                    state = T::merge(&state, patch);
                    if !self.edge_exists(&current, &next) {
                        return Err(LoomError::InvalidBranchTarget {
                            from: current,
                            to: next,
                        });
                    }
                    current = next;
                }
                NodeResult::Pause {
                    checkpoint_id,
                    state: patch,
                } => {
                    state = T::merge(&state, patch);
                    let _ = state;
                    return Err(LoomError::Paused { checkpoint_id });
                }
            }
        }
        Err(LoomError::StepLimitExceeded {
            max_steps: self.max_steps,
        })
    }

    fn default_next(&self, current: &str) -> Result<String, LoomError> {
        let edges = self
            .edges
            .get(current)
            .ok_or_else(|| LoomError::NoEdgeFromNode(current.to_owned()))?;
        if edges.len() == 1 {
            Ok(edges[0].clone())
        } else {
            Err(LoomError::AmbiguousNextNode(current.to_owned()))
        }
    }

    fn edge_exists(&self, from: &str, to: &str) -> bool {
        self.edges
            .get(from)
            .is_some_and(|targets| targets.iter().any(|target| target == to))
    }
}
