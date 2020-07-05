use super::JSGlobalContext;
use jsc_bindgen as jsc;
use jsc_bindgen::JSValueRef;
use std::ptr;

pub struct JSValue {
  ptr: JSValueRef,
  ctx: JSGlobalContext,
}
impl Drop for JSValue {
  fn drop(&mut self) {
    unsafe {
      if !self.ptr.is_null() {
        jsc::JSValueUnprotect(self.ctx.raw(), self.ptr);
        self.ptr = ptr::null();
      }
    }
  }
}

impl JSValue {
  ///
  /// # Safety
  /// ptr should be a valid JSValue.
  /// ctx must own ptr.
  pub unsafe fn from_raw(ctx: JSGlobalContext, ptr: JSValueRef) -> JSValue {
    jsc::JSValueProtect(ctx.raw(), ptr);
    JSValue { ctx, ptr }
  }
  pub fn context(&self) -> &JSGlobalContext {
    &self.ctx
  }
  pub fn raw(&self) -> JSValueRef {
    self.ptr
  }
}
