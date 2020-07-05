//! Low level wrapper of C ffi
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
  use crate as jsc;
  use std::ptr::null_mut as null;

  unsafe fn set_property(
    ctx: jsc::JSContextRef,
    parent: jsc::JSObjectRef,
    child: jsc::JSObjectRef,
    name: &str,
  ) {
    let name_cstr = std::ffi::CString::new(name).unwrap();
    let obj_name = jsc::JSStringCreateWithUTF8CString(name_cstr.as_ptr());
    jsc::JSObjectSetProperty(
      ctx,
      parent,
      obj_name,
      child,
      jsc::kJSPropertyAttributeDontDelete,
      null(),
    );
    jsc::JSStringRelease(obj_name);
  }

  fn to_jsstring(ctx: jsc::JSContextRef, value: jsc::JSValueRef) -> jsc::JSStringRef {
    let mut exception: jsc::JSValueRef = null();
    let ret = unsafe { jsc::JSValueToStringCopy(ctx, value, &mut exception) };
    if !exception.is_null() {
      to_jsstring(ctx, exception)
    } else {
      ret
    }
  }

  fn to_string(value: jsc::JSStringRef) -> String {
    let buffer_capacity = unsafe { jsc::JSStringGetMaximumUTF8CStringSize(value) };
    let mut buffer: Vec<u8> = Vec::with_capacity(buffer_capacity as usize);
    unsafe {
      let buffer_len =
        jsc::JSStringGetUTF8CString(value, buffer.as_mut_ptr() as *mut i8, buffer_capacity);
      buffer.set_len(buffer_len as usize - 1);
      String::from_utf8_unchecked(buffer)
    }
  }
  fn print_obj(ctx: jsc::JSContextRef, value: jsc::JSValueRef) {
    let js_string = to_jsstring(ctx, value);
    if js_string.is_null() {
      println!("(nullptr)");
    } else {
      println!("{}", to_string(js_string));
      unsafe { jsc::JSStringRelease(js_string) }
    }
  }

  unsafe extern "C" fn log_fn(
    ctx: jsc::JSContextRef,
    _function: jsc::JSObjectRef,
    _thisObject: jsc::JSObjectRef,
    argumentCount: jsc::size_t,
    arguments: *const jsc::JSValueRef,
    _exception: *mut jsc::JSValueRef,
  ) -> jsc::JSValueRef {
    let args = std::slice::from_raw_parts(arguments, argumentCount as usize);
    for arg in args {
      print_obj(ctx, *arg);
    }
    jsc::JSValueMakeUndefined(ctx)
  }

  #[test]
  fn try_run() {
    // let _def = jsc::JSClassDefinition {
    //   version: 0,
    //   attributes: 0,
    //   className: null(),
    //   parentClass: null(),
    //   staticValues: null(),
    //   staticFunctions: null(),
    //   initialize: None,
    //   finalize: None,
    //   hasProperty: None,
    //   getProperty: None,
    //   setProperty: None,
    //   deleteProperty: None,
    //   getPropertyNames: None,
    //   callAsFunction: None,
    //   callAsConstructor: None,
    //   hasInstance: None,
    //   convertToType: None,
    // };
    unsafe {
      let main_vm = jsc::JSContextGroupCreate();
      let main_ctx = jsc::JSGlobalContextCreateInGroup(main_vm, null());
      let global_object = jsc::JSContextGetGlobalObject(main_ctx);
      let console = jsc::JSObjectMake(main_ctx, null(), null());
      set_property(main_ctx, global_object, console, "konsole");
      set_property(main_ctx, global_object, global_object, "globalThis");

      let log = jsc::JSObjectMakeFunctionWithCallback(main_ctx, null(), Some(log_fn));
      set_property(main_ctx, console, log, "log");

      let code: &'static str = include_str!("test.js");
      let mut exception: jsc::JSValueRef = null();

      let cstr = std::ffi::CString::new(code).unwrap();
      let script = jsc::JSStringCreateWithUTF8CString(cstr.as_ptr());
      let value = jsc::JSEvaluateScript(main_ctx, script, null(), null(), 1, &mut exception);
      if !exception.is_null() {
        print!("E> ");
        print_obj(main_ctx, exception);
        panic!("Error")
      }
      if !value.is_null() {
        print!("V> ");
        print_obj(main_ctx, value);
      }
    }
  }
}
