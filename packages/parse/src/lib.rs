//! # Parse HTML Fragments
//!
//! This module provides tools for parsing HTML fragments. An HTML fragment is a
//! (possibly empty) list of HTML text or element nodes. For example:
//!
//! ```html
//! <p>This is an HTML fragment</p>
//! ```
//!
//! - Parsing on the browser is done using `Element::innerHtml`. On the server
//!   it uses [`scraper`].
//! - If any errors are present, a best effort is made to parse the HTML.
//! - Any empty text nodes are removed.
//! - Attributes are sorted to make the result more testable.
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
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
        ns: String,
        name: String,
        attributes: Vec<(String, String)>,
        children: Vec<Self>,
    },
    Text(String),
}

/// Convert an HTML fragment to Silkenweb nodes.
///
/// See the [module][`self`] documentation for details.
///
/// # Example
///
/// ```
/// # use silkenweb_parse::html_to_nodes;
/// # use silkenweb::node::Node;
/// let html_fragment = "<p>Node 1</p><p>Node 2</p>";
/// let nodes: Vec<Node> = html_to_nodes(html_fragment);
///
/// assert_eq!(format!("{}{}", nodes[0], nodes[1]), html_fragment);
/// ```
pub fn html_to_nodes<D: Dom>(html: &str) -> Vec<Node<D>> {
    tree_to_nodes(arch::parse_html(html))
}

fn tree_to_nodes<D: Dom>(nodes: Vec<DomNode>) -> Vec<Node<D>> {
    nodes
        .into_iter()
        .filter_map(|src_node| match src_node {
            DomNode::Element {
                ns,
                name,
                mut attributes,
                children,
            } => {
                // Sort attributes for testability
                attributes.sort();

                let mut elem = GenericElement::new(&Namespace::Other(ns), &name);

                for (name, value) in attributes {
                    elem = elem.attribute(&name, value);
                }

                Some(elem.children(tree_to_nodes(children)).into())
            }
            DomNode::Text(text) => {
                if text.trim().is_empty() {
                    None
                } else {
                    Some(Text::new(&text).into())
                }
            }
        })
        .collect()
}

/// Convert an HTML fragment to Silkenweb node expressions.
///
/// This is for writing your own proc macros to parse HTML fragments at compile
/// time. See the [module][`self`] documentation for details about the parsing.
///
/// The resulting [`TokenStream`]s are expressions with type
/// [`Node`][`silkenweb::node::Node`]. The [`Dom`][`silkenweb::dom::Dom`] type
/// is left unspecified, so may need to be specified if it can't be determined
/// with type inference.
pub fn html_to_tokens(dom_type: TokenStream, html: &str) -> Vec<TokenStream> {
    tree_to_tokens(dom_type.into(), arch::parse_html(html))
}

fn tree_to_tokens(dom_type: proc_macro2::TokenStream, nodes: Vec<DomNode>) -> Vec<TokenStream> {
    nodes
        .into_iter()
        .filter_map(|src_node| match src_node {
            DomNode::Element {
                ns,
                name,
                mut attributes,
                children,
            } => {
                // Sort attributes for testability
                attributes.sort();

                let children =
                    tree_to_tokens(dom_type.clone(), children).into_iter().map(proc_macro2::TokenStream::from);
                let attributes = attributes.into_iter().map(|(name, value)| {
                    quote! { elem = elem.attribute(#name, #value); }
                });

                let children = if children.len() == 0 {
                    quote! {[] as [::silkenweb::node::Node<#dom_type>; 0]}
                } else {
                    quote! {[#(#children),*]}
                };

                Some(quote! {{
                    use ::silkenweb::node::element::{Element, GenericElement, Namespace, ParentElement};
                    let mut elem = GenericElement::<#dom_type>::new(
                        &Namespace::Other(#ns.to_string()),
                        #name
                    );

                    #(#attributes)*

                    elem.children(#children)
                }})
            }
            DomNode::Text(text) => {
                if text.trim().is_empty() {
                    None
                } else {
                    Some(quote!(::silkenweb::node::Text::<#dom_type>::new(#text)))
                }
            }
        })
        .map(|node| quote!(::silkenweb::node::Node::<#dom_type>::from(#node)).into())
        .collect::<Vec<_>>()
}

#[cfg_browser(false)]
mod arch {
    use ego_tree::NodeRef;
    use scraper::Html;

    use crate::DomNode;

    pub(super) fn parse_html(html: &str) -> Vec<DomNode> {
        let fragment = Html::parse_fragment(html);
        tree_to_nodes(&fragment.root_element())
    }

    fn tree_to_nodes(src_elem: &NodeRef<scraper::node::Node>) -> Vec<DomNode> {
        src_elem
            .children()
            .filter_map(|src_node| {
                if let Some(child) = src_node.value().as_element() {
                    let ns = child.name.ns.to_string();
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

    pub(super) fn parse_html(html: &str) -> Vec<DomNode> {
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
                let ns = src_elem.namespace_uri().unwrap_or_default();
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
