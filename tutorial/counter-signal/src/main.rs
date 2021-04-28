use std::mem;

use silkenweb::signal::{ReadSignal, Signal};

// ANCHOR: body
fn main() {
    let count = Signal::new(0);
    // ANCHOR: map_signal
    let print_count: ReadSignal<()> = count
        .read()
        .map(|&count| println!("The count is {}", count));
    // ANCHOR_END: map_signal

    println!("Setting `count` to 1");
    count.write().set(1);
    println!("Dropping `print_count`");
    mem::drop(print_count);
    println!("Setting `count` to 2");
    count.write().set(2);
}
// ANCHOR_END: body
