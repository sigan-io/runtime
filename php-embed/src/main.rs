use php_embed::zend_string;
use std::borrow::Cow;
use std::ffi::c_char;
use std::marker::PhantomData;

fn main() {
    unsafe {
        php_embed::php_embed_init(0, std::ptr::null_mut());

        php_embed::php_request_startup();

        // php_embed::php_execute_script(primary_file);

        php_embed::php_request_shutdown(std::ptr::null_mut());

        php_embed::php_embed_shutdown();
    }
}

#[repr(transparent)]
pub struct ZStr {
    inner: zend_string,
    _ptr: PhantomData<*mut ()>,
}

impl ZStr {
    pub unsafe fn from_ptr<'a>(ptr: *const zend_string) -> &'a Self {
        (ptr as *const Self)
            .as_ref()
            .expect("Pointer should not be null.")
    }

    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_string) -> &'a mut Self {
        (ptr as *mut Self)
            .as_mut()
            .expect("Pointer should not be null.")
    }
}

pub struct ZString {
    inner: *mut ZStr,
}

// impl ZString {
//     fn new<'a>(value: Cow<'a, str>) -> Self {
//         // let ptr = zend_string_init_fast(value.into_owned().as_ptr());
//     }
// }
