use crate::{assert_module_error, assert_module_infer, assert_warning, assert_with_module_error};


#[test]
fn generic_phantom() {
    assert_module_infer!(
        r#"
pub record Test<a> {
  MakeTest(field: Test<Int>)
}
"#,
        vec![("MakeTest", "fn(Test<Int>) -> Test<a>")]
    );
}

#[test]
fn deprecated_type() {
    assert_warning!(
        r#"
@deprecated("Dont use this!")
pub record Cat {
  Cat(name: String, cuteness: Int)
}

pub fn name() -> String {
  let c = Cat("Numi", 20)
  c.name
}
        "#
    );
}

#[test]
fn fault_tolerance() {
    // An error in a custom type does not stop analysis
    assert_module_error!(
        r#"
pub record Cat {
  Cat(UnknownType)
}

pub type Kitten = AnotherUnknownType
        "#
    );
}

#[test]
fn duplicate_variable_error_does_not_stop_analysis() {
    // Both these aliases have errors! We do not stop on the first one.
    assert_module_error!(
        r#"
record Two<a, a> {
  Two(a, a)
}

record Three<a, a> {
  Three
}
"#
    );
}

#[test]
fn conflict_with_import() {
    // We cannot declare a type with the same name as an imported type
    assert_with_module_error!(
        ("wibble", "pub record A { B }"),
        "import wibble.{type A} record A { C }",
    );
}
