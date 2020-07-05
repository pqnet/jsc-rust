use super::*;

#[test]
fn jsstring_can_roundtrip_unicode() {
  let hello = "你好";
  let s = JSString::from(hello);
  let hello2 = String::from(&s);
  assert_eq!(hello, hello2);
}