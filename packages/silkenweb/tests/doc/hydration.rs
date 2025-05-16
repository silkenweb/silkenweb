use futures_signals::signal::always;
use silkenweb::{
    document::DocumentHead,
    elements::html::{meta, p},
    hydration::{hydrate, hydrate_in_head},
    node::element::TextParentElement,
    value::Sig,
};
use silkenweb_task::spawn_local;

pub async fn hydrate_example() {
    let app = p().text(Sig(always("Hello, world!")));

    spawn_local(async {
        hydrate("app", app).await;
    });
}

pub async fn hydrate_in_head_example() {
    let head = DocumentHead::new().child(meta().name("description").content("A description"));

    spawn_local(async {
        hydrate_in_head("my-id", head).await;
    });
}
