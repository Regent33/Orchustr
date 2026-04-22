//! Serializable graph definitions for Orchustr runtimes.

mod loader;

use serde::{Deserialize, Serialize};

pub use loader::SchemaError;

/// A serializable Orchustr graph definition stored outside executable code.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphSpec {
    /// Unique name for this graph definition.
    pub name: String,
    /// Semver string for schema versioning.
    pub version: String,
    /// Nodes declared in this graph.
    pub nodes: Vec<NodeSpec>,
    /// Edges connecting declared nodes.
    pub edges: Vec<EdgeSpec>,
    /// Entry node identifier for the graph.
    pub entry: String,
    /// Exit node identifiers for the graph.
    pub exits: Vec<String>,
}

/// A serializable node descriptor inside an [`GraphSpec`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeSpec {
    /// Must match a key registered in the `or-loom` `NodeRegistry`.
    pub id: String,
    /// Registered handler name used to resolve this node at runtime.
    pub handler: String,
    /// Arbitrary metadata carried with the node definition.
    pub metadata: serde_json::Value,
}

/// A serializable edge descriptor inside an [`GraphSpec`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EdgeSpec {
    /// Source node identifier.
    pub from: String,
    /// Destination node identifier.
    pub to: String,
    /// Optional predicate function name used for conditional routing.
    pub condition: Option<String>,
}
