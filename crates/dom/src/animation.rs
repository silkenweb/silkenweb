//! Provide animation time signals.
//!
//! Each time signal is in milliseconds, and starts from 0. Any live
//! animation time signals will be updated each frame, and are strictly
//! increasing.
use futures_signals::signal::{Signal, SignalExt};

use crate::render::{animation_timestamp, request_render_updates};

/// Provide an infinite time signal for animations.
///
/// The signal will tick each frame until it is dropped.
///
/// See [module-level documentation](self) for more details.
pub fn infinite_animation() -> impl 'static + Signal<Item = f64> {
    animation_timestamp()
        .map(|time| {
            request_render_updates();
            time
        })
        .dedupe()
}

/// Provide a finite time signal for animations.
///
/// The signal will tick each frame until `duration_millis` has elapsed. The
/// value will never exceed `duration_millis` and the last value will be
/// `None`, unless the signal is dropped first.
///
/// See [module-level documentation](self) for more details.
pub fn finite_animation(duration_millis: f64) -> impl 'static + Signal<Item = Option<f64>> {
    animation_timestamp()
        .map(move |time| {
            if time < duration_millis {
                request_render_updates();
                Some(time)
            } else {
                None
            }
        })
        .dedupe()
}
