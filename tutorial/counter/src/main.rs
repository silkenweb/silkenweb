use silkenweb::{mount, signal::Signal};
use silkenweb_tutorial_common::define_counter;

// ANCHOR: body
fn main() {
    let count = Signal::new(0);
    let app = define_counter(&count);

    mount("app", app);
}
// ANCHOR_END: body
