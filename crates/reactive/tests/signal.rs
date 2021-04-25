use std::{cell::Cell, mem, rc::Rc};

use silkenweb_reactive::signal::Signal;

#[test]
fn callback_cleanup() {
    let state = Rc::new(Cell::new(0));
    let x = Signal::new(0);
    let y = x.read().map({
        let state = state.clone();
        move |x| state.replace(*x)
    });

    x.write().set(1);
    mem::drop(y);
    x.write().set(2);
    assert_eq!(state.get(), 1);
}

#[test]
#[should_panic]
fn circular_dependency() {
    let x_signal = Signal::new(());
    x_signal.read().map(move |_| x_signal.write().set(()));
}
