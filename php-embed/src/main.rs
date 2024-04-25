use php_embed_sys::zend_string;
use std::borrow::Cow;
use std::marker::PhantomData;

fn main() {
    unsafe {
        php_embed_sys::php_embed_init(0, std::ptr::null_mut());

        php_embed_sys::php_request_startup();

        // php_embed_sys::php_execute_script(primary_file);

        php_embed_sys::php_request_shutdown(std::ptr::null_mut());

        php_embed_sys::php_embed_shutdown();
    }
}

#[repr(transparent)]
pub struct ZStr {
    inner: zend_string,
    _ptr: PhantomData<*mut ()>,
}

impl ZStr {
    pub fn from_ptr<'a>(ptr: *const zend_string) -> &'a Self {
        unsafe { (ptr as *const Self).as_ref() }.expect("Pointer should not be null.")
    }

    pub fn from_mut_ptr<'a>(ptr: *mut zend_string) -> &'a mut Self {
        unsafe { (ptr as *mut Self).as_mut() }.expect("Pointer should not be null.")
    }
}

pub struct ZString {
    inner: *mut ZStr,
}

impl ZString {
    fn new<'a>(value: Cow<'a, str>) -> Self {
        unsafe {
            let mut ptr = php_embed_sys::sigan_zend_string_init_fast(
                value.into_owned().as_ptr(),
                value.len(),
            );
        }

        Self {
            inner: ZStr::from_mut_ptr(ptr),
        }
    }
}
