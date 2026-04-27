use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

/// Python wrapper for an `or_conduit::ConduitProvider`-like object.
///
/// Holds an optional Python callable that performs message completion.
/// The callable is invoked by `complete_messages(messages)` and is
/// expected to return the completion result as a Python object (e.g. a
/// dict with `text`/`usage`/`finish_reason`).
#[pyclass(module = "orchustr._orchustr")]
#[derive(Default)]
pub struct PyConduitProvider {
    label: Option<String>,
    handler: Option<Py<PyAny>>,
}

impl Clone for PyConduitProvider {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Self {
            label: self.label.clone(),
            handler: self.handler.as_ref().map(|h| h.clone_ref(py)),
        })
    }
}

#[pymethods]
impl PyConduitProvider {
    #[new]
    #[pyo3(signature = (label=None, handler=None))]
    fn new(label: Option<String>, handler: Option<Py<PyAny>>) -> Self {
        Self { label, handler }
    }

    #[getter]
    fn label(&self) -> Option<String> {
        self.label.clone()
    }

    /// Replaces the underlying handler. Returns `True` if a handler was
    /// already registered.
    fn set_handler(&mut self, handler: Py<PyAny>) -> bool {
        self.handler.replace(handler).is_some()
    }

    fn has_handler(&self) -> bool {
        self.handler.is_some()
    }

    /// Invokes the registered handler with `messages`. Raises
    /// `RuntimeError` if no handler is configured.
    fn complete_messages(&self, py: Python<'_>, messages: Py<PyAny>) -> PyResult<Py<PyAny>> {
        match &self.handler {
            Some(handler) => handler.call1(py, (messages,)),
            None => Err(PyRuntimeError::new_err(
                "PyConduitProvider has no handler — pass `handler=` in the constructor or call `set_handler`",
            )),
        }
    }
}
