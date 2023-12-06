use futures_signals::{
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use itertools::Itertools;
use silkenweb::{
    document::{Document, DocumentHead},
    dom::{Dom, Hydro, Wet},
    elements::html::{meta, Meta},
    task::render_now,
};
use silkenweb_base::document;
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
    use silkenweb::{dom::Dry, task};
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
    Wet::unmount_all();
    Hydro::unmount_all();
    let head = document::head().unwrap();
    let existing = head.inner_html();
    let id1 = "my-id1";
    let numbers1 = head_vec::<D>(id1);
    let id2 = "my-id2";
    let numbers2 = head_vec::<D>(id2);
    render_now().await;
    assert_eq!(head.inner_html(), existing.as_str());
    numbers1.lock_mut().push(0);
    numbers2.lock_mut().push(0);
    render_now().await;
    assert_eq!(
        head.inner_html(),
        format!(r#"{existing}{}"#, items([(0, id1), (0, id2)]))
    );
    numbers1.lock_mut().push(1);
    numbers2.lock_mut().push(1);
    render_now().await;
    assert_eq!(
        head.inner_html(),
        format!(
            r#"{existing}{}"#,
            items([(0, id1), (0, id2), (1, id1), (1, id2)])
        )
    );
    numbers1.lock_mut().pop();
    render_now().await;
    assert_eq!(
        head.inner_html(),
        format!(r#"{existing}{}"#, items([(0, id1), (0, id2), (1, id2)]))
    );
    // TODO: More test cases
}

fn head_vec<D: Document>(id: &str) -> MutableVec<usize> {
    let numbers = MutableVec::new();

    D::mount_in_head(
        id,
        DocumentHead::new().children_signal(
            numbers
                .signal_vec()
                .map(|n| meta().name("description").content(format!("item {n}"))),
        ),
    );

    numbers
}

fn items<'a>(items: impl IntoIterator<Item = (usize, &'a str)>) -> String {
    items
        .into_iter()
        .map(|(n, id)| {
            format!(r#"<meta name="description" content="item {n}" {HEAD_ID_ATTRIBUTE}="{id}">"#)
        })
        .join("")
}
