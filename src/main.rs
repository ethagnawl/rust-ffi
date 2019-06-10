extern crate libc;
use libc::{c_char, c_void};
use std::ffi::CString;

#[link(name = "hello", kind = "static")]
extern "C" {
    fn hello(name: *const c_char) -> c_void;
}

fn main() {
    let entity = CString::new("World").expect("Default");

    unsafe {
        hello(entity.as_ptr());
    }
}
