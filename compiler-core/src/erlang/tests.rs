use camino::Utf8PathBuf;

use crate::analyse::TargetSupport;
use crate::config::PackageConfig;
use crate::type_::PRELUDE_MODULE_NAME;
use crate::warning::WarningEmitter;
use crate::{
    build::{Origin, Target},
    erlang::module,
    line_numbers::LineNumbers,
    uid::UniqueIdGenerator,
    warning::TypeWarningEmitter,
};

mod bit_arrays;
mod case;
mod conditional_compilation;
mod consts;
mod custom_types;
mod external_fn;
mod functions;
mod guards;
mod html;
mod let_assert;
mod numbers;
mod panic;
mod patterns;
mod pipes;
mod records;
mod reserved;
mod strings;
mod todo;
mod type_params;
mod use_;
mod variables;

pub fn compile_test_project(src: &str, src_path: &str, dep: Option<(&str, &str, &str)>) -> String {
    let mut modules = im::HashMap::new();
    let ids = UniqueIdGenerator::new();
    // DUPE: preludeinsertion
    // TODO: Currently we do this here and also in the tests. It would be better
    // to have one place where we create all this required state for use in each
    // place.
    let _ = modules.insert(
        PRELUDE_MODULE_NAME.into(),
        crate::type_::build_prelude(&ids),
    );
    let mut direct_dependencies = std::collections::HashMap::from_iter(vec![]);
    if let Some((dep_package, dep_name, dep_src)) = dep {
        let mut dep_config = PackageConfig::default();
        dep_config.name = dep_package.into();
        let parsed = crate::parse::parse_module(
            Utf8PathBuf::from("test/path"),
            dep_src,
            &WarningEmitter::null(),
        )
        .expect("dep syntax error");
        let mut ast = parsed.module;
        ast.name = dep_name.into();
        let line_numbers = LineNumbers::new(dep_src);

        let dep = crate::analyse::ModuleAnalyzerConstructor::<()> {
            target: Target::Erlang,
            ids: &ids,
            origin: Origin::Src,
            importable_modules: &modules,
            warnings: &TypeWarningEmitter::null(),
            direct_dependencies: &std::collections::HashMap::new(),
            target_support: TargetSupport::NotEnforced,
            package_config: &dep_config,
        }
        .infer_module(ast, line_numbers, "".into())
        .expect("should successfully infer dep Erlang");
        let _ = modules.insert(dep_name.into(), dep.type_info);
        let _ = direct_dependencies.insert(dep_package.into(), ());
    }
    let path = Utf8PathBuf::from(src_path);
    let parsed = crate::parse::parse_module(path.clone(), src, &WarningEmitter::null())
        .expect("syntax error");
    let mut config = PackageConfig::default();
    config.name = "thepackage".into();
    let mut ast = parsed.module;
    ast.name = "my/mod".into();
    let line_numbers = LineNumbers::new(src);
    let ast = crate::analyse::ModuleAnalyzerConstructor::<()> {
        target: Target::Erlang,
        ids: &ids,
        origin: Origin::Src,
        importable_modules: &modules,
        warnings: &TypeWarningEmitter::null(),
        direct_dependencies: &direct_dependencies,
        target_support: TargetSupport::NotEnforced,
        package_config: &config,
    }
    .infer_module(ast, line_numbers, path)
    .expect("should successfully infer root Erlang");
    let line_numbers = LineNumbers::new(src);
    module(&ast, &line_numbers).unwrap()
}

#[macro_export]
macro_rules! assert_erl {
    (($dep_package:expr, $dep_name:expr, $dep_src:expr), $src:expr $(,)?) => {{
        let compiled = $crate::erlang::tests::compile_test_project(
            $src,
            "/root/project/test/my/mod.rakun",
            Some(($dep_package, $dep_name, $dep_src)),
        );
        let output = format!(
            "----- SOURCE CODE\n{}\n\n----- COMPILED ERLANG\n{}",
            $src, compiled
        );
        insta::assert_snapshot!(insta::internals::AutoName, output, $src);
    }};

    ($src:expr $(,)?) => {{
        let compiled = $crate::erlang::tests::compile_test_project(
            $src,
            "/root/project/test/my/mod.rakun",
            None,
        );
        let output = format!(
            "----- SOURCE CODE\n{}\n\n----- COMPILED ERLANG\n{}",
            $src, compiled
        );
        insta::assert_snapshot!(insta::internals::AutoName, output, $src);
    }};
}

