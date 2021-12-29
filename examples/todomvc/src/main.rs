#[macro_use]
extern crate derive_more;

use app_model::{Filter, TodoApp};
use app_view::TodoAppView;
use futures_signals::signal::SignalExt;
use silkenweb::{mount, router::url};

mod app_model;
mod app_view;
mod item_model;
mod item_view;

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
