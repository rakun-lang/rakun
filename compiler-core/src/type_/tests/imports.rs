use crate::{assert_infer_with_module, assert_module_error, assert_with_module_error};


#[test]
fn import_value_with_same_name_as_imported_module() {
    assert_infer_with_module!(
        ("other", "pub const other = 1"),
        "
import other.{other}
pub const a = other
",
        vec![("a", "Int")],
    );
}

#[test]
fn imported_constant_record() {
    assert_infer_with_module!(
        ("one/two", "pub record Thing { Thing(Int) }"),
        "
import one/two

pub const a = two.Thing(1)
",
        vec![("a", "Thing")],
    );
}

#[test]
fn using_private_constructo() {
    assert_with_module_error!(
        ("one", "record Two { Two }"),
        "import one

pub fn main() {
  one.Two
}",
    );
}

#[test]
fn using_private_constructor_pattern() {
    assert_with_module_error!(
        ("one", "record Two { Two }"),
        "import one

pub fn main(x) {
  let one.Two = x
}",
    );
}

#[test]
fn using_opaque_constructo() {
    assert_with_module_error!(
        ("one", "pub opaque record Two { Two }"),
        "import one

pub fn main() {
  one.Two
}",
    );
}

#[test]
fn using_private_function() {
    assert_with_module_error!(
        ("one", "fn two() { 2 }"),
        "import one

pub fn main() {
  one.two
}",
    );
}

#[test]
fn using_private_type_alias() {
    assert_with_module_error!(
        ("one", "type X = Int"),
        "import one

pub fn main() {
  one.X
}",
    );
}

#[test]
fn using_private_unqualified_type_alias() {
    assert_with_module_error!(
        ("one", "type X = Int"),
        "import one.{X}

pub fn main() {
  X
}",
    );
}

#[test]
fn using_private_external_type() {
    assert_with_module_error!(
        ("one", "type X"),
        "import one

pub fn main() {
  one.X
}",
    );
}

#[test]
fn using_private_unqualified_external_type() {
    assert_with_module_error!(
        ("one", "type X"),
        "import one.{X}

pub fn main() {
  X
}",
    );
}

#[test]
fn using_private_custom_type() {
    assert_with_module_error!(
        ("one", "record X { Y }"),
        "import one

pub fn main() {
  one.X
}",
    );
}

#[test]
fn using_private_unqualified_custom_type() {
    assert_with_module_error!(
        ("one", "record X { Y }"),
        "import one.{X}

pub fn main() {
  X
}",
    );
}

#[test]
fn unqualified_using_private_constructo() {
    assert_with_module_error!(
        ("one", "record Two { Two }"),
        "import one.{Two}

pub fn main() {
  Two
}",
    );
}

#[test]
fn unqualified_using_private_constructor_pattern() {
    assert_with_module_error!(
        ("one", "record Two { Two }"),
        "import one.{Two}

pub fn main(x) {
  let Two = x
}",
    );
}

#[test]
fn unqualified_using_opaque_constructo() {
    assert_with_module_error!(
        ("one", "pub opaque record Two { Two }"),
        "import one.{Two}

pub fn main() {
  Two
}",
    );
}

#[test]
fn unqualified_using_private_function() {
    assert_with_module_error!(
        ("one", "fn two() { 2 }"),
        "import one.{two}

pub fn main() {
  two
}",
    );
}

#[test]
fn import_type() {
    assert_infer_with_module!(
        ("one", "pub type One = Int"),
        "import one.{type One}

pub fn main() -> One {
  todo
}
",
        vec![("main", "fn() -> Int")],
    );
}

#[test]
fn import_type_duplicate() {
    assert_with_module_error!(
        ("one", "pub type One = Int"),
        "import one.{One, type One}

pub fn main() -> One {
  todo
}
",
    );
}

#[test]
fn import_type_duplicate_with_as() {
    assert_with_module_error!(
        ("one", "pub type One = Int"),
        "import one.{type One as MyOne, type One as MyOne}

pub type X = One
",
    );
}

#[test]
fn import_type_duplicate_with_as_multiline() {
    assert_with_module_error!(
        ("one", "pub type One = Int"),
        "import one.{
          type One as MyOne,
          type One as MyOne
        }

pub type X = One
",
    );
}


#[test]
fn deprecated_type_import_conflict() {
    assert_infer_with_module!(
        ("one", "pub record X { X }"),
        "import one.{X, type X}",
        vec![]
    );
}

#[test]
fn aliased_unqualified_type_and_value() {
    assert_infer_with_module!(
        ("one", "pub record X { X }"),
        "import one.{X as XX, type X as XX}",
        vec![]
    );
}

#[test]
fn deprecated_type_import_conflict_two_modules() {
    assert_infer_with_module!(
        ("one", "pub record X { X }"),
        ("two", "pub record X { X }"),
        "
        import one.{type X as Y}
        import two.{X}
        ",
        vec![]
    );
}

#[test]
fn imported_constructor_instead_of_type() {
    assert_with_module_error!(
        ("module", "pub record Wibble { Wibble }"),
        "import module.{Wibble}

pub fn main(x: Wibble) {
  todo
}",
    );
}

#[test]
fn import_errors_do_not_block_analysis() {
    // An error in an import doesn't stop the rest of the module being analysed
    assert_module_error!(
        "import unknown_module

pub fn main() {
  1 + Nil
}"
    );
}

#[test]
fn unqualified_import_errors_do_not_block_later_unqualified() {
    assert_module_error!(
        "import rakun.{Unknown, type Int as Integer}

pub fn main() -> Integer {
  Nil
}"
    );
}

#[test]
fn module_alias_used_as_a_name() {
    assert_with_module_error!(
        ("one/two", ""),
        "
import one/two

pub fn main() {
  two
}
"
    );
}
