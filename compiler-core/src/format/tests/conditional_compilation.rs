use crate::assert_format;

#[test]
fn multiple() {
    assert_format!(
        "type X

@target(erlang)
record Y {
  Y
}

@target(javascript)
record Z {
  Z
}
"
    );
}
