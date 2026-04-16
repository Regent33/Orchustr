use crate::{BridgeError, normalize_state_json, render_prompt_json};
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

fn into_raw_string(value: String) -> *mut c_char {
    match CString::new(value) {
        Ok(text) => text.into_raw(),
        Err(_) => ptr::null_mut(),
    }
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

#[unsafe(no_mangle)]
pub extern "C" fn orchustr_bridge_version() -> *mut c_char {
    into_raw_string(env!("CARGO_PKG_VERSION").to_owned())
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
    match result {
        Ok(rendered) => {
            clear_error(error_out);
            into_raw_string(rendered)
        }
        Err(error) => {
            write_error(error_out, error);
            ptr::null_mut()
        }
    }
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
    match from_ptr(raw_state, "raw_state").and_then(|state| normalize_state_json(&state)) {
        Ok(normalized) => {
            clear_error(error_out);
            into_raw_string(normalized)
        }
        Err(error) => {
            write_error(error_out, error);
            ptr::null_mut()
        }
    }
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
