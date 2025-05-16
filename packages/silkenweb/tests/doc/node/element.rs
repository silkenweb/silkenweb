use clonelet::clone;
use futures_signals::signal::{Mutable, SignalExt};
#[cfg_browser(false)]
use silkenweb::task::{render_now, scope, server};
use silkenweb::{
    cfg_browser,
    dom::{DefaultDom, Dry},
    elements::{
        html::{button, div, input, p, Div},
        ElementEvents,
    },
    node::element::{Element, ParentElement, TextParentElement},
    value::Sig,
};
use web_sys::HtmlInputElement;

pub fn static_single_class_name() {
    let app: Div<Dry> = div().class("my-class").class("my-other-class");
    assert_eq!(
        app.freeze().to_string(),
        r#"<div class="my-class my-other-class"></div>"#
    );
}

#[cfg_browser(false)]
pub fn dynamic_single_class_name() {
    // `block_on` is only required if we're outside the browser
    server::block_on(scope(async {
        let my_class = Mutable::new("my-class");
        let my_other_class = Mutable::new("my-other-class");
        let app: Div<Dry> = div()
            .class(Sig(my_class.signal()))
            .class(Sig(my_other_class.signal()));
        let app = app.freeze();

        render_now().await;
        assert_eq!(
            app.to_string(),
            r#"<div class="my-class my-other-class"></div>"#
        );

        my_other_class.set("my-other-class-updated");

        render_now().await;
        assert_eq!(
            app.to_string(),
            r#"<div class="my-class my-other-class-updated"></div>"#
        );
    }))
}

pub fn static_class_names() {
    let app: Div<Dry> = div().classes(["class0", "class1"]);
    assert_eq!(
        app.freeze().to_string(),
        r#"<div class="class0 class1"></div>"#
    );
}

#[cfg_browser(false)]
pub fn dynamic_class_names() {
    // `block_on` is only required if we're outside the browser
    server::block_on(scope(async {
        let my_classes = Mutable::new(vec!["class0", "class1"]);
        let app: Div<Dry> = div().classes(Sig(my_classes.signal_cloned()));
        let app = app.freeze();

        render_now().await;
        assert_eq!(app.to_string(), r#"<div class="class0 class1"></div>"#);

        my_classes.set(vec![]);

        render_now().await;
        assert_eq!(app.to_string(), r#"<div class=""></div>"#);
    }))
}

pub fn effect() {
    input::<DefaultDom>().effect(|elem: &HtmlInputElement| elem.focus().unwrap());
}

pub fn handle() {
    let text = Mutable::new("".to_string());
    let input = input();
    let input_handle = input.handle();
    div::<DefaultDom>()
        .child(input)
        .child(button().text("Read Input").on_click({
            clone!(text);
            move |_, _| text.set(input_handle.dom_element().value())
        }))
        .text(Sig(text.signal_cloned()));
}

pub fn static_text() {
    div::<DefaultDom>().text("Hello, world!");
}

pub fn dynamic_text() {
    let text = Mutable::new("Hello, world!");
    div::<DefaultDom>().text(Sig(text.signal()));
}

pub fn static_child() {
    div::<DefaultDom>()
        .child(p().text("Hello,"))
        .child(p().text("world!"));
}

pub fn dynamic_child() {
    let text = Mutable::new("Hello, world!");
    div::<DefaultDom>().child(Sig(text.signal().map(|text| div().text(text))));
}

pub fn static_optional_child() {
    div::<DefaultDom>().optional_child(Some(p().text("Hello, world!")));
}

pub fn dynamic_optional_child() {
    let text = Mutable::new("hello");
    div::<DefaultDom>().optional_child(Sig(text.signal().map(|text| {
        if text.is_empty() {
            None
        } else {
            Some(div().text(text))
        }
    })));
}

pub fn children() {
    div::<DefaultDom>().children([p().text("Hello,"), p().text("world!")]);
}
