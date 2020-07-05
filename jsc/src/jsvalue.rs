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
      // safeguard against uninitialized values (e.g. exception buffers)
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
  pub fn loosely_equal(&self, other: &JSValue) -> Result<bool, JSValue> {
    if self.ctx.raw() != other.ctx.raw() {
      panic!("cannot compare values from two different contexts")
    } else {
      let mut exception = unsafe { JSValue::from_raw(self.ctx.clone(), ptr::null()) };
      let result =
        unsafe { jsc::JSValueIsEqual(self.ctx.raw(), self.ptr, other.ptr, &mut exception.ptr) };
      if exception.ptr.is_null() {
        Ok(result)
      } else {
        Err(exception)
      }
    }
  }
  pub fn equal(&self, other: &JSValue) -> bool {
    if self.ctx.raw() != other.ctx.raw() {
      panic!("cannot compare values from two different contexts")
    } else {
      unsafe { jsc::JSValueIsStrictEqual(self.ctx.raw(), self.ptr, other.ptr) }
    }
  }
}
impl PartialEq for JSValue {
  fn eq(&self, other: &JSValue) -> bool {
    self.equal(other)
  }
}
