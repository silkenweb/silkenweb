//! Abstract over the signal/value-ness of types.
//!
//! This allows you to write a single function that works for both signals and
//! values.
//!
//! # Example
//!
//! ```
//! # use futures_signals::signal::Mutable;
//! # use silkenweb_signals_ext::value::*;
//! # use std::future::Future;
//! #
//! struct Exec;
//!
//! impl Executor for Exec {
//!     fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
//!         // This is a stub `Executor` for brevity. In real code, it should
//!         // run the future.
//!     }
//! }
//!
//! fn increment_and_print(x: impl SignalOrValue<Item = i32>) {
//!     x.map(|x| x + 1).for_each(
//!         |_exec, x| println!("{x}"),
//!         |_exec| {
//!             |x| {
//!                 // We use `println` for brevity, but this should really do
//!                 // `async` IO as it'll be executed in a future.
//!                 println!("{x}");
//!                 async {}
//!             }
//!         },
//!         &mut Exec,
//!     );
//! }
//!
//! let x_signal = Mutable::new(0);
//! let x_value = 0;
//!
//! increment_and_print(x_value);
//! increment_and_print(Sig(x_signal.signal()));
//! ```
use std::future::Future;

use futures_signals::signal::{self, Always, Signal, SignalExt};

/// Newtype wrapper to mark this type as a signal.
///
/// For use with [`SignalOrValue`] and [`RefSignalOrValue`]
pub struct Sig<T>(pub T);

/// Newtype wrapper to mark this type as a static value.
///
/// For use with [`SignalOrValue`] and [`RefSignalOrValue`] for when you can't
/// implement [`Value`] for a type.
pub struct Val<T>(pub T);

// TODO: Doc and put in `sync` module with sharedref?
// TODO: Unify ServerSend and silkenweb::ServerSend
#[cfg(target_arch = "wasm32")]
pub trait ServerSend {}

#[cfg(target_arch = "wasm32")]
impl<T: ?Sized> ServerSend for T {}

#[cfg(not(target_arch = "wasm32"))]
pub trait ServerSend: Send {}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Send + ?Sized> ServerSend for T {}

/// Abstract over a type that can be a value or a signal of an underlying type.
pub trait RefSignalOrValue<'a>: ServerSend {
    /// The underlying type of the value or signal.
    type Item: ServerSend + 'a;
    /// The signal type. Use [`Always`] for value types.
    type Signal: Signal<Item = Self::Item> + ServerSend + 'a;
    /// The return type for [`Self::map`].
    type Map<'b, F, R>: RefSignalOrValue<'b, Item = R> + 'b
    where
        'b: 'a,
        F: FnMut(Self::Item) -> R + ServerSend + 'b,
        R: RefSignalOrValue<'b, Item = R> + 'b;

    /// Map a function over this signal/value to produce a new signal/value.
    fn map<'b: 'a, F, R>(self, callback: F) -> Self::Map<'b, F, R>
    where
        R: RefSignalOrValue<'b, Item = R> + 'b,
        F: FnMut(Self::Item) -> R + ServerSend + 'b;

    /// Apply a function over the value or each value of a signal.
    ///
    /// # Params
    ///
    /// - `fn_val`: The function to apply if [`Self`] is a value type.
    /// - `fn_init_sig`: The function to generate a function to apply if
    ///   [`Self`] is signal type.
    /// - `executor`: Where to run the future returned from
    ///   [`SignalExt::for_each`], if this is a signal.
    fn for_each<FVal, FInitSig, FSig, Task, Exec>(
        self,
        fn_val: FVal,
        fn_init_sig: FInitSig,
        executor: &mut Exec,
    ) where
        FVal: FnOnce(&mut Exec, Self::Item),
        FInitSig: FnOnce(&mut Exec) -> FSig,
        FSig: FnMut(Self::Item) -> Task + 'a,
        Task: Future<Output = ()> + 'a,
        Exec: Executor;

    /// Select a function based on whether this is a signal or value.
    ///
    /// # Params
    ///
    /// - `fn_val`: The function to call if this is a value.
    /// - `fn_sig`: The function to call if this is a signal.
    /// - `data`: Some data for the function to consume. This is useful if
    ///   either of the functions needs to consume some data.
    fn select<FVal, FSig, Data, Out>(self, fn_val: FVal, fn_sig: FSig, data: Data) -> Out
    where
        FVal: FnOnce(Data, Self::Item) -> Out,
        FSig: FnOnce(Data, Self::Signal) -> Out,
        Self: Sized;
}

/// Like [`RefSignalOrValue`], when you know the type is `'static`.
pub trait SignalOrValue: RefSignalOrValue<'static> {}

impl<T: RefSignalOrValue<'static>> SignalOrValue for T {}

