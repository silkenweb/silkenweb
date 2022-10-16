//! Animation time signals.
//!
//! Animations are built using signals that tick each animation frame. The
//! signal value is the number of milliseconds since the start of the animation,
//! so you just need to render the animation frame corresponding to the elapsed
//! time.
//!
//! See [`finite_animation`] and [`infinite_animation`] for examples.
use futures_signals::signal::{Signal, SignalExt};

use crate::task::{animation_timestamp, request_animation_frame};

/// Provide a finite time signal for animations.
///
/// The signal will tick each frame until `duration_millis` has elapsed. The
/// value will never exceed `duration_millis` and the last value will be
/// `None`, unless the signal is dropped first.
///
/// # Example
///
/// Slowly filling a progress bar:
///
/// ```no_run
/// # use futures_signals::signal::SignalExt;
/// # use silkenweb::{
/// #   animation::finite_animation,
/// #   elements::html::progress,
/// #   node::element::Sig,
/// #   mount,
/// # };
/// const DURATION: f64 = 3000.0;
/// progress().max(DURATION as f32).value(Sig(
///     finite_animation(DURATION).map(|time| time.unwrap_or(DURATION) as f32)
/// ));
/// ```
///
/// See [module-level documentation](self) for more details.
pub fn finite_animation(duration_millis: f64) -> impl Signal<Item = Option<f64>> + 'static {
    animation_timestamp()
        .map(move |time| {
            if time < duration_millis {
                request_animation_frame();
                Some(time)
            } else {
                None
            }
        })
        .dedupe()
}

/// Provide an infinite time signal for animations.
///
/// The signal will tick each frame until it is dropped.
///
/// # Example
///
/// A rotating square:
///
/// ```no_run
/// # use futures_signals::signal::SignalExt;
/// # use silkenweb::{
/// #     animation::infinite_animation,
/// #     elements::svg::{attributes::Presentation, content_type::Length::Px, rect, svg},
/// #     node::element::{Sig, ParentBuilder},
/// #     mount,
/// # };
/// svg().width(200.0).height(200.0).child(
///     rect()
///         .x(Px(25.0))
///         .y(Px(25.0))
///         .width(Px(50.0))
///         .height(Px(50.0))
///         .transform(Sig(
///             infinite_animation().map(|time| format!("rotate({} 50 50)", time / 10.0))
///         )),
/// );
/// ```
///
/// See [module-level documentation](self) for more details.
pub fn infinite_animation() -> impl Signal<Item = f64> + 'static {
    animation_timestamp()
        .map(|time| {
            request_animation_frame();
            time
        })
        .dedupe()
}