#[test]
fn integration_test() {
    assert_erl!(
        r#"pub fn go() {
let x = #(100000000000000000, #(2000000000, 3000000000000, 40000000000), 50000, 6000000000)
  x
}"#
    );
}

#[test]
fn integration_test0_1() {
    assert_erl!(
        r#"pub fn go() {
  let y = 1
  let y = 2
  y
}"#
    );
}

#[test]
fn integration_test0_2() {
    // hex, octal, and binary literals
    assert_erl!(
        r#"pub fn go() {
    let fifteen = 0xF
    let nine = 0o11
    let ten = 0b1010
  fifteen
}"#
    );
}

#[test]
fn integration_test0_3() {
    assert_erl!(
        r#"pub fn go() {
  let y = 1
  let y = 2
  y
}"#
    );
}

#[test]
fn integration_test1() {
    assert_erl!(r#"pub fn t() { True }"#);
}

#[test]
fn integration_test1_1() {
    assert_erl!(
        r#"pub record Money { Pound(Int) }
                    fn pound(x) { Pound(x) }"#
    );
}

#[test]
fn integration_test1_2() {
    assert_erl!(r#"pub fn loop() { loop() }"#);
}

#[test]
fn integration_test1_4() {
    assert_erl!(
        r#"fn inc(x) { x + 1 }
                    pub fn go() { 1 |> inc |> inc |> inc }"#
    );
}

#[test]
fn integration_test1_5() {
    assert_erl!(
        r#"fn add(x, y) { x + y }
                    pub fn go() { 1 |> add(_, 1) |> add(2, _) |> add(_, 3) }"#
    );
}

#[test]
fn integration_test1_6() {
    assert_erl!(
        r#"pub fn and(x, y) { x && y }
pub fn or(x, y) { x || y }
pub fn remainder(x, y) { x % y }
pub fn fdiv(x, y) { x /. y }
            "#
    );
}

#[test]
fn integration_test2() {
    assert_erl!(
        r#"pub fn second(list) { case list { [x, y] -> y z -> 1 } }
pub fn tail(list) { case list { [x, ..xs] -> xs z -> list } }
            "#
    );
}

#[test]
fn integration_test5() {
    assert_erl!(
        "pub fn tail(list) {
  case list {
    [x, ..] -> x
    _ -> 0
  }
}"
    );
}

#[test]
fn integration_test6() {
    assert_erl!(r#"pub fn x() { let x = 1 let x = x + 1 x }"#);
}

#[test]
fn integration_test8() {
    // Translation of Float-specific BinOp into variable-type Erlang term comparison.
    assert_erl!(r#"pub fn x() { 1. <. 2.3 }"#);
}

#[test]
fn integration_test9() {
    // Custom type creation
    assert_erl!(
        r#"pub record Pair<x, y> { Pair(x: x, y: y) } pub fn x() { Pair(1, 2) Pair(3., 4.) }"#
    );
}

#[test]
fn integration_test10() {
    assert_erl!(r#"record Null { Null } fn x() { Null }"#);
}

#[test]
fn integration_test3() {
    assert_erl!(
        r#"record Point { Point(x: Int, y: Int) }
                fn y() { fn() { Point }()(4, 6) }"#
    );
}

#[test]
fn integration_test11() {
    assert_erl!(
        r#"record Point { Point(x: Int, y: Int) }
                fn x() { Point(x: 4, y: 6) Point(y: 1, x: 9) }"#
    );
}

#[test]
fn integration_test12() {
    assert_erl!(r#"record Point { Point(x: Int, y: Int) } fn x(y) { let Point(a, b) = y a }"#);
}
//https://github.com/rakun-lang/rakun/issues/1106

#[test]
fn integration_test13() {
    assert_erl!(
        r#"pub record State{ Start(Int) End(Int) }
            pub fn build(constructor : fn(Int) -> a) -> a { constructor(1) }
            pub fn main() { build(End) }"#
    );
}

#[test]
fn integration_test16() {
    assert_erl!(
        r#"fn go(x xx, y yy) { xx }
pub fn x() { go(x: 1, y: 2) go(y: 3, x: 4) }"#
    );
}

#[test]
fn integration_test17() {
    
    assert_erl!(
        r#"
record User { User(id: Int, name: String, age: Int) }
fn create_user(user_id) { User(age: 22, id: user_id, name: "") }
                    "#
    );
}

#[test]
fn integration_test18() {
    assert_erl!(r#"pub fn run() { case 1, 2 { a, b -> a } }"#);
}

#[test]
fn integration_test19() {
    assert_erl!(
        r#"record X { X(x: Int, y: Float) }
                    fn x() { X(x: 1, y: 2.) X(y: 3., x: 4) }"#
    );
}

#[test]
fn integration_test20() {
    assert_erl!(
        r#"
pub fn go(a) {
  let a = a + 1
  a
}

                    "#
    );
}

#[test]
fn integration_test21() {
    assert_erl!(
        r#"
pub fn go(a) {
  let a = 1
  a
}

                    "#
    );
}

#[test]
fn integration_test22() {
    
    assert_erl!(
        r#"
pub fn factory(f, i) {
  f(i)
}

pub record Box {
  Box(i: Int)
}

pub fn main() {
  factory(Box, 0)
}
"#
    );
}

#[test]
fn integration_test23() {
    
    assert_erl!(
        r#"
pub fn main(args) {
  case args {
    _ -> {
      let a = 1
      a
    }
  }
  let a = 2
  a
}
"#
    );
}

#[test]
fn binop_parens() {
    // Parentheses are added for binop subexpressions
    assert_erl!(
        r#"
pub fn main() {
    let a = 2 * {3 + 1} / 2
    let b = 5 + 3 / 3 * 2 - 6 * 4
    b
}
"#
    );
}

#[test]
fn field_access_function_call() {
    // Parentheses are added when calling functions returned by record access
    assert_erl!(
        r#"
record FnBox {
  FnBox(f: fn(Int) -> Int)
}
fn main() {
    let b = FnBox(f: fn(x) { x })
    b.f(5)
}
"#
    );
}

#[test]
fn field_access_function_call1() {
    // Parentheses are added when calling functions returned by tuple access
    assert_erl!(
        r#"
pub fn main() {
    let t = #(fn(x) { x })

    t.0(5)
}
"#
    );
}


#[test]
fn block_assignment() {
    assert_erl!(
        r#"
pub fn main() {
  let x = {
    1
    2
  }
  x
}
"#
    );
}

#[test]
fn recursive_type() {
    // TODO: we should be able to generalise `id` and we should be
    // able to handle recursive types. Either of these type features
    // would make this module type check OK.
    assert_erl!(
        r#"
fn id(x) {
  x
}

pub fn main() {
  id(id)
}
"#
    );
}

#[test]
fn tuple_access_in_guard() {
    assert_erl!(
        r#"
pub fn main() {
    let key = 10
    let x = [#(10, 2), #(1, 2)]
    case x {
        [first, ..rest] if first.0 == key -> "ok"
        _ -> "ko"
    }
}
"#
    );
}

#[test]
fn variable_name_underscores_preserved() {
    assert_erl!(
        "pub fn a(name_: String) -> String {
    let name__ = name_
    let name = name__
    let one_1 = 1
    let one1 = one_1
    name
}"
    );
}

#[test]
fn allowed_string_escapes() {
    assert_erl!(r#"pub fn a() { "\n" "\r" "\t" "\\" "\"" "\\^" }"#);
}


#[test]
fn keyword_constructors() {
    assert_erl!("pub record X { Div }");
}


#[test]
fn keyword_constructors1() {
    assert_erl!("pub record X { Fun(Int) }");
}

#[test]
fn discard_in_assert() {
    assert_erl!(
        "pub fn x(y) {
  let assert Ok(_) = y
  1
}"
    );
}


#[test]
fn operator_pipe_right_hand_side() {
    assert_erl!(
        "fn id(x) {
  x
}

pub fn bool_expr(x, y) {
  y || x |> id
}"
    );
}

#[test]
fn negation() {
    assert_erl!(
        "pub fn negate(x) {
  !x
}"
    )
}

#[test]
fn negation_block() {
    assert_erl!(
        "pub fn negate(x) {
  !{
    123
    x
  }
}"
    )
}


#[test]
fn tail_maybe_expr_block() {
    assert_erl!(
        "pub fn a() {
  let fake_tap = fn(x) { x }
  let b = [99]
  [
    1,
    2,
    ..b
    |> fake_tap
  ]
}
"
    );
}


#[test]
fn guard_variable_rewriting() {
    assert_erl!(
        "pub fn main() {
  case 1.0 {
    a if a <. 0.0 -> {
      let a = a
      a
    }
    _ -> 0.0
  }
}
"
    )
}


#[test]
fn function_argument_shadowing() {
    assert_erl!(
        "pub fn main(a) {
  Box
}

pub record Box {
  Box(Int)
}
"
    )
}