/// A type that can spawn futures.
pub trait Executor {
    fn spawn(&mut self, future: impl Future<Output = ()> + 'static);
}

/// Marker trait for values that can be used with [`RefSignalOrValue`].
pub trait RefValue<'a>: ServerSend {}

/// Marker trait for values that can be used with [`SignalOrValue`].
pub trait Value: RefValue<'static> {}

impl<T: Value> RefValue<'static> for T {}

macro_rules! static_values{
    ($($t:ty),*) => {
        $(
            impl Value for $t {}
        )*
    }
}

static_values!(i8, i16, i32, i64);
static_values!(u8, u16, u32, u64);
static_values!(f32, f64);
static_values!(bool, String);

impl<'a> RefValue<'a> for &'a str {}
impl<'a> RefValue<'a> for &'a String {}
impl<'a, T: ServerSend + 'a> RefValue<'a> for Option<T> {}
impl<'a, T: ServerSend + 'a> RefValue<'a> for [T] {}
impl<'a, T> RefValue<'a> for &'a [T]
where 
    &'a [T]: ServerSend,
    T: ServerSend + 'a 
{}
impl<'a, const COUNT: usize, T: ServerSend + 'a> RefValue<'a> for [T; COUNT] {}

impl<'a> RefValue<'a> for () {}

macro_rules! tuple_values {
    ($t:ident $(,)?) => {};
    ($t0:ident, $t1:ident $(, $tail:ident)* $(,)?) => {
        impl<'a, $t0: $crate::value::ServerSend, $t1: $crate::value::ServerSend $(, $tail: $crate::value::ServerSend)*> RefValue<'a> for ($t0, $t1 $(, $tail)*) {}

        tuple_values!($t1, $($tail),*);
    }
}

tuple_values!(A, B, C, D, E, F, G, H, I, J);

impl<'a, T> RefSignalOrValue<'a> for T
where
    T: RefValue<'a> + 'a,
{
    type Item = Self;
    type Map<'b, F, R> = R
    where
        'b: 'a,
        F: FnMut(Self::Item) -> R + ServerSend + 'b,
        R: RefSignalOrValue<'b, Item = R> + 'b;
    type Signal = Always<Self::Item>;

    fn map<'b: 'a, F, R>(self, mut callback: F) -> Self::Map<'b, F, R>
    where
        R: RefSignalOrValue<'b, Item = R> + 'b,
        F: FnMut(Self::Item) -> R + ServerSend + 'b,
    {
        callback(self)
    }

    fn for_each<FVal, FInitSig, FSig, Task, Exec>(
        self,
        fn_val: FVal,
        _fn_init_sig: FInitSig,
        executor: &mut Exec,
    ) where
        FVal: FnOnce(&mut Exec, Self::Item),
        FInitSig: FnOnce(&mut Exec) -> FSig,
        FSig: FnMut(Self::Item) -> Task + 'a,
        Task: Future<Output = ()> + 'a,
        Exec: Executor,
    {
        fn_val(executor, self);
    }

    fn select<FVal, FSig, Data, Out>(self, fn_val: FVal, _fn_sig: FSig, data: Data) -> Out
    where
        FVal: FnOnce(Data, Self::Item) -> Out,
        FSig: FnOnce(Data, Self::Signal) -> Out,
    {
        fn_val(data, self)
    }
}

impl<'a, T> RefSignalOrValue<'a> for Val<T>
where
    T: ServerSend + 'static,
{
    type Item = T;
    type Map<'b, F, R> = R
    where
        'b: 'a,
        F: FnMut(Self::Item) -> R + ServerSend + 'b,
        R: RefSignalOrValue<'b, Item = R> + 'b;
    type Signal = Always<Self::Item>;

    fn map<'b: 'a, F, R>(self, mut callback: F) -> Self::Map<'b, F, R>
    where
        R: RefSignalOrValue<'b, Item = R> + 'b,
        F: FnMut(Self::Item) -> R + ServerSend + 'b,
    {
        callback(self.0)
    }

    fn for_each<FVal, FInitSig, FSig, Task, Exec>(
        self,
        fn_val: FVal,
        _fn_init_sig: FInitSig,
        executor: &mut Exec,
    ) where
        FVal: FnOnce(&mut Exec, Self::Item),
        FInitSig: FnOnce(&mut Exec) -> FSig,
        FSig: FnMut(Self::Item) -> Task + 'a,
        Task: Future<Output = ()> + 'a,
        Exec: Executor,
    {
        fn_val(executor, self.0);
    }

    fn select<FVal, FSig, Data, Out>(self, fn_val: FVal, _fn_sig: FSig, data: Data) -> Out
    where
        FVal: FnOnce(Data, Self::Item) -> Out,
        FSig: FnOnce(Data, Self::Signal) -> Out,
    {
        fn_val(data, self.0)
    }
}

impl<T, S> RefSignalOrValue<'static> for Sig<S>
where
    T: ServerSend + 'static,
    S: Signal<Item = T> + ServerSend + 'static,
{
    type Item = T;
    type Map<'b, F, R> = Sig<signal::Map<S, F>>
    where
        'b: 'static,
        F: FnMut(Self::Item) -> R + ServerSend + 'b,
        R: RefSignalOrValue<'b, Item = R> + 'b;
    type Signal = S;

    fn map<'b, F, R>(self, callback: F) -> Self::Map<'b, F, R>
    where
        'b: 'static,
        R: RefSignalOrValue<'b, Item = R> + 'b,
        F: FnMut(Self::Item) -> R + ServerSend + 'b,
    {
        Sig(self.0.map(callback))
    }

    fn for_each<FVal, FInitSig, FSig, Task, Exec>(
        self,
        _fn_val: FVal,
        fn_init_sig: FInitSig,
        executor: &mut Exec,
    ) where
        FVal: FnOnce(&mut Exec, Self::Item),
        FInitSig: FnOnce(&mut Exec) -> FSig,
        FSig: FnMut(Self::Item) -> Task + 'static,
        Task: Future<Output = ()> + 'static,
        Exec: Executor,
    {
        let fn_sig = fn_init_sig(executor);
        executor.spawn(self.0.for_each(fn_sig));
    }

    fn select<FVal, FSig, Data, Out>(self, _fn_val: FVal, fn_sig: FSig, data: Data) -> Out
    where
        FVal: FnOnce(Data, Self::Item) -> Out,
        FSig: FnOnce(Data, Self::Signal) -> Out,
    {
        fn_sig(data, self.0)
    }
}
