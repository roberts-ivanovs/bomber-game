// Docs: https://github.com/not-fl3/miniquad/wiki/JavaScript-interop

use sapp_jsutils::JsObject;
pub struct FromJS {}
pub struct ToJS {}

// ------------- WASM imports ------------- //

extern "C" {
    fn console_log_unsafe(to_log: JsObject);
}

impl FromJS {
    pub fn console_log(to_log: &str) {
        unsafe { console_log_unsafe(JsObject::string(&to_log)) };
    }
}

// ------------- WASM exports ------------- //
#[no_mangle]
extern "C" fn hi_from_rust() {}

impl ToJS {}