#[test]
fn dynamic() {
    assert_erl!("pub type Dynamic")
}


#[test]
fn inline_const_pattern_option() {
    assert_erl!(
        "pub fn main() {
            let fifteen = 15
            let x = <<5:size(sixteen)>>
            case x {
              <<5:size(sixteen)>> -> <<5:size(sixteen)>>
              <<6:size(fifteen)>> -> <<5:size(fifteen)>>
              _ -> <<>>
            }
          }

          pub const sixteen = 16"
    )
}


#[test]
fn positive_zero() {
    assert_erl!(
        "
pub fn main() {
  0.0
}
"
    )
}


#[test]
fn scientific_notation() {
    assert_erl!(
        "
pub fn main() {
  1.0e6
  1.e6
}
"
    );
}


#[test]
fn type_named_else() {
    assert_erl!(
        "
pub record Else {
  Else
}

pub fn main() {
  Else
}
"
    );
}


#[test]
fn type_named_module_info() {
    assert_erl!(
        "
pub record ModuleInfo {
    ModuleInfo
}

pub fn main() {
    ModuleInfo
}
"
    );
}


#[test]
fn function_named_module_info() {
    assert_erl!(
        "
pub fn module_info() {
    1
}

pub fn main() {
    module_info()
}
"
    );
}


