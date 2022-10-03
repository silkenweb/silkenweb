use std::iter;

use futures_signals::signal::{Broadcaster, Signal, SignalExt};
use num_traits::ToPrimitive;
use silkenweb::{
    animation::infinite_animation,
    elements::svg::{
        self,
        attributes::Presentation,
        path::{
            Data,
            Offset::{Abs, Rel},
        },
    },
    mount,
    node::element::{ElementBuilder, ParentBuilder},
};
use wasm_bindgen::UnwrapThrowExt;

const WIDTH: f64 = 600.0;
const HEIGHT: f64 = 300.0;

fn path(time: impl Signal<Item = f64> + 'static, humps: usize, speed: f64) -> svg::Path {
    let path = time.map(move |time| {
        let multiplier = (time / speed).sin();
        let control_point = 150.0 * multiplier + 150.0;
        let half_height = HEIGHT / 2.0;
        let hump_width = WIDTH / humps.to_f64().unwrap_throw();

        assert!(humps >= 1);

        Data::new()
            .move_to(Abs, 0.0, half_height)
            .quadradic_bezier_curves(
                Abs,
                [(hump_width / 2.0, control_point, hump_width, half_height)],
            )
            .smooth_quadradic_bezier_curves(Rel, iter::repeat((hump_width, 0.0)).take(humps - 1))
    });

    svg::path()
        .d_signal(path)
        .stroke("black")
        .fill("transparent")
        .build()
}

fn main() {
    let ts = Broadcaster::new(infinite_animation());
    let mut svg = svg::svg().width(WIDTH).height(HEIGHT);

    for i in 2..6 {
        svg = svg.child(path(ts.signal(), i, 150.0 * i.to_f64().unwrap_throw()));
    }

    mount("app", svg);
}
