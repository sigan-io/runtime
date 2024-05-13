use php_embed_sys::{
    sigan_zend_string_init, sigan_zend_string_release, zend_file_handle, zend_string,
};
use std::{mem, ptr};

fn main() {
    unsafe {
        php_embed_sys::php_embed_init(0, std::ptr::null_mut());

        php_embed_sys::php_request_startup();

        // php_embed_sys::php_register_variable();
        let mut file_handle = ZFileHandle::new("/mnt/wordpress/index.php");

        php_embed_sys::php_execute_script(file_handle.as_mut_ptr());

        // pub struct _zend_file_handle {
        //     pub handle: _zend_file_handle__bindgen_ty_1,
        //     pub filename: *mut zend_string,
        //     pub opened_path: *mut zend_string,
        //     pub type_: u8,
        //     pub primary_script: bool,
        //     pub in_list: bool,
        //     pub buf: *mut ::std::os::raw::c_char,
        //     pub len: usize,
        // }

        php_embed_sys::php_request_shutdown(std::ptr::null_mut());

        php_embed_sys::php_embed_shutdown();
    }
}

pub struct ZString {
    inner: *mut zend_string,
}

impl ZString {
    pub fn new(value: &str) -> Self {
        unsafe {
            Self {
                inner: sigan_zend_string_init(value.as_ptr().cast(), value.len()),
            }
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_string {
        self.inner
    }
}

impl Drop for ZString {
    fn drop(&mut self) {
        unsafe { sigan_zend_string_release(self.inner) }
    }
}

#[repr(transparent)]
pub struct ZFileHandle {
    inner: *mut zend_file_handle,
}

impl ZFileHandle {
    pub fn new(filename: &str) -> Self {
        let mut file_handle: zend_file_handle = unsafe { mem::zeroed() };

        file_handle.filename = ZString::new(filename).as_mut_ptr();

        Self {
            inner: &mut file_handle,
        }
    }

    pub fn default() -> Self {
        let mut file_handle: zend_file_handle = unsafe { mem::zeroed() };

        Self {
            inner: &mut file_handle,
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_file_handle {
        self.inner
    }
}

impl Drop for ZFileHandle {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self.inner) }
    }
}

// pub struct ZStr {
//     inner: zend_string,
//     _ptr: PhantomData<*mut ()>,
// }

// impl ZStr {
//     pub fn from_mut_ptr<'a>(ptr: *mut zend_string) -> &'a mut Self {
//         unsafe { (ptr as *mut Self).as_mut() }.expect("Pointer should not be null.")
//     }

//     pub fn as_mut_ptr(&mut self) -> *mut zend_string {
//         &mut self.inner
//     }
// }

// pub struct ZString {
//     inner: *mut ZStr,
// }

// impl ZString {
//     pub fn new<'a>(value: Cow<'a, str>) -> Self {
//         unsafe {
//             let ptr = sigan_zend_string_init(value.as_ptr().cast(), value.len());

//             Self {
//                 inner: ZStr::from_mut_ptr(ptr),
//             }
//         }
//     }
// }

// impl Deref for ZString {
//     type Target = ZStr;

//     fn deref(&self) -> &Self::Target {
//         unsafe { self.inner.as_ref().unwrap() }
//     }
// }

// impl DerefMut for ZString {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         unsafe { self.inner.as_mut().unwrap() }
//     }
// }

// impl Drop for ZString {
//     fn drop(&mut self) {
//         unsafe { sigan_zend_string_release(self.inner.as_mut().unwrap().as_mut_ptr()) }
//     }
// }
