use silkenweb::{document::Document, dom::DefaultDom, mount};
use ssr_full_app::app;

pub fn main() {
    let (head, body) = app();
    DefaultDom::mount_in_head("head", head);
    mount("body", body);
}
