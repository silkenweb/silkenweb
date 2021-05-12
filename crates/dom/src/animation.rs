use silkenweb_reactive::signal::ReadSignal;

use crate::render::{animation_timestamp, request_render_updates};

pub fn infinite_animation() -> ReadSignal<f64> {
    // TODO: How do we base the timer to zero?
    animation_timestamp().map(|ts| {
        // Make sure we continue the animation even if it didn't change the DOM this
        // frame
        request_render_updates();
        *ts
    })
}
