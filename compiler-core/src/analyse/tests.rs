use super::*;

#[test]
fn module_name_validation() {
    assert!(validate_module_name(&"dream".into()).is_ok());

    assert!(validate_module_name(&"rakun".into()).is_err());

    assert!(validate_module_name(&"rakun/ok".into()).is_ok());

    assert!(validate_module_name(&"ok/rakun".into()).is_ok());

    assert!(validate_module_name(&"type".into()).is_err());

    assert!(validate_module_name(&"pub".into()).is_err());

    assert!(validate_module_name(&"ok/type".into()).is_err());

    assert!(validate_module_name(&"ok/pub".into()).is_err());
}
