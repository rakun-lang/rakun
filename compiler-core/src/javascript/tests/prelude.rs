use crate::{assert_js, assert_ts_def};

#[test]
fn qualified_ok() {
    assert_js!(
        r#"import rakun
pub fn go() { rakun.Ok(1) }
"#,
    );
}

#[test]
fn qualified_ok_typescript() {
    assert_ts_def!(
        r#"import rakun
pub fn go() { rakun.Ok(1) }
"#,
    );
}

#[test]
fn qualified_error() {
    assert_js!(
        r#"import rakun
pub fn go() { rakun.Error(1) }
"#,
    );
}

#[test]
fn qualified_nil() {
    assert_js!(
        r#"import rakun
pub fn go() { rakun.Nil }
"#,
    );
}

#[test]
fn qualified_nil_typescript() {
    assert_ts_def!(
        r#"import rakun
pub fn go() { rakun.Nil }
"#,
    );
}
