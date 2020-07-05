use jsc_bindgen::{JSStringRef};
use jsc_bindgen as jsc;
use std::ptr;
/// A wrapper around JSC strings.
/// They are UTF-16 character buffers and other meta information used by JSC
pub struct JSString {
  ptr: JSStringRef,
}
impl JSString {
  ///
  /// # Safety
  ///
  /// ptr must be pointing to a valid JSString
  pub unsafe fn from_raw(ptr: JSStringRef) -> JSString {
    JSString {
      ptr: jsc::JSStringRetain(ptr),
    }
  }
  pub fn get_raw(&self) -> JSStringRef {
    self.ptr
  }
}
impl From<&JSString> for String {
  fn from(s: &JSString) -> String {
    let buffer_capacity = unsafe { jsc::JSStringGetMaximumUTF8CStringSize(s.ptr) };
    let mut buffer: Vec<u8> = Vec::with_capacity(buffer_capacity as usize);
    unsafe {
      let buffer_len =
        jsc::JSStringGetUTF8CString(s.ptr, buffer.as_mut_ptr() as *mut i8, buffer_capacity);
      // -1 because String doesn't need to be zero terminated
      buffer.set_len(buffer_len as usize - 1);
      String::from_utf8_unchecked(buffer)
    }
  }
}

impl From<&str> for JSString {
  fn from(s: &str) -> JSString {
    let cstr = std::ffi::CString::new(s).unwrap();
    JSString {
      ptr: unsafe { jsc::JSStringCreateWithUTF8CString(cstr.as_ptr()) },
    }
  }
}

impl Drop for JSString {
  fn drop(&mut self) {
    if !self.ptr.is_null() {
      let ptr = self.ptr;
      self.ptr = ptr::null_mut();
      unsafe { jsc::JSStringRelease(ptr) };
    }
  }
}
impl Clone for JSString {
  fn clone(&self) -> JSString {
    unsafe { JSString::from_raw(self.ptr) }
  }
}