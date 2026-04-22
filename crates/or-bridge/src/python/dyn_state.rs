use crate::python::conversions::{py_to_value, value_to_py};
use or_core::DynState;
use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;

/// Python wrapper around `or_core::DynState`.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Clone, Default)]
pub struct PyDynState {
    pub(crate) inner: DynState,
}

#[pymethods]
impl PyDynState {
    #[new]
    #[pyo3(signature = (initial=None))]
    fn new(py: Python<'_>, initial: Option<Py<PyAny>>) -> PyResult<Self> {
        let mut inner = DynState::new();
        if let Some(initial) = initial {
            let value = py_to_value(py, &initial)?;
            let object = value
                .as_object()
                .ok_or_else(|| PyValueError::new_err("DynState requires a JSON object"))?;
            for (key, value) in object {
                inner.insert(key.clone(), value.clone());
            }
        }
        Ok(Self { inner })
    }

    fn __getitem__(&self, py: Python<'_>, key: &str) -> PyResult<Py<PyAny>> {
        let value = self
            .inner
            .get(key)
            .ok_or_else(|| PyKeyError::new_err(key.to_owned()))?;
        value_to_py(py, value)
    }

    fn __setitem__(&mut self, py: Python<'_>, key: &str, value: Py<PyAny>) -> PyResult<()> {
        self.inner.insert(key.to_owned(), py_to_value(py, &value)?);
        Ok(())
    }

    fn __contains__(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    fn get(&self, py: Python<'_>, key: &str) -> PyResult<Option<Py<PyAny>>> {
        self.inner
            .get(key)
            .map(|value| value_to_py(py, value))
            .transpose()
    }

    fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = serde_json::to_value(&self.inner)
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        value_to_py(py, &value)
    }

    fn keys(&self) -> Vec<String> {
        self.inner.keys().cloned().collect()
    }
}