#[test]
fn function_named_module_info_imported() {
    assert_erl!(
        (
            "some_module",
            "some_module",
            "
pub fn module_info() {
    1
}
            "
        ),
        "
import some_module

pub fn main() {
    some_module.module_info()
}
"
    );
}


#[test]
fn function_named_module_info_imported_qualified() {
    assert_erl!(
        (
            "some_module",
            "some_module",
            "
pub fn module_info() {
    1
}
            "
        ),
        "
import some_module.{module_info}

pub fn main() {
    module_info()
}
"
    );
}


#[test]
fn constant_named_module_info() {
    assert_erl!(
        "
pub const module_info = 1

pub fn main() {
    module_info
}
"
    );
}


#[test]
fn constant_named_module_info_imported() {
    assert_erl!(
        (
            "some_module",
            "some_module",
            "
pub const module_info = 1
            "
        ),
        "
import some_module

pub fn main() {
    some_module.module_info
}
"
    );
}


#[test]
fn constant_named_module_info_imported_qualified() {
    assert_erl!(
        (
            "some_module",
            "some_module",
            "
pub const module_info = 1
            "
        ),
        "
import some_module.{module_info}

pub fn main() {
    module_info
}
"
    );
}


#[test]
fn constant_named_module_info_with_function_inside() {
    assert_erl!(
        "
pub fn function() {
    1
}

pub const module_info = function

pub fn main() {
    module_info()
}
"
    );
}


#[test]
fn constant_named_module_info_with_function_inside_imported() {
    assert_erl!(
        (
            "some_module",
            "some_module",
            "
pub fn function() {
    1
}

pub const module_info = function
"
        ),
        "
import some_module

pub fn main() {
    some_module.module_info()
}
"
    );
}


#[test]
fn constant_named_module_info_with_function_inside_imported_qualified() {
    assert_erl!(
        (
            "some_module",
            "some_module",
            "
pub fn function() {
    1
}

pub const module_info = function
"
        ),
        "
import some_module.{module_info}

pub fn main() {
    module_info()
}
"
    );
}


#[test]
fn function_named_module_info_in_constant() {
    assert_erl!(
        "
pub fn module_info() {
    1
}

pub const constant = module_info

pub fn main() {
    constant()
}
"
    );
}


#[test]
fn function_named_module_info_in_constant_imported() {
    assert_erl!(
        (
            "some_module",
            "some_module",
            "
pub fn module_info() {
    1
}

pub const constant = module_info
            "
        ),
        "
import some_module

pub fn main() {
    some_module.constant()
}
"
    );
}


#[test]
fn function_named_module_info_in_constant_imported_qualified() {
    assert_erl!(
        (
            "some_module",
            "some_module",
            "
pub fn module_info() {
    1
}

pub const constant = module_info
            "
        ),
        "
import some_module.{constant}

pub fn main() {
    constant()
}
"
    );
}


#[test]
fn windows_file_escaping_bug() {
    let src = "pub fn main() { Nil }";
    let path = "C:\\root\\project\\test\\my\\mod.rakun";
    let output = compile_test_project(src, path, None);
    insta::assert_snapshot!(insta::internals::AutoName, output, src);
}


#[test]
fn bit_pattern_shadowing() {
    assert_erl!(
        "
pub fn main() {
  let code = <<\"hello world\":utf8>>
  let pre = 1
  case code {
    <<pre:bytes-size(pre), _:bytes>> -> pre
    _ -> panic
  }
}        "
    )
}
