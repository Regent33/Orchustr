#[cfg(not(any(feature = "dart", feature = "node", feature = "python")))]
compile_error!(
    "or-bridge requires at least one FFI feature: `dart`, `node`, or `python`. \
     Enable one in your Cargo.toml, e.g.: or-bridge = { features = [\"dart\"] }"
);

pub mod application;
pub mod domain;
pub mod infra;

pub use application::orchestrators::{
    invoke_crate_json, normalize_state_json, render_prompt_json, workspace_catalog_json,
};
#[cfg(feature = "dart")]
pub use dart::{
    orchustr_bridge_free_string, orchustr_bridge_version, orchustr_invoke_crate_json,
    orchustr_normalize_state_json, orchustr_render_prompt_json,
    orchustr_workspace_catalog_json,
};
pub use domain::errors::BridgeError;

#[cfg(feature = "dart")]
mod dart;
#[cfg(feature = "node")]
mod node;
#[cfg(feature = "python")]
mod python;
