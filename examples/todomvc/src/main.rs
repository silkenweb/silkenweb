use silkenweb::mount;
use silkenweb_examples_todomvc::{model::TodoApp, view::TodoAppView};

fn main() {
    console_error_panic_hook::set_once();
    mount("app", TodoAppView::new(TodoApp::load()).render());
}
