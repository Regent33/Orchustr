use pyo3::prelude::*;
use std::collections::HashMap;

/// Python wrapper that mirrors `or_loom::GraphBuilder<or_core::DynState>` structure.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Clone, Default)]
pub struct PyGraphBuilder {
    nodes: Vec<String>,
    edges: HashMap<String, Vec<String>>,
    entry: Option<String>,
    exit: Option<String>,
}

/// Python wrapper that exposes a built graph shape.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Clone)]
pub struct PyExecutionGraph {
    nodes: Vec<String>,
    edges: HashMap<String, Vec<String>>,
    entry: String,
    exit: String,
}

#[pymethods]
impl PyGraphBuilder {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn add_node(&mut self, name: &str, _handler: Py<PyAny>) {
        if !self.nodes.iter().any(|existing| existing == name) {
            self.nodes.push(name.to_owned());
        }
    }

    fn add_edge(&mut self, source: &str, target: &str) {
        self.edges
            .entry(source.to_owned())
            .or_default()
            .push(target.to_owned());
    }

    fn set_entry(&mut self, name: &str) {
        self.entry = Some(name.to_owned());
    }

    fn set_exit(&mut self, name: &str) {
        self.exit = Some(name.to_owned());
    }

    fn build(&self) -> PyExecutionGraph {
        PyExecutionGraph {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
            entry: self.entry.clone().unwrap_or_default(),
            exit: self.exit.clone().unwrap_or_default(),
        }
    }
}

#[pymethods]
impl PyExecutionGraph {
    fn entry(&self) -> String {
        self.entry.clone()
    }

    fn exit(&self) -> String {
        self.exit.clone()
    }

    fn node_names(&self) -> Vec<String> {
        self.nodes.clone()
    }

    fn edge_targets(&self, node: &str) -> Vec<String> {
        self.edges.get(node).cloned().unwrap_or_default()
    }

    fn get_handler(&self, node: &str) -> Option<String> {
        self.nodes
            .iter()
            .any(|existing| existing == node)
            .then(|| "python-callable".to_owned())
    }
}
