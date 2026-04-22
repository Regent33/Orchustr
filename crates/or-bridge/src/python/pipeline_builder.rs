use pyo3::prelude::*;

/// Python wrapper that mirrors `or_pipeline::PipelineBuilder<or_core::DynState>` structure.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Clone, Default)]
pub struct PyPipelineBuilder {
    nodes: Vec<String>,
}

/// Python wrapper that exposes a built pipeline shape.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Clone)]
pub struct PyPipeline {
    nodes: Vec<String>,
}

#[pymethods]
impl PyPipelineBuilder {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn add_node(&mut self, name: &str, _handler: Py<PyAny>) {
        self.nodes.push(name.to_owned());
    }

    fn build(&self) -> PyPipeline {
        PyPipeline {
            nodes: self.nodes.clone(),
        }
    }
}

#[pymethods]
impl PyPipeline {
    fn node_names(&self) -> Vec<String> {
        self.nodes.clone()
    }
}
