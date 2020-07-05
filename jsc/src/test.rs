use super::*;

#[test]
fn jsstring_can_roundtrip_unicode() {
  let hello = "你好";
  let s = JSString::from(hello);
  let hello2 = String::from(&s);
  assert_eq!(hello, hello2);
}

#[test]
fn jscontext_can_create_standalone() {
  let ctx = JSGlobalContext::new();
  assert!(!ctx.raw().is_null());
}

#[test]
fn jscontext_can_create_group() {
  let group = JSContextGroup::new();
  assert!(!group.raw().is_null());
  let ctx = JSGlobalContext::new_in_context_group(group);
  assert!(!ctx.raw().is_null());
}

#[test]
fn jsvalue_has_global_context() {
  let ctx = JSGlobalContext::new();
  assert!(!ctx.raw().is_null());
  let global_this = ctx.global_object();
  assert!(!global_this.raw().is_null());
}
struct JsValues {
  js_null: JSValue,
  js_undefined: JSValue,
  js_true: JSValue,
  js_false: JSValue,
}

fn mk_values(ctx: &JSGlobalContext) -> JsValues {
  JsValues {
    js_undefined: ctx.new_undefined(),
    js_null: ctx.new_null(),
    js_true: ctx.new_boolean(true),
    js_false: ctx.new_boolean(false),
  }
}

#[test]
fn strict_equality() {
  let ctx = JSGlobalContext::new();
  assert!(!ctx.raw().is_null());
  let JsValues {
    js_undefined,
    js_null,
    js_true,
    js_false,
  } = mk_values(&ctx);
  assert!(js_undefined == ctx.new_undefined());
  assert!(js_null == ctx.new_null());
  assert!(js_true == ctx.new_boolean(true));
  assert!(js_false == ctx.new_boolean(false));
  assert!(js_true != js_false, "true === false");
  assert!(js_true != js_null, "true === null");
  assert!(js_true != js_undefined, "true === undefined");
  assert!(js_false != js_null, "false === null");
  assert!(js_false != js_undefined, "false === undefined");
  assert!(js_null != js_undefined, "null === undefined");
}

#[test]
fn loose_equality() {
  let ctx = JSGlobalContext::new();
  assert!(!ctx.raw().is_null());
  let JsValues {
    js_undefined,
    js_null,
    js_true,
    js_false,
  } = mk_values(&ctx);
  assert!(js_undefined.loosely_equal(&js_null) == Ok(true), "undefined != null");
  assert!(js_undefined.loosely_equal(&js_false) == Ok(false), "undefined == false");
  assert!(js_null.loosely_equal(&js_false) == Ok(false), "null == false");
  assert!(js_true.loosely_equal(&js_false) == Ok(false), "true == false");
}
