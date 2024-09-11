use crate::assert_erl;

#[test]
fn build_in_erlang_type_escaping() {
    assert_erl!("pub type Map");
}

#[test]
fn escape_erlang_reserved_keywords_in_type_names() {
    // list of all reserved words in erlang
    // http://erlang.org/documentation/doc-5.8/doc/reference_manual/introduction.html
    assert_erl!(
        r#"pub record After { TestAfter }
pub record And { TestAnd }
pub record Andalso { TestAndAlso }
pub record Band { TestBAnd }
pub record Begin { TestBegin }
pub record Bnot { TestBNot }
pub record Bor { TestBOr }
pub record Bsl { TestBsl }
pub record Bsr { TestBsr }
pub record Bxor { TestBXor }
pub record Case { TestCase }
pub record Catch { TestCatch }
pub record Cond { TestCond }
pub record Div { TestDiv }
pub record End { TestEnd }
pub record Fun { TestFun }
pub record If { TestIf }
pub record Let { TestLet }
pub record Maybe { TestMaybe }
pub record Not { TestNot }
pub record Of { TestOf }
pub record Or { TestOr }
pub record Orelse { TestOrElse }
pub record Query { TestQuery }
pub record Receive { TestReceive }
pub record Rem { TestRem }
pub record Try { TestTry }
pub record When { TestWhen }
pub record Xor { TestXor }"#
    );
}
