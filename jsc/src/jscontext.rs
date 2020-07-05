use super::JSValue;
use jsc_bindgen as jsc;
use jsc_bindgen::{JSContextGroupRef, JSGlobalContextRef};
use std::ptr;

pub struct JSContextGroup {
  ptr: JSContextGroupRef,
}

impl JSContextGroup {
  pub fn new() -> JSContextGroup {
    JSContextGroup {
      ptr: unsafe { jsc::JSContextGroupCreate() },
    }
  }
  pub fn raw(&self) -> JSContextGroupRef {
    self.ptr
  }
}

impl Default for JSContextGroup {
  fn default() -> Self {
    Self::new()
  }
}

impl Drop for JSContextGroup {
  fn drop(&mut self) {
    unsafe { jsc::JSContextGroupRelease(self.ptr) }
  }
}
impl Clone for JSContextGroup {
  fn clone(&self) -> Self {
    JSContextGroup {
      ptr: unsafe { jsc::JSContextGroupRetain(self.ptr) },
    }
  }
}

pub struct JSGlobalContext {
  ptr: JSGlobalContextRef,
}
impl JSGlobalContext {
  /// # Safety
  /// ptrs must be valid. The created object takes ownership of the pointed data
  pub unsafe fn from_raw(ptr: JSGlobalContextRef) -> Self {
    Self { ptr }
  }
  pub fn new_in_context_group(context_group: JSContextGroup) -> Self {
    let ptr = unsafe { jsc::JSGlobalContextCreateInGroup(context_group.ptr, ptr::null_mut()) };
    Self { ptr }
  }
  pub fn new() -> Self {
    let ptr = unsafe { jsc::JSGlobalContextCreate(ptr::null_mut()) };
    Self { ptr }
  }
  pub fn context_group(&self) -> JSContextGroup {
    JSContextGroup {
      ptr: unsafe { jsc::JSContextGetGroup(self.ptr) },
    }
  }
  pub fn raw(&self) -> JSGlobalContextRef {
    self.ptr
  }

  pub fn global_object(&self) -> JSValue {
    unsafe { JSValue::from_raw(self.clone(), jsc::JSContextGetGlobalObject(self.ptr)) }
  }
  pub fn new_undefined(&self) -> JSValue {
    unsafe { JSValue::from_raw(self.clone(), jsc::JSValueMakeUndefined(self.ptr)) }
  }
  pub fn new_null(&self) -> JSValue {
    unsafe { JSValue::from_raw(self.clone(), jsc::JSValueMakeNull(self.ptr)) }
  }
  pub fn new_boolean(&self, value: bool) -> JSValue {
    unsafe { JSValue::from_raw(self.clone(), jsc::JSValueMakeBoolean(self.ptr, value)) }
  }
}
impl Drop for JSGlobalContext {
  fn drop(&mut self) {
    unsafe { jsc::JSGlobalContextRelease(self.ptr) }
  }
}
impl Clone for JSGlobalContext {
  fn clone(&self) -> Self {
    Self {
      ptr: unsafe { jsc::JSGlobalContextRetain(self.ptr) },
    }
  }
}
