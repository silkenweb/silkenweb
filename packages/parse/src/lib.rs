// TODO: Test server side
// TODO: Write a proc macro to generate code from a string
// TODO: Doc

use silkenweb::{
    cfg_browser,
    dom::Dom,
    node::{
        element::{Element, GenericElement, Namespace, ParentElement},
        Node, Text,
    },
};

enum DomNode {
    Element {
        ns: Namespace,
        name: String,
        attributes: Vec<(String, String)>,
        children: Vec<Self>,
    },
    Text(String),
}

// TODO: Doc
pub fn html_to_nodes<D: Dom>(html: &str) -> Vec<Node<D>> {
    tree_to_nodes(arch::parse_html(html))
}

fn tree_to_nodes<D: Dom>(nodes: Vec<DomNode>) -> Vec<Node<D>> {
    nodes
        .into_iter()
        .map(|src_node| match src_node {
            DomNode::Element {
                ns,
                name,
                attributes,
                children,
            } => {
                let mut elem = GenericElement::new(&ns, &name);

                for (name, value) in attributes {
                    elem = elem.attribute(&name, value);
                }

                elem.children(tree_to_nodes(children)).into()
            }
            DomNode::Text(text) => Text::new(&text).into(),
        })
        .collect()
}

#[cfg_browser(false)]
mod arch {
    use ego_tree::NodeRef;
    use scraper::Html;
    use silkenweb::node::element::Namespace;

    use crate::DomNode;

    pub fn parse_html(html: &str) -> Vec<DomNode> {
        let fragment = Html::parse_fragment(html);
        tree_to_nodes(&fragment.root_element())
    }

    fn tree_to_nodes(src_elem: &NodeRef<scraper::node::Node>) -> Vec<DomNode> {
        src_elem
            .children()
            .filter_map(|src_node| {
                if let Some(child) = src_node.value().as_element() {
                    let ns = Namespace::Other(child.name.ns.to_string());
                    let name = child.name.local.to_string();
                    let attributes = child
                        .attrs()
                        .map(|(key, value)| (key.to_string(), value.to_string()))
                        .collect();
                    let children = tree_to_nodes(&src_node);

                    Some(DomNode::Element {
                        ns,
                        name,
                        attributes,
                        children,
                    })
                } else {
                    src_node
                        .value()
                        .as_text()
                        .map(|text| DomNode::Text(text.to_string()))
                }
            })
            .collect()
    }
}

#[cfg_browser(true)]
mod arch {
    use silkenweb::{
        dom::Wet,
        node::element::{Element, GenericElement, Namespace},
    };
    use wasm_bindgen::JsCast;

    use crate::DomNode;

    pub fn parse_html(html: &str) -> Vec<DomNode> {
        let tmpl = GenericElement::<Wet>::new(&Namespace::Html, "template");
        let tmpl_elem = tmpl
            .handle()
            .dom_element()
            .dyn_into::<web_sys::HtmlTemplateElement>()
            .unwrap();
        tmpl_elem.set_inner_html(html);
        first_child_to_nodes(tmpl_elem.content().first_child())
    }

    fn first_child_to_nodes(mut child: Option<web_sys::Node>) -> Vec<DomNode> {
        let mut nodes = Vec::new();

        while let Some(current) = child {
            if let Some(src_elem) = current.dyn_ref::<web_sys::Element>() {
                let ns = Namespace::Other(src_elem.namespace_uri().unwrap_or_default());
                let name = src_elem.local_name();

                let src_attributes = src_elem.attributes();
                let mut attributes = Vec::new();

                for item_index in 0.. {
                    if let Some(attr) = src_attributes.item(item_index) {
                        attributes.push((attr.name(), attr.value()));
                    } else {
                        break;
                    }
                }

                let children = first_child_to_nodes(src_elem.first_child());

                nodes.push(DomNode::Element {
                    ns,
                    name,
                    attributes,
                    children,
                });
            } else if let Some(text) = current.dyn_ref::<web_sys::Text>() {
                nodes.push(DomNode::Text(
                    text.text_content()
                        .as_deref()
                        .unwrap_or_default()
                        .to_string(),
                ));
            }

            child = current.next_sibling();
        }

        nodes
    }
}
