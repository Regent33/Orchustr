use pyo3::prelude::*;
use std::collections::HashMap;

/// Python wrapper that mirrors `or_loom::GraphBuilder<or_core::DynState>`
/// structure and **stores** registered Python handlers so callers can
/// retrieve them via [`PyExecutionGraph::get_handler`].
///
/// This type is structural metadata only — it does not execute the
/// graph. For Python-driven execution use the pure-Python
/// `orchustr.GraphBuilder` in `bindings/python/orchustr/graph.py`,
/// which calls these stored handlers in topological order.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Default)]
pub struct PyGraphBuilder {
    nodes: Vec<String>,
    handlers: HashMap<String, Py<PyAny>>,
    edges: HashMap<String, Vec<String>>,
    entry: Option<String>,
    exit: Option<String>,
}

impl Clone for PyGraphBuilder {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Self {
            nodes: self.nodes.clone(),
            handlers: self
                .handlers
                .iter()
                .map(|(name, handler)| (name.clone(), handler.clone_ref(py)))
                .collect(),
            edges: self.edges.clone(),
            entry: self.entry.clone(),
            exit: self.exit.clone(),
        })
    }
}

/// Python wrapper that exposes a built graph shape together with the
/// originally registered handlers.
#[pyclass(module = "orchustr._orchustr")]
pub struct PyExecutionGraph {
    nodes: Vec<String>,
    handlers: HashMap<String, Py<PyAny>>,
    edges: HashMap<String, Vec<String>>,
    entry: String,
    exit: String,
}

impl Clone for PyExecutionGraph {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Self {
            nodes: self.nodes.clone(),
            handlers: self
                .handlers
                .iter()
                .map(|(name, handler)| (name.clone(), handler.clone_ref(py)))
                .collect(),
            edges: self.edges.clone(),
            entry: self.entry.clone(),
            exit: self.exit.clone(),
        })
    }
}

#[pymethods]
impl PyGraphBuilder {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    /// Registers a Python callable for the named node. Subsequent calls
    /// for the same name overwrite the previously stored handler.
    fn add_node(&mut self, name: &str, handler: Py<PyAny>) {
        if !self.nodes.iter().any(|existing| existing == name) {
            self.nodes.push(name.to_owned());
        }
        self.handlers.insert(name.to_owned(), handler);
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
        Python::with_gil(|py| PyExecutionGraph {
            nodes: self.nodes.clone(),
            handlers: self
                .handlers
                .iter()
                .map(|(name, handler)| (name.clone(), handler.clone_ref(py)))
                .collect(),
            edges: self.edges.clone(),
            entry: self.entry.clone().unwrap_or_default(),
            exit: self.exit.clone().unwrap_or_default(),
        })
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

    /// Returns the Python callable registered for `node`, or `None`.
    /// The returned object is the same callable handed to
    /// `PyGraphBuilder::add_node`, ready to be invoked by the caller.
    fn get_handler(&self, py: Python<'_>, node: &str) -> Option<Py<PyAny>> {
        self.handlers.get(node).map(|handler| handler.clone_ref(py))
    }
}
