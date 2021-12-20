//! A minimal example
use std::iter;

use num_traits::ToPrimitive;
use silkenweb::{
    animation::infinite_animation, mount, signal::ReadSignal, tag_in_namespace, Builder, Element,
};

const WIDTH: f32 = 600.0;
const HEIGHT: f32 = 300.0;

fn path(time: &ReadSignal<f64>, humps: usize, speed: f64) -> Element {
    let path = time.map(move |time| {
        let multiplier = (*time / speed).sin().to_f32().unwrap();
        let control_point = 150.0 * multiplier + 150.0;
        let half_height = HEIGHT / 2.0;
        let hump_width = WIDTH / humps.to_f32().unwrap();

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

    tag_in_namespace("http://www.w3.org/2000/svg", "path")
        .attribute("d", path)
        .attribute("stroke", "black")
        .attribute("fill", "transparent")
        .build()
}

fn main() {
    let ts = infinite_animation();
    let mut svg = tag_in_namespace("http://www.w3.org/2000/svg", "svg")
        .attribute("width", WIDTH)
        .attribute("height", HEIGHT);

    for i in 2..6 {
        svg = svg.child(path(&ts, i, 150.0 * i.to_f64().unwrap()));
    }

    mount("app", svg);
}
