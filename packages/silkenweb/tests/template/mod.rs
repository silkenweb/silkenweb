use futures_signals::{
    signal::Mutable,
    signal_vec::{MutableVec, SignalVecExt},
};
use silkenweb::{
    elements::{
        html::{div, DivTemplate},
        HtmlElement,
    },
    node::Node,
    prelude::ParentElement,
    task::render_now,
    value::Sig,
};

use super::PlatformDom;

async fn check<Param: 'static>(
    template: &DivTemplate<Param, PlatformDom>,
    param: Param,
    expected: &str,
) {
    render_now().await;
    let node: Node<PlatformDom> = template.instantiate(&param).into();
    assert_eq!(node.to_string(), expected);
}

isomorphic_test! {
    async fn template_text() {
        let template: DivTemplate<String, PlatformDom> = div().on_instantiate(|div, s| div.text(s)).freeze();
        check(&template, "Hello, world!".to_string(), r#"<div>Hello, world!</div>"#).await;
        check(&template, "Goodbye!".to_string(), r#"<div>Goodbye!</div>"#).await;
    }
}

isomorphic_test! {
    async fn template_attribute() {
        let template: DivTemplate<String, PlatformDom> = div().on_instantiate(|div, s| div.id(s)).freeze();
        check(&template, "my-id".to_string(), r#"<div id="my-id"></div>"#).await;
        check(&template, "my-other-id".to_string(), r#"<div id="my-other-id"></div>"#).await;
    }
}

isomorphic_test! {
    async fn template_child() {
        let template: DivTemplate<String, PlatformDom> = div().on_instantiate(|d, s| d.child(div().id(s))).freeze();
        check(&template, "my-id".to_string(), r#"<div><div id="my-id"></div></div>"#).await;
        check(&template, "my-other-id".to_string(), r#"<div><div id="my-other-id"></div></div>"#).await;
    }
}

isomorphic_test! {
    async fn template_text_signal() {
        let text = Mutable::new("Hello, world!".to_string());
        let template: DivTemplate<(), PlatformDom> = div().text(Sig(text.signal_cloned())).freeze();
        check(&template, (), r#"<div>Hello, world!</div>"#).await;
        text.set("Goodbye!".to_string());
        check(&template, (), r#"<div>Goodbye!</div>"#).await;
    }
}

isomorphic_test! {
    async fn template_attribute_signal() {
        let text = Mutable::new("my-id".to_string());
        let template: DivTemplate<(), PlatformDom> = div().id(Sig(text.signal_cloned())).freeze();
        check(&template, (), r#"<div id="my-id"></div>"#).await;
        text.set("my-other-id".to_string());
        check(&template, (), r#"<div id="my-other-id"></div>"#).await;
    }
}

isomorphic_test! {
    async fn template_children_signal() {
        let children: MutableVec<usize> = MutableVec::new();
        let children_signal = children.signal_vec().map(|i| div().text(i.to_string()));
        let template: DivTemplate<(), PlatformDom> = div().children_signal(children_signal).freeze();
        children.lock_mut().push(0);
        check(&template, (), r#"<div><div>0</div></div>"#).await;
        children.lock_mut().push(1);
        check(&template, (), r#"<div><div>0</div><div>1</div></div>"#).await;
        children.lock_mut().insert(1, 2);
        check(&template, (), r#"<div><div>0</div><div>2</div><div>1</div></div>"#).await;
        children.lock_mut().set(1, 3);
        check(&template, (), r#"<div><div>0</div><div>3</div><div>1</div></div>"#).await;
        children.lock_mut().remove(1);
        check(&template, (), r#"<div><div>0</div><div>1</div></div>"#).await;
        children.lock_mut().clear();
        check(&template, (), r#"<div></div>"#).await;
    }
}
