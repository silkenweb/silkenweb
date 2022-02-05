use std::fmt;

use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::{insert_component, mount_point, node::element::Element, unmount};

pub(super) mod lazy;
pub(super) mod node;

#[derive(Default)]
pub struct HydrationStats {
    nodes_added: u64,
    nodes_removed: u64,
    empty_text_removed: u64,
    attributes_set: u64,
    attributes_removed: u64,
}

impl HydrationStats {
    pub fn only_whitespace_diffs(&self) -> bool {
        self.nodes_added == 0
            && self.nodes_removed == 0
            && self.attributes_set == 0
            && self.attributes_removed == 0
    }

    pub fn exact_match(&self) -> bool {
        self.empty_text_removed == 0 && self.only_whitespace_diffs()
    }

    fn node_added(&mut self, _elem: &web_sys::Node) {
        self.nodes_added += 1;
    }

    fn node_removed(&mut self, node: &web_sys::Node) {
        match node
            .dyn_ref::<web_sys::Text>()
            .and_then(|t| t.text_content())
        {
            Some(text) if text.trim().is_empty() => self.empty_text_removed += 1,
            _ => self.nodes_removed += 1,
        }
    }

    fn attribute_set(&mut self, _elem: &web_sys::Element, _name: &str, _value: &str) {
        self.attributes_set += 1;
    }

    fn attribute_removed(&mut self, _elem: &web_sys::Element, _name: &str) {
        self.attributes_removed += 1;
    }
}

impl fmt::Display for HydrationStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Hydration stats:")?;
        writeln!(f, "    nodes added = {}", self.nodes_added)?;
        writeln!(f, "    nodes removed = {}", self.nodes_removed)?;
        writeln!(f, "    empty text removed = {}", self.empty_text_removed)?;
        writeln!(f, "    attributes set = {}", self.attributes_set)?;
        writeln!(f, "    attributes removed = {}", self.attributes_removed)
    }
}

pub async fn hydrate(id: &str, elem: impl Into<Element>) -> HydrationStats {
    let elem = elem.into();
    let mut stats = HydrationStats::default();

    unmount(id);

    let mount_point = mount_point(id);

    if let Some(hydration_point) = mount_point.first_child() {
        let node: web_sys::Node = elem
            .hydrate_child(&mount_point, &hydration_point, &mut stats)
            .into();

        remove_following_siblings(&mount_point, node.next_sibling());
    } else {
        let new_elem = elem.eval_dom_element();
        stats.node_added(&new_elem);
        mount_point.append_child(&new_elem).unwrap_throw();
    }

    insert_component(id, elem);

    stats
}

/// Remove `child` and all siblings after `child`
fn remove_following_siblings(parent: &web_sys::Node, mut child: Option<web_sys::Node>) {
    while let Some(node) = child {
        let next_child = node.next_sibling();
        parent.remove_child(&node).unwrap_throw();
        child = next_child;
    }
}
