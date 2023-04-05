use silkenweb::{log_panics, mount};
use silkenweb_examples_todomvc::{model::TodoApp, view::TodoAppView};

fn main() {
    log_panics();
    mount("app", TodoAppView::new(TodoApp::load()).render());
}
