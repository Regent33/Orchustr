use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use std::collections::BTreeMap;

/// Python wrapper that mirrors an `or_forge::ForgeRegistry`-style tool table.
///
/// Unlike the previous version, registered handlers are kept and can be
/// invoked through `invoke(name, args)`. This restores the documented
/// behaviour of the registry — a Python callable supplied via `register`
/// is the function that runs when the tool is invoked.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Default)]
pub struct PyForgeRegistry {
    handlers: BTreeMap<String, Py<PyAny>>,
}

impl Clone for PyForgeRegistry {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Self {
            handlers: self
                .handlers
                .iter()
                .map(|(name, handler)| (name.clone(), handler.clone_ref(py)))
                .collect(),
        })
    }
}

#[pymethods]
impl PyForgeRegistry {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    /// Registers a Python callable under `name`. The callable is stored and
    /// invoked verbatim by `invoke(name, args)`.
    fn register(&mut self, name: &str, handler: Py<PyAny>) {
        self.handlers.insert(name.to_owned(), handler);
    }

    /// Removes a previously registered handler. Returns `True` if a
    /// handler was removed.
    fn unregister(&mut self, name: &str) -> bool {
        self.handlers.remove(name).is_some()
    }

    fn __contains__(&self, name: &str) -> bool {
        self.handlers.contains_key(name)
    }

    fn len(&self) -> usize {
        self.handlers.len()
    }

    fn tool_names(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }

    /// Calls the registered Python handler with `args` and returns its
    /// result. Raises `KeyError` if no handler is registered for `name`.
    #[pyo3(signature = (name, args=None))]
    fn invoke(&self, py: Python<'_>, name: &str, args: Option<Py<PyAny>>) -> PyResult<Py<PyAny>> {
        let handler = self
            .handlers
            .get(name)
            .ok_or_else(|| PyKeyError::new_err(format!("unknown tool: {name}")))?;
        match args {
            Some(value) => handler.call1(py, (value,)),
            None => handler.call0(py),
        }
    }
}
