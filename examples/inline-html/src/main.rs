use silkenweb::{elements::html::div, mount, node::element::ParentElement};
use silkenweb_inline_html::{html_dir, html_file, inline_html};

mod svg {
    super::html_dir!("svg");
}

html_file!("svg/test-image.svg");

fn main() {
    let snippet = inline_html!(r#"<p>This is an HTML snippet</p>"#);
    mount(
        "app",
        div().children([snippet, test_image(), svg::test_image()]),
    );
}
