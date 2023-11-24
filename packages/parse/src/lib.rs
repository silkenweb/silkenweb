// TODO: Test server side

// TODO: Factor out a trait to traverse the tree, creating nodes, and write a
// proc macro to generate code from a string

// TODO: Doc

use silkenweb::{cfg_browser, dom::Dom, node::Node};

// TODO: Doc
pub fn html_to_nodes<D: Dom>(html: &str) -> Vec<Node<D>> {
    arch::html_to_nodes(html)
}

#[cfg_browser(false)]
mod arch {
    use ego_tree::NodeRef;
    use scraper::Html;
    use silkenweb::{
        dom::Dom,
        node::{
            element::{Element, GenericElement, Namespace, ParentElement},
            Node, Text,
        },
    };

    pub fn html_to_nodes<D: Dom>(html: &str) -> Vec<Node<D>> {
        let fragment = Html::parse_fragment(html);
        tree_to_nodes(&fragment.root_element())
    }

    fn tree_to_nodes<D: Dom>(src_elem: &NodeRef<scraper::node::Node>) -> Vec<Node<D>> {
        src_elem
            .children()
            .filter_map(|src_node| {
                if let Some(child) = src_node.value().as_element() {
                    let mut elem = GenericElement::new(
                        &Namespace::Other(child.name.ns.to_string()),
                        child.name.local.as_ref(),
                    );

                    for (name, value) in child.attrs() {
                        elem = elem.attribute(name, value);
                    }

                    let children = tree_to_nodes(&src_node);
                    Some(elem.children(children).into())
                } else {
                    src_node
                        .value()
                        .as_text()
                        .map(|text| Text::new(text).into())
                }
            })
            .collect()
    }
}

#[cfg_browser(true)]
mod arch {
    use silkenweb::{
        dom::{Dom, Wet},
        node::{
            element::{Element, GenericElement, Namespace, ParentElement},
            Node, Text,
        },
    };
    use wasm_bindgen::JsCast;

    pub fn html_to_nodes<D: Dom>(html: &str) -> Vec<Node<D>> {
        let tmpl = GenericElement::<Wet>::new(&Namespace::Html, "template");
        let tmpl_elem = tmpl
            .handle()
            .dom_element()
            .dyn_into::<web_sys::HtmlTemplateElement>()
            .unwrap();
        tmpl_elem.set_inner_html(html);
        first_child_to_nodes(tmpl_elem.content().first_child())
    }

    fn first_child_to_nodes<D: Dom>(mut child: Option<web_sys::Node>) -> Vec<Node<D>> {
        let mut nodes = Vec::new();

        while let Some(current) = child {
            if let Some(src_elem) = current.dyn_ref::<web_sys::Element>() {
                let ns = src_elem.namespace_uri().unwrap_or_default();
                let mut elem = GenericElement::new(&Namespace::Other(ns), &src_elem.tag_name());
                let attributes = src_elem.attributes();

                for item_index in 0.. {
                    if let Some(attr) = attributes.item(item_index) {
                        elem = elem.attribute(&attr.name(), attr.value());
                    } else {
                        break;
                    }
                }

                nodes.push(
                    elem.children(first_child_to_nodes(src_elem.first_child()))
                        .into(),
                );
            } else if let Some(text) = current.dyn_ref::<web_sys::Text>() {
                nodes.push(Text::new(text.text_content().as_deref().unwrap_or_default()).into());
            }

            child = current.next_sibling();
        }

        nodes
    }
}
