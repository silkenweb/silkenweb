#[doc(hidden)]
pub mod private {
    pub use futures_signals::map_ref;
}

/// Clone all the identifiers supplied as arguments.
///
/// `clone!(x, y, z);` will generate:
///
/// ```
/// # #[macro_use] extern crate silkenweb_dom;
/// # let (x, y, z) = (0, 0, 0);
/// let x = x.clone();
/// let y = y.clone();
/// let z = z.clone();
/// ```
///
/// This is useful for capturing variables by copy in closures. For example:
///
/// ```
/// # #[macro_use] extern crate silkenweb_dom;
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

#[doc(hidden)]
#[macro_export]
macro_rules! named_product{
    ( $($name:ident),* ; ; $($id:ident = $e:expr),*) => {
        $crate::macros::private::map_ref!(
            $(let $id = $e),* => ($(*$id),*)
        )
    };
    ($name:ident $(, $name_tail:ident)* ; $expression:expr $(, $expression_tail:expr)* ; $($id:ident = $e:expr),*) => {
        $crate::named_product!($($name_tail),*; $($expression_tail),*; $($id = $e, )* $name = $expression )
    };
    ( ; $($expression:expr),* ; $($id:ident = $e:expr),*) => { compile_error!("Exceeded maximum of 10 arguments") }
}

#[macro_export]
macro_rules! product{
    ($($e:expr),* $(,)?) => {
        $crate::named_product!(a, b, c, d, e, f, g, h, i, j; $($e),*; )
    };
}
