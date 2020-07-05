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