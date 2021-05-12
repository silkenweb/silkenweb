use std::cell::Cell;

use silkenweb_reactive::signal::{ReadSignal, SignalReceiver};

use crate::render::{animation_timestamp, request_render_updates};

#[derive(Default)]
struct AnimationTime {
    base: Cell<Option<f64>>,
}

impl SignalReceiver<f64, f64> for AnimationTime {
    fn receive(&self, x: &f64) -> f64 {
        let base = self.base.get();

        if let Some(base) = base {
            (x - base).max(0.0)
        } else {
            self.base.set(Some(*x));
            0.0
        }
    }
}

/// Provide an infinite time signal for animations.
///
/// The signal will tick each frame until it is dropped.
pub fn infinite_animation() -> ReadSignal<f64> {
    animation_timestamp()
        .map_to(AnimationTime::default())
        .map(|time| {
            request_render_updates();
            *time
        })
        .only_changes()
}

/// Provide a finite time signal for animations.
///
/// The signal will tick each frame until `duration_millis` has elapsed. The
/// value will never exceed `duration_millis` and the last value will be
/// `duration_millis`, unless the signal is dropped first.
pub fn finite_animation(duration_millis: f64) -> ReadSignal<f64> {
    animation_timestamp()
        .map_to(AnimationTime::default())
        .map(move |&time| {
            if time < duration_millis {
                request_render_updates();
                time
            } else {
                duration_millis
            }
        })
        .only_changes()
}
