use silkenweb::{
    dom::Wet,
    elements::html::div,
    node::{element::ParentElement, Node},
};
use silkenweb_inline_html::{html_dir, html_file, inline_html};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

/// Check we can create a `Node` where `Dom` != `DefaultDom`.
#[test]
fn inline_compile() {
    let _ = || -> Node<Wet> { inline_html!(r#"<svg></svg>"#) };
}

#[wasm_bindgen_test]
#[test]
fn inline() {
    inline_html_test(inline_html!(
        r#"<svg><rect height="100" style="fill:rgb(0,255,0)" width="100"></rect>Inline SVG</svg>"#
    ));
}

#[wasm_bindgen_test]
#[test]
fn file() {
    html_file!("tests/svg/image.svg");
    inline_html_test(image());
}

#[wasm_bindgen_test]
#[test]
fn dir() {
    html_dir!("tests/svg");
    inline_html_test(image());
}

fn inline_html_test(image: Node) {
    let image_contents = include_str!("svg/image.svg");
    let expected = format!(r#"<div>{image_contents}</div>"#);
    assert_eq!(div().children([image]).freeze().to_string(), expected);
}
