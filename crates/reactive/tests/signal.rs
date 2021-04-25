mod common;

use std::mem;

use common::State;
use silkenweb_reactive::{
    clone,
    signal::{Signal, ZipSignal},
};

#[test]
fn zip_signals() {
    let x = Signal::new(1);
    let y = Signal::new(10);

    let sum = (x.read(), y.read()).map(move |&x, &y| x + y);
    assert_eq!(*sum.current(), 11);

    x.write().set(2);
    assert_eq!(*sum.current(), 12);

    y.write().set(20);
    assert_eq!(*sum.current(), 22);
}

#[test]
fn callback_cleanup() {
    let state = State::new(0);
    let x = Signal::new(0);
    let y = x.read().map({
        clone!(state);
        move |x| *state.get_mut() = *x
    });

    x.write().set(1);
    mem::drop(y);
    x.write().set(2);
    assert_eq!(*state.get(), 1, "The state shouldn't be updated once `y` is dropped");
}

#[test]
#[should_panic]
fn circular_dependency() {
    let x_signal = Signal::new(());
    x_signal.read().map(move |_| x_signal.write().set(()));
}

#[test]
fn change_propagation() {
    let x = Signal::new(0);

    let all_changes_state = State::new(0);
    let all_changes = x.read().map({
        clone!(all_changes_state);
        move |_| *all_changes_state.get_mut() += 1
    });

    let only_diffs_state = State::new(0);
    let only_diffs = x.read().only_changes().map({
        clone!(only_diffs_state);
        move |_| *only_diffs_state.get_mut() += 1
    });

    x.write().set(1);
    x.write().set(1);

    assert_eq!(*all_changes_state.get(), 3, "Once for initialization, then 2 updates");
    assert_eq!(*only_diffs_state.get(), 2, "Once for initialization, the 1 update that actually changes");
    mem::drop(all_changes);
    mem::drop(only_diffs);
}
