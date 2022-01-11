//! A minimal example
use std::iter;

use futures_signals::signal::{Broadcaster, Signal, SignalExt};
use num_traits::ToPrimitive;
use silkenweb::{
    animation::infinite_animation,
    dom::{element::ElementBuilder, mount},
    elements::{svg, ParentBuilder},
};
use wasm_bindgen::UnwrapThrowExt;

const WIDTH: f32 = 600.0;
const HEIGHT: f32 = 300.0;

fn path(time: impl Signal<Item = f64> + 'static, humps: usize, speed: f64) -> svg::Path {
    let path = time.map(move |time| {
        let multiplier = (time / speed).sin().to_f32().unwrap_throw();
        let control_point = 150.0 * multiplier + 150.0;
        let half_height = HEIGHT / 2.0;
        let hump_width = WIDTH / humps.to_f32().unwrap_throw();

        let initial_path = format!(
            "M 0,{} Q {},{} {},{}",
            half_height,
            hump_width / 2.0,
            control_point,
            hump_width,
            half_height,
        );

        assert!(humps >= 1);

        iter::repeat(format!(" t {},0", hump_width))
            .take(humps - 1)
            .fold(initial_path, |path, hump| path + &hump)
    });

    svg::path()
        .d_signal(path)
        .stroke("black")
        .fill("transparent")
        .build()
}

fn main() {
    let ts = Broadcaster::new(infinite_animation());
    let mut svg = svg::svg()
        .width(&WIDTH.to_string())
        .height(&HEIGHT.to_string());

    for i in 2..6 {
        svg = svg.child(path(ts.signal(), i, 150.0 * i.to_f64().unwrap_throw()));
    }

    mount("app", svg);
}
