mod colony_builder;
mod conduit_provider;
mod conversions;
mod dyn_state;
mod forge_registry;
mod graph_builder;
mod node_result;
mod pipeline_builder;
mod prompt_builder;
mod relay_builder;

use colony_builder::PyColonyBuilder;
use conduit_provider::PyConduitProvider;
use dyn_state::PyDynState;
use forge_registry::PyForgeRegistry;
use graph_builder::{PyExecutionGraph, PyGraphBuilder};
use node_result::PyNodeResult;
use pipeline_builder::{PyPipeline, PyPipelineBuilder};
use prompt_builder::{PyPromptBuilder, PyPromptTemplate};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use relay_builder::{PyRelayBuilder, PyRelayPlan};

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
    module.add_class::<PyDynState>()?;
    module.add_class::<PyNodeResult>()?;
    module.add_class::<PyPromptBuilder>()?;
    module.add_class::<PyPromptTemplate>()?;
    module.add_class::<PyGraphBuilder>()?;
    module.add_class::<PyExecutionGraph>()?;
    module.add_class::<PyPipelineBuilder>()?;
    module.add_class::<PyPipeline>()?;
    module.add_class::<PyConduitProvider>()?;
    module.add_class::<PyForgeRegistry>()?;
    module.add_class::<PyColonyBuilder>()?;
    module.add_class::<PyRelayBuilder>()?;
    module.add_class::<PyRelayPlan>()?;
    Ok(())
}
