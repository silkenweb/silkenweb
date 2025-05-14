use futures_signals::{
    signal::Mutable,
    signal_vec::{MutableVec, SignalVecExt},
};
use silkenweb::{
    dom::Template,
    elements::{
        html::{div, Div},
        HtmlElement,
    },
    node::{
        element::{Const, ParentElement, TextParentElement},
        Node,
    },
    task::render_now,
    value::Sig,
};
use silkenweb_macros::cfg_browser;

async fn check<Param: 'static>(
    template: &Div<Template<Param>, Const>,
    param: Param,
    expected: &str,
) {
    render_now().await;
    let node: Node = template.instantiate(&param).into();
    assert_eq!(node.to_string(), expected);
}

isomorphic_test! {
    async fn template_text() {
        let template: Div<Template<String>, Const> = div().on_instantiate(|div, s| div.text(s)).freeze();
        check(&template, "Hello, world!".to_string(), r#"<div>Hello, world!</div>"#).await;
        check(&template, "Goodbye!".to_string(), r#"<div>Goodbye!</div>"#).await;
    }
}

isomorphic_test! {
    async fn template_attribute() {
        let template: Div<Template<String>, Const> = div().on_instantiate(|div, s| div.id(s)).freeze();
        check(&template, "my-id".to_string(), r#"<div id="my-id"></div>"#).await;
        check(&template, "my-other-id".to_string(), r#"<div id="my-other-id"></div>"#).await;
    }
}

isomorphic_test! {
    async fn template_child() {
        let template: Div<Template<String>, Const> = div().on_instantiate(|d, s| d.child(div().id(s))).freeze();
        check(&template, "my-id".to_string(), r#"<div><div id="my-id"></div></div>"#).await;
        check(&template, "my-other-id".to_string(), r#"<div><div id="my-other-id"></div></div>"#).await;
    }
}

isomorphic_test! {
    async fn template_text_signal() {
        let text = Mutable::new("Hello, world!".to_string());
        let template: Div<Template<()>, Const> = div().text(Sig(text.signal_cloned())).freeze();
        check(&template, (), r#"<div>Hello, world!</div>"#).await;
        text.set("Goodbye!".to_string());
        check(&template, (), r#"<div>Goodbye!</div>"#).await;
    }
}

isomorphic_test! {
    async fn template_attribute_signal() {
        let text = Mutable::new("my-id".to_string());
        let template: Div<Template<()>, Const> = div().id(Sig(text.signal_cloned())).freeze();
        check(&template, (), r#"<div id="my-id"></div>"#).await;
        text.set("my-other-id".to_string());
        check(&template, (), r#"<div id="my-other-id"></div>"#).await;
    }
}

isomorphic_test! {
    async fn template_children_signal() {
        let children: MutableVec<usize> = MutableVec::new();
        let children_signal = children.signal_vec().map(|i| div().text(i.to_string()));
        let template: Div<Template<()>, Const> = div().children_signal(children_signal).freeze();
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

#[cfg_browser(false)]
mod dry {
    use silkenweb::{
        dom::{Dry, Template},
        elements::html::{div, Div},
        node::element::{Const, ParentElement, TextParentElement},
        task::{server::render_now_sync, sync_scope},
    };

    #[test]
    fn dry_clone_node_is_deep() {
        sync_scope(|| {
            let template: Div<Template<String, Dry>, Const> = div()
                .child(div().on_instantiate(|div, s| div.text(s)))
                .freeze();
            render_now_sync();
            let node1 = template.instantiate(&"Hello, world!".to_string()).freeze();
            let expected_node1 = r#"<div><div>Hello, world!</div></div>"#;
            assert_eq!(node1.to_string(), expected_node1);

            render_now_sync();
            let node2 = template.instantiate(&"Goodbye!".to_string()).freeze();
            assert_eq!(node2.to_string(), r#"<div><div>Goodbye!</div></div>"#);

            // If the dry clone isn't deep, this will fail.
            assert_eq!(node1.to_string(), expected_node1);
        })
    }
}
