use crate::assert_js;

#[test]
fn test_html_elements() {
    assert_js!(
        (
            "package",
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
    DivProps(class: String, children: List<rkx.Element>)
}

pub record HeaderProps {
    HeaderProps(class: String, children: List<rkx.Element>)
}

pub record ParagraphProps {
    ParagraphProps(class: String, children: List<rkx.Element>)
}

pub record H1Props {
    H1Props(class: String, children: List<rkx.Element>)
}

pub fn div(props: DivProps) {
     rkx.Element
}

pub fn header(props: HeaderProps) {
     rkx.Element
}

pub fn p(props: ParagraphProps) {
     rkx.Element
}

pub fn h1(props: H1Props) {
     rkx.Element
}

pub fn h2(props: H1Props) {
    rkx.Element
}
    

pub fn main() {
    <div class="container">
        <h1 class="title">Título do Conteúdo</h1>
        <p class="description">Descrição breve sobre o conteúdo apresentado.</p>
        <div class="nested-container">
            <h2 class="subtitle">Subtítulo</h2>
            <p class="nested-content">Informação adicional sobre o conteúdo.</p>
        </div>
    </div>
}
"#
    );
}

#[test]
fn test_html_elements_module() {
    assert_js!(
        (
            "package",
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
    
pub record DivProps {
    DivProps(class: String, children: List<Element>)
}

pub record HeaderProps {
    HeaderProps(class: String, children: List<Element>)
}

pub record ParagraphProps {
    ParagraphProps(class: String, children: List<Element>)
}

pub record H1Props {
    H1Props(class: String, children: List<Element>)
}

pub fn div(props: DivProps) {
    Element
}

pub fn header(props: HeaderProps) {
     Element
}

pub fn p(props: ParagraphProps) {
    Element
}

pub fn h1(props: H1Props) {
     Element
}

pub fn h2(props: H1Props) {
    Element
}        
"#
        ),
        r#"
import rakun/rkx
    

pub fn main() {
    <rkx.div class="container">
        <rkx.h1 class="title">Título do Conteúdo</rkx.h1>
        <rkx.p class="description">Descrição breve sobre o conteúdo apresentado.</rkx.p>
        <rkx.div class="nested-container">
            <rkx.h2 class="subtitle">Subtítulo</rkx.h2>
            <rkx.p class="nested-content">Informação adicional sobre o conteúdo.</rkx.p>
        </rkx.div>
    </rkx.div>
}
"#
    );
}
