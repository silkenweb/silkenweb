use futures_signals::signal::{Mutable, SignalExt};
use silkenweb::{
    document::{Document, DocumentHead},
    dom::{Dom, Dry, Hydro, Wet},
    elements::html::{meta, Meta},
    task::render_now,
};
use silkenweb_macros::cfg_browser;
use silkenweb_signals_ext::value::Sig;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

const HEAD_ID_ATTRIBUTE: &str = "data-silkenweb-head-id";

fn meta_description<D: Dom>() -> Meta<D> {
    meta().name("description").content("A description")
}

fn meta_description_text(id: &str) -> String {
    format!(r#"<meta name="description" content="A description" {HEAD_ID_ATTRIBUTE}="{id}">"#)
}

fn meta_keywords<D: Dom>() -> Meta<D> {
    meta().name("keywords").content("Test")
}

fn meta_keywords_text(id: &str) -> String {
    format!(r#"<meta name="keywords" content="Test" {HEAD_ID_ATTRIBUTE}="{id}">"#)
}

fn meta_author<D: Dom>() -> Meta<D> {
    meta().name("author").content("Simon Bourne")
}

fn meta_author_text(id: &str) -> String {
    format!(r#"<meta name="author" content="Simon Bourne" {HEAD_ID_ATTRIBUTE}="{id}">"#)
}

async fn basic<D: Document>() {
    D::unmount_all();
    let id = "my-id";
    let author = Mutable::new(false);
    let head = DocumentHead::new()
        .children([meta_description(), meta_keywords()])
        .optional_child(Sig(author.signal().map(|cond| cond.then(meta_author))));

    D::mount_in_head(id, head);

    render_now().await;

    assert_eq!(
        D::head_inner_html(),
        format!("{}{}", meta_description_text(id), meta_keywords_text(id))
    );

    author.set(true);
    render_now().await;

    assert_eq!(
        D::head_inner_html(),
        format!(
            "{}{}{}",
            meta_description_text(id),
            meta_keywords_text(id),
            meta_author_text(id)
        )
    );
}

#[cfg_browser(false)]
#[test]
fn dry_basic() {
    use silkenweb::task;
    task::server::block_on(task::scope(basic::<Dry>()))
}

#[wasm_bindgen_test]
async fn wet_basic() {
    basic::<Wet>().await
}

#[wasm_bindgen_test]
async fn hydro_basic() {
    basic::<Hydro>().await
}

#[wasm_bindgen_test]
async fn hydro_interleaved() {
    interleaved::<Hydro>().await
}

#[wasm_bindgen_test]
async fn wet_interleaved() {
    interleaved::<Wet>().await
}

// We don't test interleaving on `Dry` DOMs as the ordering is different, and
// the elements are segregated anyway.
async fn interleaved<D: Document>() {
    D::unmount_all();
    // TODO: Implement
}
