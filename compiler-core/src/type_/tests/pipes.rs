use crate::{assert_module_error, assert_module_infer};

// https://github.com/rakun-lang/rakun/issues/2392
#[test]
fn empty_list() {
    assert_module_infer!(
        "
pub fn a() {
  fn(_) { Nil }
}

pub fn b(_) {
  fn(_) { Nil }
}

pub fn c() {
  Nil
  |> b(
    Nil
    |> a(),
  )
}",
        vec![
            ("a", "fn() -> fn(a) -> Nil"),
            ("b", "fn(a) -> fn(b) -> Nil"),
            ("c", "fn() -> Nil"),
        ]
    );
}

// https://github.com/rakun-lang/rakun/pull/3406#discussion_r1683068647
#[test]
fn pipe_rewrite_with_missing_argument() {
    assert_module_infer!(
        r#"
pub fn main() {
  let f = fn(a, b) { fn(c) { a + b + c } }
  1 |> f(2)
}
"#,
        vec![("main", "fn() -> fn(Int) -> Int")]
    );
}

#[test]
fn pipe_regression_gh3515() {
    // https://github.com/rakun-lang/rakun/issues/3515
    assert_module_infer!(
        r#"
fn relu(t) {
  fn(theta: String) {
    // use t and theta and return a Float
    0.0
  }
}

pub fn k_relu(k: Int) {
  fn(t: Float) {
    fn(theta: String) {
      case k {
        0 -> t
        _ -> {
          // following code is OK on rakun 1.3.2,
          // but raised error on rakun 1.4.1
          // The key here is that it is not a direct function call,
          // but a "var" call, which points to the same function.
          let next_layer = theta |> relu(t) |> k_relu(k - 1)
          theta |> next_layer
        }
      }
    }
  }
}"#,
        vec![("k_relu", "fn(Int) -> fn(Float) -> fn(String) -> Float")],
    );
}

#[test]
fn pipe_callback_var_function1() {
    assert_module_infer!(
        r#"
pub fn main() {
  let f = fn(a) { fn(b) { #(a, b) } }
  let x = 1 |> f()
}
"#,
        vec![("main", "fn() -> fn(a) -> #(Int, a)")],
    );
}

#[test]
fn pipe_callback_var_function2() {
    assert_module_infer!(
        r#"
pub fn main() {
  let f = fn(a) { fn(b) { #(a, b) } }
  let x = 1 |> f(1)
}
"#,
        vec![("main", "fn() -> #(Int, Int)")],
    );
}

#[test]
fn pipe_callback_correct_arity1() {
    assert_module_infer!(
        r#"
fn callback(a: Int) {
  fn() -> String {
    "Called"
  }
}

pub fn main() {
  let x = 1 |> callback()
}
"#,
        vec![("main", "fn() -> fn() -> String")],
    );
}

#[test]
fn pipe_callback_correct_arity2() {
    assert_module_infer!(
        r#"
fn callback(a: Float) {
  fn(b: Int) -> String {
    "Called"
  }
}

pub fn main() {
  let x = 1 |> callback(2.5)
}
"#,
        vec![("main", "fn() -> String")],
    );
}

#[test]
fn pipe_callback_wrong_arity() {
    assert_module_error!(
        r#"
fn callback(a: Int) {
  fn() -> String {
    "Called"
  }
}

pub fn main() {
  let x = 1 |> callback(2)
}
"#
    );
}
