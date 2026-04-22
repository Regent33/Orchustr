use super::dyn_state::PyDynState;
use or_loom::NodeResult;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn map_loom_error(error: or_loom::LoomError) -> PyErr {
    PyValueError::new_err(error.to_string())
}

/// Python wrapper around `or_loom::NodeResult<or_core::DynState>`.
#[pyclass(module = "orchustr._orchustr")]
#[derive(Clone)]
pub struct PyNodeResult {
    pub(crate) inner: NodeResult<or_core::DynState>,
    pub(crate) force_exit: bool,
}

#[pymethods]
impl PyNodeResult {
    #[staticmethod]
    fn advance(state: PyDynState) -> PyResult<Self> {
        Ok(Self {
            inner: NodeResult::advance(state.inner).map_err(map_loom_error)?,
            force_exit: false,
        })
    }

    #[staticmethod]
    fn exit(state: PyDynState) -> PyResult<Self> {
        Ok(Self {
            inner: NodeResult::advance(state.inner).map_err(map_loom_error)?,
            force_exit: true,
        })
    }

    #[staticmethod]
    fn branch(state: PyDynState, next: &str) -> PyResult<Self> {
        Ok(Self {
            inner: NodeResult::branch(state.inner, next).map_err(map_loom_error)?,
            force_exit: false,
        })
    }

    #[staticmethod]
    fn pause(checkpoint_id: &str, state: PyDynState) -> PyResult<Self> {
        Ok(Self {
            inner: NodeResult::pause(checkpoint_id, state.inner).map_err(map_loom_error)?,
            force_exit: false,
        })
    }

    #[getter]
    fn kind(&self) -> &'static str {
        if self.force_exit {
            return "exit";
        }
        match self.inner {
            NodeResult::Advance(_) => "advance",
            NodeResult::Branch { .. } => "branch",
            NodeResult::Pause { .. } => "pause",
        }
    }

    #[getter]
    fn next(&self) -> Option<String> {
        match &self.inner {
            NodeResult::Branch { next, .. } => Some(next.clone()),
            _ => None,
        }
    }
}
