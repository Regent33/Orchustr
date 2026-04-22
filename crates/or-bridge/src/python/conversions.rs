use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde_json::Value;

pub(super) fn py_to_value(py: Python<'_>, value: &Py<PyAny>) -> PyResult<Value> {
    let json = py.import("json")?;
    let dumped = json
        .call_method1("dumps", (value.bind(py),))?
        .extract::<String>()?;
    serde_json::from_str(&dumped).map_err(|error| PyValueError::new_err(error.to_string()))
}

pub(super) fn value_to_py(py: Python<'_>, value: &Value) -> PyResult<Py<PyAny>> {
    let json = py.import("json")?;
    let dumped =
        serde_json::to_string(value).map_err(|error| PyValueError::new_err(error.to_string()))?;
    Ok(json.call_method1("loads", (dumped,))?.unbind())
}
