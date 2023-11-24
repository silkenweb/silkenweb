use silkenweb::{elements::html::div, mount, prelude::ParentElement};
use silkenweb_inline_svgs::{inline_svg, inline_svg_dir};

mod svg {
    super::inline_svg_dir!("svg");
}

inline_svg!("svg/test-image.svg");

fn main() {
    mount("app", div().children([test_image(), svg::test_image()]));
}
