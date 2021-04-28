use silkenweb::{mount, signal::Signal};
use silkenweb_tutorial_common::render_counter;

fn main() {
    let count = Signal::new(0);
    // TODO: Change render_ to define_
    let app = render_counter(&count);

    mount("app", app);
}
