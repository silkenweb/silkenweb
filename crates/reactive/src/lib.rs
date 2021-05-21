//! Various primitives to make your code reactive to upstream updates.
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]
pub mod accumulators;
pub mod containers;
pub mod memo;
pub mod signal;

/// Clone all the identifiers supplied as arguments.
///
/// `clone!(x, y, z);` will generate:
///
/// ```
/// # #[macro_use] extern crate silkenweb_reactive;
/// # let (x, y, z) = (0, 0, 0);
/// let x = x.clone();
/// let y = y.clone();
/// let z = z.clone();
/// ```
///
/// This is useful for capturing variables by copy in closures. For example:
///
/// ```
/// # #[macro_use] extern crate silkenweb_reactive;
/// # let (x, y, z) = (0, 0, 0);
/// # let signal = vec![0].into_iter();
/// # fn do_something(x: u32, y: u32, z: u32) {}
/// signal.map({
///     clone!(x, y, z);
///     move |_| do_something(x, y, z)
/// });
/// ```
#[macro_export]
macro_rules! clone{
    ($($name:ident),* $(,)?) => {
        $(
            let $name = $name.clone();
        )*
    }
}
