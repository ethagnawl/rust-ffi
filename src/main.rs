use libc::{c_char, c_void};
use std::ffi::{CStr, CString};

#[link(name = "greet", kind = "static")]
extern "C" {
    fn display_greeting(name: *const c_char) -> c_void;
    fn punctuate_greeting(name: *const c_char) -> *const c_char;
}

fn main() {
    let entity = CString::new("Hello, World").expect("Default");
    // I _believe_ decorated_entity won't need to be explicitly free'd because
    // it's a CStr.
    let decorated_entity: &CStr =
        unsafe { CStr::from_ptr(punctuate_greeting(entity.as_ptr())) };

    unsafe {
        display_greeting(decorated_entity.as_ptr());
    }
}
