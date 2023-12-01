use futures_signals::signal::Mutable;

pub fn body() {
    /*
    # Signals

    This chapter teaches you the very basics of Signals. See the [`futures-signals` tutorial](https://docs.rs/futures-signals/latest/futures_signals/tutorial/index.html) for a more thorough introduction.
    */
    let x = Mutable::new(1);
    assert_eq!(x.get(), 1);
}
