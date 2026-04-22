use pyo3::prelude::*;

/// Python wrapper placeholder for an `or_conduit::ConduitProvider` trait object.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Clone, Default)]
pub struct PyConduitProvider {
    label: Option<String>,
}

#[pymethods]
impl PyConduitProvider {
    #[new]
    #[pyo3(signature = (label=None))]
    fn new(label: Option<String>) -> Self {
        Self { label }
    }

    #[getter]
    fn label(&self) -> Option<String> {
        self.label.clone()
    }
}
