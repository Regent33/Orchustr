use or_bridge::{
    orchustr_bridge_free_string, orchustr_normalize_state_json, orchustr_render_prompt_json,
};
use std::ffi::{CStr, CString, c_char};
use std::ptr;

fn take_string(value: *mut c_char) -> String {
    assert!(!value.is_null());
    // SAFETY: test cases only pass pointers returned by the bridge library.
    let text = unsafe { CStr::from_ptr(value) }
        .to_str()
        .unwrap()
        .to_owned();
    // SAFETY: test cases only pass pointers returned by the bridge library.
    unsafe {
        orchustr_bridge_free_string(value);
    }
    text
}

#[test]
fn dart_bridge_render_returns_allocated_string() {
    let template = CString::new("Hello {{name}}").unwrap();
    let context = CString::new("{\"name\":\"Ralph\"}").unwrap();
    let mut error: *mut c_char = ptr::null_mut();
    // SAFETY: the test passes valid NUL-terminated pointers and a writable error out-pointer.
    let rendered = unsafe {
        orchustr_render_prompt_json(template.as_ptr(), context.as_ptr(), &mut error as *mut _)
    };
    assert!(error.is_null());
    assert_eq!(take_string(rendered), "Hello Ralph");
}

#[test]
fn dart_bridge_reports_failures_via_error_pointer() {
    let raw_state = CString::new("[1,2,3]").unwrap();
    let mut error: *mut c_char = ptr::null_mut();
    // SAFETY: the test passes a valid NUL-terminated pointer and a writable error out-pointer.
    let normalized =
        unsafe { orchustr_normalize_state_json(raw_state.as_ptr(), &mut error as *mut _) };
    assert!(normalized.is_null());
    assert_eq!(take_string(error), "state must serialize to a JSON object");
}
