use pyo3::prelude::*;

/// Python wrapper that mirrors `or_relay::RelayBuilder<or_core::DynState>`.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Clone, Default)]
pub struct PyRelayBuilder {
    branches: Vec<String>,
}

/// Python wrapper that exposes a built relay plan shape.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Clone)]
pub struct PyRelayPlan {
    branches: Vec<String>,
}

#[pymethods]
impl PyRelayBuilder {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn add_branch(&mut self, name: &str, _handler: Py<PyAny>) {
        self.branches.push(name.to_owned());
    }

    fn build(&self) -> PyRelayPlan {
        PyRelayPlan {
            branches: self.branches.clone(),
        }
    }
}

#[pymethods]
impl PyRelayPlan {
    fn branch_names(&self) -> Vec<String> {
        self.branches.clone()
    }
}
