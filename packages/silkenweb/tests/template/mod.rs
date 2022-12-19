use silkenweb::{
    elements::html::{div, DivTemplate},
    node::Node,
    prelude::ParentElement,
};

use super::PlatformDom;

isomorphic_test! {
    async fn template_text() {
        let template: DivTemplate<PlatformDom, String> = div().on_instantiate(|div, s| div.text(s)).freeze();
        let node: Node<PlatformDom> = template.instantiate(&"Hello, world!".to_string()).into();
        assert_eq!(node.to_string(), r#"<div>Hello, world!</div>"#);
        let node: Node<PlatformDom> = template.instantiate(&"Goodbye!".to_string()).into();
        assert_eq!(node.to_string(), r#"<div>Goodbye!</div>"#);
    }
}
