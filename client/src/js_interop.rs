// Docs: https://github.com/not-fl3/miniquad/wiki/JavaScript-interop

use sapp_jsutils::JsObject;
pub struct FromJS {

}
pub struct ToJS {

}

// ------------- Import from JS ------------- //

extern "C" {
    fn console_log_unsafe(to_log: JsObject);
    // fn connect_websocket_unsafe(ip: JsObject);
}

impl FromJS {
    pub fn console_log(to_log: &str) {
        unsafe { console_log_unsafe(JsObject::string(&to_log)) };
    }
    // pub fn connect_websocket(ip: &str) {
    //     unsafe { connect_websocket_unsafe(JsObject::string(&ip)) };
    // }
}

// ------------- Export to JS ------------- //


impl ToJS {

}
