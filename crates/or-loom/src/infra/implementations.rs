use crate::domain::entities::NodeResult;
use crate::domain::errors::LoomError;
use crate::infra::adapters::validate_graph_shape;
use crate::inspection::{GraphEdgeInspection, GraphInspection};
use or_core::OrchState;
use std::collections::{BTreeSet, HashMap};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type NodeFuture<T> = Pin<Box<dyn Future<Output = Result<NodeResult<T>, LoomError>> + 'static>>;
type NodeHandler<T> = Arc<dyn Fn(T) -> NodeFuture<T> + Send + Sync + 'static>;

#[derive(Clone)]
struct GraphNode<T: OrchState> {
    handler: Option<NodeHandler<T>>,
}

#[derive(Clone)]
pub struct GraphBuilder<T: OrchState> {
    nodes: HashMap<String, GraphNode<T>>,
    edges: HashMap<String, Vec<String>>,
    entry: Option<String>,
    exit: Option<String>,
    exits: BTreeSet<String>,
}

impl<T: OrchState> Default for GraphBuilder<T> {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            entry: None,
            exit: None,
            exits: BTreeSet::new(),
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
        self = self.add_placeholder_node(name);
        self.bind_node(name, handler)
    }

    /// Adds a named node without a handler so it can be bound later.
    ///
    /// This is useful for crates such as `or-sentinel`, where a topology may
    /// define graph structure first and attach runtime-specific handlers later.
    #[must_use]
    pub fn add_placeholder_node(mut self, name: &str) -> Self {
        self.nodes
            .entry(name.to_owned())
            .or_insert(GraphNode { handler: None });
        self
    }

    /// Binds or replaces the handler for an existing or placeholder node.
    ///
    /// If the node does not exist yet, this method creates it first.
    #[must_use]
    pub fn bind_node<F, Fut>(mut self, name: &str, handler: F) -> Self
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<NodeResult<T>, LoomError>> + 'static,
    {
        let handler: NodeHandler<T> = Arc::new(move |state| Box::pin(handler(state)));
        self.nodes
            .entry(name.to_owned())
            .and_modify(|node| {
                node.handler = Some(handler.clone());
            })
            .or_insert(GraphNode {
                handler: Some(handler),
            });
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
        self.exits.clear();
        self.exits.insert(name.to_owned());
        self
    }

    /// Sets all recognized exit nodes for the graph while preserving the first exit as primary.
    #[must_use]
    pub fn set_exit_nodes<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.exits.clear();
        self.exit = None;
        for name in names {
            let name = name.into();
            if self.exit.is_none() {
                self.exit = Some(name.clone());
            }
            self.exits.insert(name);
        }
        self
    }

    pub fn build(self) -> Result<ExecutionGraph<T>, LoomError> {
        validate_graph_shape(
            self.nodes.keys(),
            &self.edges,
            self.entry.as_deref(),
            self.exit.as_deref(),
        )?;
        for exit in &self.exits {
            if !self.nodes.contains_key(exit) {
                return Err(LoomError::UnknownNode(exit.clone()));
            }
        }
        let mut nodes = HashMap::new();
        for (name, node) in self.nodes {
            match node.handler {
                Some(handler) => {
                    nodes.insert(name, handler);
                }
                None => return Err(LoomError::UnboundNode(name)),
            }
        }
        Ok(ExecutionGraph {
            nodes,
            edges: self.edges,
            entry: self.entry.unwrap_or_default(),
            exit: self.exit.unwrap_or_default(),
            exits: self.exits,
            max_steps: 1024,
        })
    }
}

#[derive(Clone)]
pub struct ExecutionGraph<T: OrchState> {
    nodes: HashMap<String, NodeHandler<T>>,
    edges: HashMap<String, Vec<String>>,
    entry: String,
    exit: String,
    exits: BTreeSet<String>,
    max_steps: usize,
}

impl<T: OrchState> ExecutionGraph<T> {
    /// Returns a deterministic structural snapshot of this `or-loom` graph.
    #[must_use]
    pub fn inspect(&self) -> GraphInspection {
        let mut nodes = self.nodes.keys().cloned().collect::<Vec<_>>();
        nodes.sort();

        let mut edges = self
            .edges
            .iter()
            .flat_map(|(from, targets)| {
                targets.iter().map(|to| GraphEdgeInspection {
                    from: from.clone(),
                    to: to.clone(),
                })
            })
            .collect::<Vec<_>>();
        edges.sort_by(|left, right| left.from.cmp(&right.from).then(left.to.cmp(&right.to)));

        GraphInspection {
            entry: self.entry.clone(),
            exit: self.exit.clone(),
            nodes,
            edges,
        }
    }

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
            match node(state.clone()).await? {
                NodeResult::Advance(patch) => {
                    state = T::merge(&state, patch);
                    if self.exits.contains(&current) {
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
                    // Serialize the merged state so the error variant can
                    // carry it across the boundary regardless of `T`. If
                    // serialization fails we still surface the pause but
                    // with a null state — better than silently dropping.
                    let snapshot = serde_json::to_value(&state).unwrap_or(serde_json::Value::Null);
                    return Err(LoomError::Paused {
                        checkpoint_id,
                        state: snapshot,
                    });
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
