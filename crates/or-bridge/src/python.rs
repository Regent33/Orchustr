use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pyfunction]
fn render_prompt_json(template: &str, context_json: &str) -> PyResult<String> {
    crate::render_prompt_json(template, context_json)
        .map_err(|error| PyValueError::new_err(error.to_string()))
}

#[pyfunction]
fn normalize_state_json(raw_state: &str) -> PyResult<String> {
    crate::normalize_state_json(raw_state).map_err(|error| PyValueError::new_err(error.to_string()))
}

#[pyfunction]
fn workspace_catalog_json() -> PyResult<String> {
    crate::workspace_catalog_json().map_err(|error| PyValueError::new_err(error.to_string()))
}

#[pyfunction]
fn invoke_crate_json(crate_name: &str, operation: &str, payload_json: &str) -> PyResult<String> {
    crate::invoke_crate_json(crate_name, operation, payload_json)
        .map_err(|error| PyValueError::new_err(error.to_string()))
}

#[pymodule]
fn _orchustr(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(version, module)?)?;
    module.add_function(wrap_pyfunction!(render_prompt_json, module)?)?;
    module.add_function(wrap_pyfunction!(normalize_state_json, module)?)?;
    module.add_function(wrap_pyfunction!(workspace_catalog_json, module)?)?;
    module.add_function(wrap_pyfunction!(invoke_crate_json, module)?)?;
    Ok(())
}
