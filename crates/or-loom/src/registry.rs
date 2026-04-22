use crate::{ExecutionGraph, GraphBuilder, LoomError, NodeResult};
use or_core::{DynState, OrchState};
use or_schema::{EdgeSpec, GraphSpec};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type RegistryFuture = Pin<Box<dyn Future<Output = Result<NodeResult<DynState>, LoomError>> + Send>>;
type RegisteredHandler = Arc<dyn Fn(DynState) -> RegistryFuture + Send + Sync>;
type RegisteredCondition = Arc<dyn Fn(&DynState) -> Result<bool, LoomError> + Send + Sync>;

#[derive(Clone)]
struct CompiledConditionalEdge {
    to: String,
    condition: RegisteredCondition,
}

#[derive(Clone, Default)]
struct CompiledRouting {
    conditional_edges: Vec<CompiledConditionalEdge>,
    default_edges: Vec<String>,
}

/// A runtime registry that resolves `or-schema` handler names into executable `or-loom` nodes.
#[derive(Clone, Default)]
pub struct NodeRegistry {
    handlers: HashMap<String, RegisteredHandler>,
    conditions: HashMap<String, RegisteredCondition>,
}

impl NodeRegistry {
    /// Creates an empty `or-loom` node registry for `or-schema` graph compilation.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a named async node handler for `or-schema` graph compilation in `or-loom`.
    pub fn register<F, Fut>(&mut self, name: &str, handler: F) -> &mut Self
    where
        F: Fn(DynState) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<NodeResult<DynState>, LoomError>> + Send + 'static,
    {
        let handler: RegisteredHandler = Arc::new(move |state| Box::pin(handler(state)));
        self.handlers.insert(name.to_owned(), handler);
        self
    }

    /// Registers a named conditional edge predicate for `or-schema` graph compilation in `or-loom`.
    pub fn register_condition<F>(&mut self, name: &str, predicate: F) -> &mut Self
    where
        F: Fn(&DynState) -> Result<bool, LoomError> + Send + Sync + 'static,
    {
        self.conditions
            .insert(name.to_owned(), Arc::new(predicate) as RegisteredCondition);
        self
    }

    /// Compiles an `or-schema` `GraphSpec` into a live `or-loom` execution graph.
    pub fn compile(&self, spec: &GraphSpec) -> Result<ExecutionGraph<DynState>, LoomError> {
        let mut builder = GraphBuilder::new();
        let outgoing = collect_outgoing_edges(&spec.edges);

        for node in &spec.nodes {
            let handler = self
                .handlers
                .get(&node.handler)
                .cloned()
                .ok_or_else(|| LoomError::UnknownHandler(node.handler.clone()))?;
            let routing =
                compile_routing(self, outgoing.get(&node.id).map_or(&[][..], Vec::as_slice))?;
            let node_name = node.id.clone();

            builder = builder.add_node(&node.id, move |state: DynState| {
                let handler = Arc::clone(&handler);
                let node_name = node_name.clone();
                let routing = routing.clone();
                async move {
                    let original_state = state.clone();
                    match handler(state).await? {
                        NodeResult::Advance(patch) => {
                            if let Some(next) =
                                select_conditional_target(&node_name, &original_state, &patch, &routing)?
                            {
                                NodeResult::branch(patch, next)
                            } else {
                                NodeResult::advance(patch)
                            }
                        }
                        other => Ok(other),
                    }
                }
            });
        }

        for edge in &spec.edges {
            builder = builder.add_edge(&edge.from, &edge.to);
        }

        if spec.exits.is_empty() {
            return Err(LoomError::MissingExit);
        }

        builder
            .set_entry(&spec.entry)
            .set_exit_nodes(spec.exits.clone())
            .build()
    }
}

fn collect_outgoing_edges(edges: &[EdgeSpec]) -> HashMap<String, Vec<EdgeSpec>> {
    let mut outgoing = HashMap::new();
    for edge in edges {
        outgoing
            .entry(edge.from.clone())
            .or_insert_with(Vec::new)
            .push(edge.clone());
    }
    outgoing
}

fn compile_routing(
    registry: &NodeRegistry,
    edges: &[EdgeSpec],
) -> Result<CompiledRouting, LoomError> {
    let mut compiled = CompiledRouting::default();
    for edge in edges {
        if let Some(condition_name) = &edge.condition {
            let condition = registry
                .conditions
                .get(condition_name)
                .cloned()
                .ok_or_else(|| LoomError::UnknownCondition(condition_name.clone()))?;
            compiled.conditional_edges.push(CompiledConditionalEdge {
                to: edge.to.clone(),
                condition,
            });
        } else {
            compiled.default_edges.push(edge.to.clone());
        }
    }
    Ok(compiled)
}

fn select_conditional_target(
    node_name: &str,
    original_state: &DynState,
    patch: &DynState,
    routing: &CompiledRouting,
) -> Result<Option<String>, LoomError> {
    if routing.conditional_edges.is_empty() {
        return Ok(None);
    }

    let merged = DynState::merge(original_state, patch.clone());
    for edge in &routing.conditional_edges {
        if (edge.condition)(&merged)? {
            return Ok(Some(edge.to.clone()));
        }
    }

    if routing.default_edges.len() == 1 {
        Ok(routing.default_edges.first().cloned())
    } else if !routing.default_edges.is_empty() {
        Ok(None)
    } else {
        Err(LoomError::NoConditionalMatch(node_name.to_owned()))
    }
}
