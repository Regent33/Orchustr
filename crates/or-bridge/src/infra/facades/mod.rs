//! Per-crate bridge facades.
//!
//! Each submodule owns the JSON-in / JSON-out adapter for one Rust
//! crate (or one tool surface). To add a new tool surface:
//!
//! 1. Create `facades/<crate>.rs` with a `pub(crate) fn invoke(operation,
//!    payload) -> Result<Value, BridgeError>` and any private build
//!    helpers.
//! 2. Add a `mod` line below.
//! 3. Add a dispatch arm in [`invoke`] mapping the catalog crate name
//!    to the new module.
//! 4. Add a [`catalog::CrateBinding`] entry in `catalog.rs`.
//!
//! Until the audit refactor, every step happened in one ~1100-line
//! file; now each step touches at most three short files.

mod catalog;
mod comms;
mod exec;
mod file;
mod helpers;
mod loaders;
mod native;
mod productivity;
mod search;
mod vector;
mod web;

use crate::domain::errors::BridgeError;
use serde_json::Value;

pub(crate) use catalog::workspace_catalog;

/// Top-level dispatch for `invoke_crate_json`. Routes to the per-crate
/// facade module owning that surface.
pub(crate) fn invoke(
    crate_name: &str,
    operation: &str,
    payload: Value,
) -> Result<Value, BridgeError> {
    match crate_name {
        "or-core" => native::invoke_core(operation, payload),
        "or-beacon" => native::invoke_beacon(operation, payload),
        "or-bridge" => native::invoke_bridge(operation, payload),
        "or-conduit" => native::invoke_conduit(operation, payload),
        "or-prism" => native::invoke_prism(operation, payload),
        "or-sieve" => native::invoke_sieve(operation, payload),
        "or-tools-search" => search::invoke(operation, payload),
        "or-tools-web" => web::invoke(operation, payload),
        "or-tools-vector" => vector::invoke(operation, payload),
        "or-tools-loaders" => loaders::invoke(operation, payload),
        "or-tools-exec" => exec::invoke(operation, payload),
        "or-tools-file" => file::invoke(operation, payload),
        "or-tools-comms" => comms::invoke(operation, payload),
        "or-tools-productivity" => productivity::invoke(operation, payload),
        other => Err(BridgeError::UnsupportedCrate(other.to_owned())),
    }
}
