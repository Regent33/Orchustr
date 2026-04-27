use crate::{
    BridgeError, invoke_crate_json, normalize_state_json, render_prompt_json,
    workspace_catalog_json,
};
use std::ffi::{CStr, CString, c_char};
use std::ptr;

fn write_error(error_out: *mut *mut c_char, error: BridgeError) {
    if error_out.is_null() {
        return;
    }
    let message = match CString::new(error.to_string()) {
        Ok(value) => value.into_raw(),
        Err(_) => ptr::null_mut(),
    };
    // SAFETY: `error_out` is an optional out-parameter supplied by the caller.
    unsafe {
        *error_out = message;
    }
}

fn clear_error(error_out: *mut *mut c_char) {
    if error_out.is_null() {
        return;
    }
    // SAFETY: `error_out` is an optional out-parameter supplied by the caller.
    unsafe {
        *error_out = ptr::null_mut();
    }
}

/// Converts a Rust `String` into a heap-allocated C string the caller is
/// responsible for freeing via `orchustr_bridge_free_string`.
///
/// Fails (`Err`) only if `value` contains an interior NUL byte. Returning
/// a `Result` lets callers distinguish a real error from "operation
/// succeeded with empty output", which the previous null-on-failure
/// convention conflated.
fn into_raw_string(value: String) -> Result<*mut c_char, BridgeError> {
    CString::new(value).map(CString::into_raw).map_err(|error| {
        BridgeError::InvalidInput(format!(
            "bridge output contained an interior NUL at byte {}",
            error.nul_position()
        ))
    })
}

fn from_ptr(value: *const c_char, field: &str) -> Result<String, BridgeError> {
    if value.is_null() {
        return Err(BridgeError::InvalidInput(format!(
            "{field} pointer must not be null"
        )));
    }
    // SAFETY: non-null pointers are expected to reference valid NUL-terminated C strings.
    let raw = unsafe { CStr::from_ptr(value) };
    raw.to_str()
        .map(str::to_owned)
        .map_err(|_| BridgeError::InvalidInput(format!("{field} must be valid UTF-8")))
}

/// Translates a `Result<String, BridgeError>` into the FFI ABI:
/// returns a heap-allocated C string on success, or null with the error
/// written into `error_out` on failure.
fn finish(result: Result<String, BridgeError>, error_out: *mut *mut c_char) -> *mut c_char {
    match result.and_then(into_raw_string) {
        Ok(ptr) => {
            clear_error(error_out);
            ptr
        }
        Err(error) => {
            write_error(error_out, error);
            ptr::null_mut()
        }
    }
}

/// Returns the bridge crate version as a NUL-terminated UTF-8 C string.
///
/// # Allocation
/// The returned pointer is owned by the Rust side. The caller **must**
/// free it with [`orchustr_bridge_free_string`]; calling the system
/// `free` would mismatch allocators and corrupt the heap. Returns null
/// only if `CARGO_PKG_VERSION` itself contained an interior NUL byte
/// (impossible in practice).
#[unsafe(no_mangle)]
pub extern "C" fn orchustr_bridge_version() -> *mut c_char {
    into_raw_string(env!("CARGO_PKG_VERSION").to_owned()).unwrap_or(ptr::null_mut())
}

#[unsafe(no_mangle)]
/// Render a prompt template against a JSON object context.
///
/// # Safety
/// `template` and `context_json` must be valid pointers to NUL-terminated UTF-8 strings.
/// `error_out`, when non-null, must point to writable memory for a single string pointer.
pub unsafe extern "C" fn orchustr_render_prompt_json(
    template: *const c_char,
    context_json: *const c_char,
    error_out: *mut *mut c_char,
) -> *mut c_char {
    let result = from_ptr(template, "template")
        .and_then(|template| {
            from_ptr(context_json, "context_json").map(|context| (template, context))
        })
        .and_then(|(template, context)| render_prompt_json(&template, &context));
    finish(result, error_out)
}

#[unsafe(no_mangle)]
/// Normalize a JSON object string through the bridge layer.
///
/// # Safety
/// `raw_state` must be a valid pointer to a NUL-terminated UTF-8 string.
/// `error_out`, when non-null, must point to writable memory for a single string pointer.
pub unsafe extern "C" fn orchustr_normalize_state_json(
    raw_state: *const c_char,
    error_out: *mut *mut c_char,
) -> *mut c_char {
    let result = from_ptr(raw_state, "raw_state").and_then(|state| normalize_state_json(&state));
    finish(result, error_out)
}

#[unsafe(no_mangle)]
/// Return the workspace crate-binding catalog as JSON.
///
/// # Safety
/// `error_out`, when non-null, must point to writable memory for a single string pointer.
pub unsafe extern "C" fn orchustr_workspace_catalog_json(
    error_out: *mut *mut c_char,
) -> *mut c_char {
    finish(workspace_catalog_json(), error_out)
}

#[unsafe(no_mangle)]
/// Invoke a crate operation through the generic JSON bridge.
///
/// # Safety
/// `crate_name`, `operation`, and `payload_json` must be valid pointers to NUL-terminated UTF-8 strings.
/// `error_out`, when non-null, must point to writable memory for a single string pointer.
pub unsafe extern "C" fn orchustr_invoke_crate_json(
    crate_name: *const c_char,
    operation: *const c_char,
    payload_json: *const c_char,
    error_out: *mut *mut c_char,
) -> *mut c_char {
    let result = from_ptr(crate_name, "crate_name")
        .and_then(|crate_name| {
            from_ptr(operation, "operation").and_then(|operation| {
                from_ptr(payload_json, "payload_json")
                    .map(|payload| (crate_name, operation, payload))
            })
        })
        .and_then(|(crate_name, operation, payload_json)| {
            invoke_crate_json(&crate_name, &operation, &payload_json)
        });
    finish(result, error_out)
}

#[unsafe(no_mangle)]
/// Release a string previously allocated by this bridge.
///
/// # Safety
/// `value` must be either null or a pointer returned by `CString::into_raw` in this library.
pub unsafe extern "C" fn orchustr_bridge_free_string(value: *mut c_char) {
    if value.is_null() {
        return;
    }
    // SAFETY: `value` must have been allocated by `CString::into_raw` in this library.
    unsafe {
        drop(CString::from_raw(value));
    }
}
