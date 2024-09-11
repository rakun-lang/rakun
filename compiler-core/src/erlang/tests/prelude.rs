use crate::assert_erl;

#[test]
fn qualified_prelude() {
    assert_erl!(
        "import rakun
pub record X { X(rakun.Int) }
"
    );

    assert_erl!(
        "import rakun
pub fn x() { rakun.Ok(1) }
"
    );
}
