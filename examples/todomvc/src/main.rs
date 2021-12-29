#[macro_use]
extern crate derive_more;

use futures_signals::signal::SignalExt;
use model::{Filter, TodoApp};
use silkenweb::{mount, router::url};
use view::TodoAppView;

mod model;
mod view;

fn main() {
    console_error_panic_hook::set_once();

    let item_filter = url().signal_cloned().map({
        |url| match url.hash().as_str() {
            "#/active" => Filter::Active,
            "#/completed" => Filter::Completed,
            _ => Filter::All,
        }
    });

    mount("app", TodoAppView::new(TodoApp::load()).render(item_filter));
}
