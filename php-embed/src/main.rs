use php_embed_sys::{sigan_zend_string_init, sigan_zend_string_release, zend_string};
use std::{
    borrow::Cow,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

fn main() {
    unsafe {
        php_embed_sys::php_embed_init(0, std::ptr::null_mut());

        php_embed_sys::php_request_startup();

        // php_embed_sys::php_register_variable();

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
    pub fn from_mut_ptr<'a>(ptr: *mut zend_string) -> &'a mut Self {
        unsafe { (ptr as *mut Self).as_mut() }.expect("Pointer should not be null.")
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_string {
        &mut self.inner
    }
}

pub struct ZString {
    inner: *mut ZStr,
}

impl ZString {
    pub fn new<'a>(value: Cow<'a, str>) -> Self {
        unsafe {
            let ptr = sigan_zend_string_init(value.as_ptr().cast(), value.len());

            Self {
                inner: ZStr::from_mut_ptr(ptr),
            }
        }
    }
}

impl Deref for ZString {
    type Target = ZStr;

    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref().unwrap() }
    }
}

impl DerefMut for ZString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.inner.as_mut().unwrap() }
    }
}

impl Drop for ZString {
    fn drop(&mut self) {
        unsafe { sigan_zend_string_release(self.inner.as_mut().unwrap().as_mut_ptr()) }
    }
}
