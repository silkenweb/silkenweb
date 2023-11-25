use silkenweb::{elements::html::div, node::Node, prelude::ParentElement};
use silkenweb_inline_svg::{svg_dir, svg_file};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
#[test]
fn file() {
    svg_file!("tests/svg/image.svg");
    inline_svg_test(image());
}

#[wasm_bindgen_test]
#[test]
fn dir() {
    svg_dir!("tests/svg");
    inline_svg_test(image());
}

fn inline_svg_test(image: Node) {
    let image_contents = include_str!("svg/image.svg");
    let expected = format!(r#"<div>{image_contents}</div>"#);
    assert_eq!(div().children([image]).freeze().to_string(), expected);
}
