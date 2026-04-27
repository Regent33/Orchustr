//! Shared utilities used by every per-crate facade module: runtime
//! management, blocking adapter, JSON field accessors, and the small
//! set of error constructors.

use crate::domain::errors::BridgeError;
use serde::Serialize;
use serde_json::{Map, Value};
use std::future::Future;
use std::sync::OnceLock;

pub(crate) fn from_field<T>(
    payload: &Value,
    key: &str,
    crate_name: &str,
    operation: &str,
) -> Result<T, BridgeError>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_value(payload.get(key).cloned().ok_or_else(|| {
        BridgeError::InvalidInput(format!(
            "missing `{key}` for `{crate_name}` / `{operation}`"
        ))
    })?)
    .map_err(|error| BridgeError::InvalidInput(error.to_string()))
}

pub(crate) fn required_str<'a>(
    payload: &'a Value,
    key: &str,
    crate_name: &str,
    operation: &str,
) -> Result<&'a str, BridgeError> {
    payload.get(key).and_then(Value::as_str).ok_or_else(|| {
        BridgeError::InvalidInput(format!(
            "missing string `{key}` for `{crate_name}` / `{operation}`"
        ))
    })
}

pub(crate) fn required_u64(
    payload: &Value,
    key: &str,
    crate_name: &str,
    operation: &str,
) -> Result<u64, BridgeError> {
    payload.get(key).and_then(Value::as_u64).ok_or_else(|| {
        BridgeError::InvalidInput(format!(
            "missing integer `{key}` for `{crate_name}` / `{operation}`"
        ))
    })
}

pub(crate) fn get_str<'a>(payload: &'a Map<String, Value>, key: &str) -> Option<&'a str> {
    payload.get(key).and_then(Value::as_str)
}

pub(crate) fn json_value<T: Serialize>(value: T) -> Result<Value, BridgeError> {
    serde_json::to_value(value).map_err(|error| BridgeError::InvalidJson(error.to_string()))
}

pub(crate) fn invocation(
    crate_name: &str,
    operation: &str,
    error: impl std::fmt::Display,
) -> BridgeError {
    BridgeError::Invocation {
        crate_name: crate_name.to_owned(),
        operation: operation.to_owned(),
        reason: error.to_string(),
    }
}

pub(crate) fn unsupported(crate_name: &str, operation: &str) -> BridgeError {
    BridgeError::UnsupportedOperation {
        crate_name: crate_name.to_owned(),
        operation: operation.to_owned(),
    }
}

pub(crate) fn unsupported_provider(crate_name: &str, provider: &str) -> BridgeError {
    BridgeError::InvalidInput(format!(
        "unsupported provider `{provider}` for `{crate_name}`"
    ))
}

/// Returns the bridge's process-wide multi-thread Tokio runtime.
///
/// This runtime is built lazily on first use and lives for the rest of
/// the process lifetime — `OnceLock` deliberately never drops it. We do
/// not orderly-shut down the runtime at process exit because:
///
/// * From a `pyo3` extension module there is no `Py_AtExit` hook that
///   can be guaranteed to run before the host interpreter tears down
///   the GIL.
/// * From a `napi-rs`-loaded `.node` add-on the runtime would be
///   reaped together with the loading process anyway.
/// * From a Dart `DynamicLibrary.open` load the runtime is freed by
///   the OS when Dart unloads the library.
///
/// In all three cases letting the OS reclaim the runtime threads at
/// exit is correct. Long-running embedders that need explicit shutdown
/// should call into the underlying tools' `*_orchestrator::shutdown()`
/// hooks rather than expecting `or-bridge` to drain its runtime.
fn runtime() -> &'static tokio::runtime::Runtime {
    static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("or-bridge runtime")
    })
}

pub(crate) fn block_on<T, E>(
    crate_name: &str,
    operation: &str,
    future: impl Future<Output = Result<T, E>>,
) -> Result<T, BridgeError>
where
    E: std::fmt::Display,
{
    // Three runtime scenarios:
    //
    // 1. No runtime current → spin up (or reuse) the bridge's own
    //    multi-thread runtime and block on it.
    // 2. A multi-thread runtime is current → use `block_in_place` so we
    //    don't deadlock the executor.
    // 3. A current-thread runtime is current → `block_in_place` would
    //    panic, so refuse with a clear `BridgeError` instead.
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => match handle.runtime_flavor() {
            tokio::runtime::RuntimeFlavor::MultiThread => {
                tokio::task::block_in_place(|| handle.block_on(future))
                    .map_err(|error| invocation(crate_name, operation, error))
            }
            // Includes `CurrentThread` and any future single-threaded flavors.
            _ => Err(BridgeError::Invocation {
                crate_name: crate_name.to_owned(),
                operation: operation.to_owned(),
                reason: "or-bridge cannot block on a current-thread tokio runtime; \
                         call from a multi-thread runtime or a non-async context"
                    .to_owned(),
            }),
        },
        Err(_) => runtime()
            .block_on(future)
            .map_err(|error| invocation(crate_name, operation, error)),
    }
}
