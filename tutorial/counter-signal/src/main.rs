use std::mem;

use silkenweb::signal::{ReadSignal, Signal, WriteSignal};

// ANCHOR: body
fn main() {
    let count = Signal::new(0);
    // ANCHOR: print_count
    let print_count: ReadSignal<()> = count
        .read()
        .map(|&count| println!("The count is {}", count));
    // ANCHOR_END: print_count

    // ANCHOR: define_set_count
    let set_count: WriteSignal<usize> = count.write();
    // ANCHOR_END: define_set_count

    println!("Setting `count` to 1");
    // ANCHOR: set_count
    set_count.set(1);
    // ANCHOR_END: set_count
    println!("Dropping `print_count`");
    mem::drop(print_count);
    println!("Setting `count` to 2");
    set_count.set(2);
}
// ANCHOR_END: body
