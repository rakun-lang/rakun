use crate::assert_erl;


#[test]
fn alternative_pattern_variable_rewriting() {
    assert_erl!(
        "
pub fn myfun(mt) {
  case mt {
    1 | _ ->
      1
      |> Ok
  }
  1
  |> Ok
}
"
    )
}


#[test]
fn positive_zero_pattern() {
    assert_erl!(
        "
pub fn main(x) {
  case x {
    0.0 -> 1
    _ -> 2
  }
}
"
    )
}


#[test]
fn negative_zero_pattern() {
    assert_erl!(
        "
pub fn main(x) {
  case x {
    -0.0 -> 1
    _ -> 2
  }
}
"
    )
}

#[test]
fn not() {
    assert_erl!(
        r#"pub fn main(x, y) {
  case x {
    _ if !y -> 0
    _ -> 1
  }
}
"#,
    );
}

#[test]
fn not_two() {
    assert_erl!(
        r#"pub fn main(x, y) {
  case x {
    _ if !y && !x -> 0
    _ -> 1
  }
}
"#,
    );
}


#[test]
fn spread_empty_list() {
    assert_erl!(
        r#"
pub fn main() {
  case [] {
    [..] -> 1
  }
}
"#,
    );
}


#[test]
fn spread_empty_list_assigning() {
    assert_erl!(
        r#"
pub fn main() {
  case [] {
    [..rest] -> rest
  }
}
"#,
    );
}
