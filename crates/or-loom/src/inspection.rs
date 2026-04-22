use serde::{Deserialize, Serialize};

/// A serializable description of a graph edge in `or-loom`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphEdgeInspection {
    /// Source node name for this edge.
    pub from: String,
    /// Destination node name for this edge.
    pub to: String,
}

/// A serializable structural snapshot of an `or-loom` execution graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphInspection {
    /// Entry node name for the graph.
    pub entry: String,
    /// Exit node name for the graph.
    pub exit: String,
    /// All node names in the graph, sorted for deterministic comparison.
    pub nodes: Vec<String>,
    /// All directed edges in the graph, sorted for deterministic comparison.
    pub edges: Vec<GraphEdgeInspection>,
}
