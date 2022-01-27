#[macro_use]
extern crate derive_more;

use futures_signals::signal;
use model::{Filter, TodoApp};
use silkenweb::{dom::{mount, run_until_stalled, element::Element}, router::url};
use view::TodoAppView;

mod model;
mod view;

fn main() {
    console_error_panic_hook::set_once();

    run_until_stalled();
    println!("{}", Element::from(TodoAppView::new(TodoApp::load()).render(signal::always(Filter::Active))));
}
