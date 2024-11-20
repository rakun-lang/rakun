use crate::{assert_format, assert_format_rewrite};

#[test]
fn html_nested_elements() {
    assert_format!(
        r#"pub fn main() {
  let a = "nested"

  <div>
    <div>
      <span>
        { a }
      </span>
      <p>
        Some text
      </p>
    </div>
  </div>
}
"#
    );
}

#[test]
fn record_inside_const_list() {
    assert_format_rewrite!(
        r#"const commands = [
  Command(
    "dev",
    "Start a file watcher that automatically re-compiles your app on all file changes.",
  ), Command("help", "Show this help text."),
]
"#,
        r#"const commands = [
  Command(
    "dev",
    "Start a file watcher that automatically re-compiles your app on all file changes.",
  ),
  Command("help", "Show this help text."),
]
"#
    );
}
#[test]
fn html_conditional_rendering() {
    assert_format!(
        r#"pub fn main() {
  let show = True
  let a = "conditional"

  <div>
    {
      case show {
        True -> <span>
            { a }
          </span>
        False -> <span>
            Not shown
          </span>
      }
    }
  </div>
}
"#
    );
}

#[test]
fn html_dynamic_attributes() {
    assert_format!(
        r#"pub fn main() {
  let id = "unique-id"

  <div id={ id } class="container">
    Content here
  </div>
}
"#
    );
}

#[test]
fn html_self_closing_with_attributes() {
    assert_format!(
        r#"pub fn main() {
  <img src="image.png" alt="Image description"/>
  <input type="text" placeholder="Enter text here"/>
}
"#
    );
}

#[test]
fn html_multiple_sibling_elements() {
    assert_format!(
        r#"pub fn main() {
  <div>
    <div>
      First
    </div>
    <div>
      Second
    </div>
    <div>
      Third
    </div>
  </div>
}
"#
    );
}

#[test]
fn html_custom_component() {
    assert_format!(
        r#"pub fn main() {
  let content = "Hello from my component"

  <c.my_component>
    { content }
  </c.my_component>
}
"#
    );
}

#[test]
fn html_nested_custom_component() {
    assert_format!(
        r#"pub fn main() {
  <c.outer_component>
    <c.inner_component>
      Nested content
    </c.inner_component>
  </c.outer_component>
}
"#
    );
}

#[test]
fn html_custom_component_with_props() {
    assert_format!(
        r#"pub fn main() {
  let title = "My Title"
  let description = "This is a description."

  <c.card title={ title } description={ description }>
    <p>
      This is the card content.
    </p>
  </c.card>
}
"#
    );
}

#[test]
fn html_custom_component_with_conditional() {
    assert_format!(
        r#"pub fn main() {
  let show_details = true

  <c.toggle_component>
    {
      case show_details {
        True -> <p>
            Details are shown!
          </p>
        False -> <p>
            No details to show.
          </p>
      }
    }
  </c.toggle_component>
}
"#
    );
}

#[test]
fn html_custom_component_with_children() {
    assert_format!(
        r#"pub fn main() {
  <c.parent_component>
    <>
      <c.child_component>
        Child content goes here.
      </c.child_component>
      <c.child_component>
        Another child content.
      </c.child_component>
    </>
  </c.parent_component>
}
"#
    );
}

#[test]
fn html_custom_component_with_attributes() {
    assert_format!(
        r#"pub fn main() {
  let is_active = true

  <c.button is_active={ is_active } class="btn-primary">
    Click Me
  </c.button>
}
"#
    );
}

#[test]
fn html_fragment() {
    assert_format!(
        r#"pub fn main() {
  let is_active = true

  <>
    hey
    <div>
    </div>
  </>
}
"#
    );
}

#[test]
fn html_nested_elements_with_multiple_levels() {
    assert_format!(
        r#"pub fn main() {
  let a = "nested"

  <div>
    <div>
      <span>
        { a }
      </span>
      <p>
        Some text
      </p>
      <ul>
        <li>
          Item 1
        </li>
        <li>
          Item 2
        </li>
        <li>
          Item 3
        </li>
      </ul>
    </div>
  </div>
}
"#
    );
}

