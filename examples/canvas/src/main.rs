use silkenweb::{elements::html::canvas, mount, prelude::Element};
use web_sys::{wasm_bindgen::JsCast, CanvasRenderingContext2d};

fn main() {
    mount(
        "app",
        canvas().effect(|c| {
            let ctx = c
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();
            ctx.move_to(0.0, 0.0);
            ctx.line_to(200.0, 100.0);
            ctx.stroke();
        }),
    );
}
