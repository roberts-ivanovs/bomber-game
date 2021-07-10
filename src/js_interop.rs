// Docs: https://github.com/not-fl3/miniquad/wiki/JavaScript-interop
pub struct FromJS {
}
pub struct ToJS {
}

// ------------- WASM imports ------------- //
extern "C" {
    fn hi_from_js();
}

impl FromJS {
    pub fn hi_from_js() {
        unsafe { hi_from_js() };
    }
}

// ------------- WASM exports ------------- //
#[no_mangle]
extern "C" fn hi_from_rust() {
    FromJS::hi_from_js();
}

impl ToJS {
    pub fn hi_from_rust() {
        unsafe { hi_from_js() };
    }
}
