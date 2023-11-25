use silkenweb::{dom::Wet, elements::html::div, node::Node, prelude::ParentElement};
use silkenweb_inline_svg::{inline_svg, svg_dir, svg_file};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

/// Check we can create a `Node` where `Dom` != `DefaultDom`.
#[test]
fn inline_compile() {
    let _ = || -> Node<Wet> { inline_svg!(r#"<svg></svg>"#) };
}

#[wasm_bindgen_test]
#[test]
fn inline() {
    inline_svg_test(inline_svg!(
        r#"<svg><rect height="100" style="fill:rgb(0,255,0)" width="100"></rect>Inline SVG</svg>"#
    ));
}

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