#[test]
fn html_custom_component_with_nested_props() {
    assert_format!(
        r#"pub fn main() {
  let title = "My Title"
  let description = "This is a description."

  <c.card title={ title }>
    <c.card_content description={ description }>
      <p>
        This is the card content.
      </p>
    </c.card_content>
  </c.card>
}
"#
    );
}

#[test]
fn html_custom_component_with_event_handlers() {
    assert_format!(
        r#"pub fn main() {
  <c.button on_click={ handle_click }>
    Click Me
  </c.button>
}

fn handle_click() {
  todo
  // Handle button click event
}
"#
    );
}

#[test]
fn html_custom_component_with_inline_styles() {
    assert_format!(
        r#"pub fn main() {
  <c.button style="background-color: red; color: white;">
    Click Me
  </c.button>
}
"#
    );
}

#[test]
fn html_custom_component_with_nested_fragments() {
    assert_format!(
        r#"pub fn main() {
  <c.parent_component>
    <>
      <c.child_component>
        Child content goes here.
      </c.child_component>
      <c.child_component>
        Another child content.
      </c.child_component>
    </>
  </c.parent_component>
}
"#
    );
}

#[test]
fn html_custom_component_with_nested_custom_components() {
    assert_format!(
        r#"pub fn main() {
  <c.parent_component>
    <c.child_component>
      <c.nested_child_component>
        Nested child content goes here.
      </c.nested_child_component>
    </c.child_component>
    <c.child_component>
      Another child content.
    </c.child_component>
  </c.parent_component>
}
"#
    );
}

#[test]
fn html_custom_component_with_nested_custom_components_and_props() {
    assert_format!(
        r#"pub fn main() {
  let title = "My Title"
  let description = "This is a description."

  <c.parent_component>
    <c.child_component>
      <c.nested_card on_click={ handle_nested_click }>
        <c.nested_card_content>
          Nested card content goes here.
        </c.nested_card_content>
      </c.nested_card>
    </c.child_component>
    <c.child_component>
      Another child content.
    </c.child_component>
  </c.parent_component>
}
"#
    );
}

#[test]
fn html_custom_component_with_nested_custom_components_and_event_handlers() {
    assert_format!(
        r#"pub fn main() {
  <c.parent_component>
    <c.child_component>
      <c.nested_card on_click={ handle_nested_click }>
        <c.nested_card_content>
          Nested card content goes here.
        </c.nested_card_content>
      </c.nested_card>
    </c.child_component>
    <c.child_component>
      Another child content.
    </c.child_component>
  </c.parent_component>
}

fn handle_nested_click() {
  todo
  // Handle nested button click event
}
"#
    );
}

#[test]
fn html_custom_component_with_nested_custom_components_and_inline_styles() {
    assert_format!(
        r#"pub fn main() {
  <c.parent_component>
    <c.child_component>
      <c.nested_card style="background-color: red; color: white;">
        <c.nested_card_content>
          Nested card content goes here.
        </c.nested_card_content>
      </c.nested_card>
    </c.child_component>
    <c.child_component>
      Another child content.
    </c.child_component>
  </c.parent_component>
}
"#
    );
}

#[test]
fn html_custom_component_with_nested_custom_components_and_dynamic_class_names() {
    assert_format!(
        r#"pub fn main() {
  <c.parent_component>
    <c.child_component>
      <c.nested_card class={ nested_card_class }>
        <c.nested_card_content>
          Nested card content goes here.
        </c.nested_card_content>
      </c.nested_card>
    </c.child_component>
    <c.child_component>
      Another child content.
    </c.child_component>
  </c.parent_component>
}

fn nested_card_class() -> String {
  // Dynamically generate nested card class name
  "nested-card-class".to_string()
}
"#
    );
}

#[test]
fn html_custom_component_with_nested_custom_components_and_dynamic_styles() {
    assert_format!(
        r#"pub fn main() {
  <c.parent_component>
    <c.child_component>
      <c.nested_card style={ nested_card_style }>
        <c.nested_card_content>
          Nested card content goes here.
        </c.nested_card_content>
      </c.nested_card>
    </c.child_component>
  </c.parent_component>
}

fn nested_card_style() -> String {
  // Dynamically generate nested card style
  "background-color: red; color: white;".to_string()
}
"#
    );
}
