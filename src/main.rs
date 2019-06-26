use libc::{c_char, c_void};
use std::ffi::{CStr, CString};

#[link(name = "yaml")]
extern "C" {
    fn yaml_get_version_string() -> *const c_char;
}

#[link(name = "greet", kind = "static")]
extern "C" {
    fn display_greeting(name: *const c_char) -> c_void;
    fn punctuate_greeting(name: *const c_char) -> *const c_char;
}

fn main() {
    let entity = CString::new("Hello, World").unwrap();
    // I _believe_ decorated_entity won't need to be explicitly free'd because
    // it's a CStr.
    let decorated_entity: &CStr =
        unsafe { CStr::from_ptr(punctuate_greeting(entity.as_ptr())) };

    unsafe {
        display_greeting(decorated_entity.as_ptr());
    }

    let yaml_version: &CStr =
        unsafe { CStr::from_ptr(yaml_get_version_string()) };
    let yaml_version = yaml_version
        .to_str()
        .expect("Unable to detect YAML version");
    let yaml_version_message_prefix: &str = "You're running YAML version:";
    let yaml_version_message = CString::new(format!(
        "{} {}",
        yaml_version_message_prefix, yaml_version
    ))
    .unwrap();

    unsafe {
        display_greeting(yaml_version_message.as_ptr());
    }
}
