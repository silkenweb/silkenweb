use silkenweb::mount;
use ssr_example_app::app;

pub fn main() {
    let (title, body) = app();
    mount("title", title);
    mount("body", body);
}
