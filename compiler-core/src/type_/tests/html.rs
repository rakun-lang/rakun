use crate::assert_infer_with_module;

#[test]
fn html_div() {
    assert_infer_with_module!(
        (
            "rakun/rkx",
            r#"
pub record Element {
    Element
}
pub fn html_text(value:e) {
    Element
}
pub fn html_interpolation(value:e) {
    Element
}
pub fn component_init(value:e) {
    Element
}
        
    "#
        ),
        r#"
import rakun/rkx

pub record DivProps {
    DivProps(class: String)
}      
pub fn div(props: DivProps) {
    rkx.Element
}

pub fn main() {
  <div class="56464"/>
}
"#,
        vec![
            ("DivProps", "fn(String) -> DivProps"),
            ("div", "fn(DivProps) -> Element"),
            ("main", "fn() -> Element")
        ]
    );
}
