use futures_signals::{
    signal::Mutable,
    signal_vec::{MutableVec, VecDiff},
};
use silkenweb::task::{TaskSignal, TaskSignalVec};

pub fn body() {
    // # Signals
    //
    // This chapter teaches you the very basics of Signals. See the
    // [`futures-signals` tutorial] for a more thorough introduction.

    // ## Signals and Mutables
    //
    // A `Mutable` is a variable with [interior mutability] whose changes can be
    // tracked by a `Signal`. Lets go through a simple example.
    //
    // First we create a new `Mutable` initialized with the value `1`:
    let x = Mutable::new(1);
    assert_eq!(x.get(), 1);
    // Now lets set up a signal to track changes to `x`:
    let x_signal = x.signal();
    // By itself, this won't do anything. We have to tell it what we want to do when
    // the signal changes. In this case `spawn_for_each` will spawn a task on the
    // [microtask queue] that logs the value of `x` when it changes.
    x_signal.spawn_for_each(|x| {
        web_log::println!("{x}");
        async {}
    });
    // Normally, you won't need to call `spawn_for_each` as you'll pass the
    // `Signal` to Silkenweb, and Silkenweb will watch for any changes. We'll
    // see this in action in the chapter on [reactivity](./reactivity.md).

    // ## Push/Pull
    //
    // Signals are both push and pull.
    //
    // - They are push, in the sense that the future will notify that it wants to be
    //   woken when there is a change. This uses the normal [`Future`] polling
    //   mechanism.
    // - They are pull, in the sense that the future will pull the value from the
    //   signal when it is polled.

    // ## Streams
    //
    // Signals are like streams in some ways, but there are differences:
    //
    // - Signals are allowed to skip intermediate values for efficiency. In our
    //   example, not every intermediate value of `x` will be printed.
    // - Signals always have at least one value, whereas a stream can be empty.

    // # Differential Signals
    //
    // More complex data types don't always fit well with the normal, value
    // based signals that we've seen so far. It wouldn't be very useful to be
    // notified with a completely new vector every time an element changes, for
    // example. For vectors, we use [`signal_vec`]. This allows us to see what's
    // changed in the vector using [`VecDiff`].
    //
    // Here's how we create a [`MutableVec`] and push a value onto it:
    let v = MutableVec::new();
    v.lock_mut().push(1);
    // and we can listen for changes with:
    v.signal_vec().spawn_for_each(|delta| {
        let action = match delta {
            VecDiff::Replace { .. } => "Replace",
            VecDiff::InsertAt { .. } => "InsertAt",
            VecDiff::UpdateAt { .. } => "UpdateAt",
            VecDiff::RemoveAt { .. } => "RemoveAt",
            VecDiff::Move { .. } => "Move",
            VecDiff::Push { .. } => "Push",
            VecDiff::Pop {} => "Pop",
            VecDiff::Clear {} => "Clear",
        };

        web_log::println!("{action}");
        async {}
    })

    // [`signal_vec`] intermediate values can't be discarded like with value
    // signals, as we need to know all the deltas to reconstruct a vector. They
    // can however be combined as an optimization. For example, maybe a push
    // followed by a pop could be discarded. This is implementation defined
    // though, and not guaranteed.

    // [`futures-signals` tutorial]: https://docs.rs/futures-signals/latest/futures_signals/tutorial/index.html
    // [interior mutability]: https://doc.rust-lang.org/reference/interior-mutability.html
    // [microtask queue]: https://developer.mozilla.org/en-US/docs/Web/API/HTML_DOM_API/Microtask_guide
    // [`Future`]: https://doc.rust-lang.org/std/future/trait.Future.html
    // [`signal_vec`]: https://docs.rs/futures-signals/latest/futures_signals/signal_vec/index.html
    // [`MutableVec`]: https://docs.rs/futures-signals/latest/futures_signals/signal_vec/struct.MutableVec.html
    // [`VecDiff`]: https://docs.rs/futures-signals/latest/futures_signals/signal_vec/enum.VecDiff.html
}
