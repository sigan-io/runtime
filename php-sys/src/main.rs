use php_sys::{php_embed_init, php_embed_shutdown, zend_eval_string};
use std::ffi::CString;

fn main() {
    unsafe {
        php_embed_init(0, std::ptr::null_mut());

        let code = CString::new("echo 'Hello from PHP!';").expect("Failed to create CString");
        let name = CString::new("Test Code").expect("Failed to create CString");
        zend_eval_string(code.as_ptr(), std::ptr::null_mut(), name.as_ptr());

        php_embed_shutdown();
    }
}
