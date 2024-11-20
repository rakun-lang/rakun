use crate::assert_format;


#[test]
fn comment() {
    assert_format!(
        r#"pub fn main() {
  // Hello
  let x = 1
  x
}
"#
    );
}


#[test]
fn assert_comment() {
    assert_format!(
        r#"pub fn main() {
  // Hello
  let assert x = 1
  x
}
"#
    );
}
