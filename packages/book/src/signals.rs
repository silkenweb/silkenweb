use futures_signals::signal::Mutable;
use silkenweb::task::TaskSignal;

pub fn body() {
    // # Signals
    //
    // This chapter teaches you the very basics of Signals. See the [`futures-signals` tutorial](https://docs.rs/futures-signals/latest/futures_signals/tutorial/index.html) for a more thorough introduction.
    //
    // ## Signals and Mutables
    //
    // A `Mutable` is a variable whose changes can be tracked by a `Signal`. Lets go
    // through a simple example.
    //
    // First we create a new `Mutable` initialized with the value `1`:
    let x = Mutable::new(1);
    assert_eq!(x.get(), 1);
    // Now lets set up a signal to track changes to `x`:
    let x_signal = x.signal();
    // By itself, this won't do anything. We have to tell it what we want to do when
    // the signal changes. In this case `spawn_for_each` will spawn a task on the
    // [microtask queue](https://developer.mozilla.org/en-US/docs/Web/API/HTML_DOM_API/Microtask_guide) that logs the value of `x` when it changes.
    x_signal.spawn_for_each(|x| {
        web_log::println!("{x}");
        async {}
    });
    // Normally, you won't need to call `spawn_for_each` as you'll pass the
    // `Signal` to Silkenweb, and Silkenweb will watch for any changes. We'll
    // see this in action in the chapter on [reactivity](./reactivity.md).
    //
    // ## Push/Pull
    //
    // Signals are both push and pull.
    //
    // - They are push, in the sense that the future will notify that it wants
    //   to be woken when there is a change. This uses the normal
    //   [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html#)
    //   polling mechanism.
    // - They are pull, in the sense that the future will pull the value from
    //   the signal when it is polled.
    //
    // ## Streams
    //
    // Signals are like streams in some ways, but there are differences:
    //
    // - Signals are allowed to skip intermediate values for efficiency. In our
    //   example, not every intermediate value of `x` will be printed.
    // - Signals always have at least one value, whereas a stream can be empty.
}
