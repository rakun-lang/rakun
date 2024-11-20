use crate::assert_format;

#[test]
fn one() {
    assert_format!(
        "pub record Counter {
  Counter(a: Int, b: Int)
}

fn main() {
  let c = Counter(0, 0)
  let c = Counter(..c, a: c.a + 1, b: c.a + c.b)
  c
}
"
    );
}

#[test]
fn two() {
    // Long record updates are split onto multiple lines
    assert_format!(
        "pub record Counter {
  Counter(loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong: Int)
}

fn main() {
  let c = Counter(0)
  let c =
    Counter(
      ..c,
      loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong: 1,
    )
  c
}
"
    );
}


#[test]
fn comment_before_spread() {
    assert_format!(
        r#"fn main() {
  Thingy(
    // Def?
    // Def!
    ..thingy.defaults,
    one: One,
  )
}
"#
    );
}


#[test]
fn comment_before_update_label() {
    assert_format!(
        r#"fn main() {
  Thingy(
    ..thingy.defaults,
    // Def?
    // Def!
    one: One,
  )
}
"#
    );
}


#[test]
fn multiple_line_custom_type_field_comments() {
    assert_format!(
        r#"fn main() {
  Thingy(
    // Def?
    // Def!
    ..thingy.defaults,
    // One?
    // One!
    one: One,
    // Two?
    // Two!
    two: Two,
  )
}
"#
    );
}
