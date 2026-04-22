use crate::python::conversions::{py_to_value, value_to_py};
use or_beacon::{PromptBuilder, PromptTemplate};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Python wrapper around `or_beacon::PromptBuilder`.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Clone, Default)]
pub struct PyPromptBuilder {
    inner: PromptBuilder,
}

/// Python wrapper around `or_beacon::PromptTemplate`.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Clone)]
pub struct PyPromptTemplate {
    inner: PromptTemplate,
}

#[pymethods]
impl PyPromptBuilder {
    #[new]
    fn new() -> Self {
        Self {
            inner: PromptBuilder::new(),
        }
    }

    fn template(&mut self, template: &str) {
        self.inner = self.inner.clone().template(template);
    }

    fn build(&self) -> PyResult<PyPromptTemplate> {
        self.inner
            .clone()
            .build()
            .map(|inner| PyPromptTemplate { inner })
            .map_err(|error| PyValueError::new_err(error.to_string()))
    }
}

#[pymethods]
impl PyPromptTemplate {
    fn render(&self, py: Python<'_>, context: Py<PyAny>) -> PyResult<Py<PyAny>> {
        let value = py_to_value(py, &context)?;
        let rendered = self
            .inner
            .render(&value)
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        value_to_py(py, &serde_json::json!(rendered))
    }
}
