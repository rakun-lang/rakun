use crate::assert_js;

#[test]
fn ok() {
    assert_js!(r#"pub fn main() { Ok(1) }"#);
}

#[test]
fn error() {
    assert_js!(r#"pub fn main() { Error(1) }"#);
}

#[test]
fn ok_fn() {
    assert_js!(r#"pub fn main() { Ok }"#);
}

#[test]
fn error_fn() {
    assert_js!(r#"pub fn main() { Error }"#);
}

#[test]
fn qualified_ok() {
    assert_js!(
        r#"import rakun
pub fn main() { rakun.Ok(1) }"#
    );
}

#[test]
fn qualified_error() {
    assert_js!(
        r#"import rakun
pub fn main() { rakun.Error(1) }"#
    );
}

#[test]
fn qualified_ok_fn() {
    assert_js!(
        r#"import rakun
pub fn main() { rakun.Ok }"#
    );
}

#[test]
fn qualified_error_fn() {
    assert_js!(
        r#"import rakun
pub fn main() { rakun.Error }"#
    );
}

#[test]
fn aliased_ok() {
    assert_js!(
        r#"import rakun.{Ok as Thing}
pub fn main() { Thing(1) }"#
    );
}

#[test]
fn aliased_error() {
    assert_js!(
        r#"import rakun.{Error as Thing}
pub fn main() { Thing(1) }"#
    );
}

#[test]
fn aliased_ok_fn() {
    assert_js!(
        r#"import rakun.{Ok as Thing}
pub fn main() { Thing }"#
    );
}

#[test]
fn aliased_error_fn() {
    assert_js!(
        r#"import rakun.{Error as Thing}
pub fn main() { Thing }"#
    );
}
