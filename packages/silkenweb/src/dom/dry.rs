use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    fmt,
    rc::Rc,
};

use caseless::default_caseless_match_str;
use html_escape::encode_double_quoted_attribute;
use indexmap::IndexMap;
use itertools::Itertools;
use silkenweb_base::clone;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use super::{
    wet::{WetElement, WetNode, WetText},
    Dom, DomElement, DomText, InstantiableDom, InstantiableDomElement, InstantiableDomNode,
};
use crate::{
    hydration::{remove_following_siblings, HydrationStats},
    node::element::Namespace,
};

pub struct Dry;

impl Dom for Dry {
    type Element = DryElement;
    type Node = DryNode;
    type Text = DryText;
}

impl InstantiableDom for Dry {
    type InstantiableElement = DryElement;
    type InstantiableNode = DryNode;
}

// TODO: Come up with better names than wet and dry. "Fresh" for wet? "Dry"
// represents either wet or dry really.
#[derive(Clone)]
pub struct DryElement(Rc<RefCell<SharedDryElement>>);

impl fmt::Display for DryElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.borrow() {
            SharedDryElement::Dry(dry) => {
                write!(f, "<{}", dry.tag)?;

                for (name, value) in &dry.attributes {
                    write!(f, " {}=\"{}\"", name, encode_double_quoted_attribute(value))?;
                }

                f.write_str(">")?;

                for child in &dry.children {
                    child.fmt(f)?;
                }

                let has_children = !dry.children.is_empty();
                let requires_closing_tag = !NO_CLOSING_TAG.contains(&dry.tag.as_str());

                if requires_closing_tag || has_children {
                    write!(f, "</{}>", dry.tag)?;
                }
            }
            SharedDryElement::Wet(wet) => f.write_str(&wet.dom_element().outer_html())?,
            SharedDryElement::Unreachable => (),
        }

        Ok(())
    }
}

const NO_CLOSING_TAG: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "keygen", "link", "meta", "param",
    "source", "track", "wbr",
];

impl DryElement {
    fn from_shared(shared: SharedDryElement) -> Self {
        Self(Rc::new(RefCell::new(shared)))
    }

    fn first_child(&self) -> DryNode {
        match &*self.borrow() {
            SharedDryElement::Dry(dry) => dry.children.first().unwrap().clone(),
            SharedDryElement::Wet(wet) => DryNode::Wet(WetNode::from(wet.clone()).first_child()),
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn next_sibling(&self) -> DryNode {
        match &*self.borrow() {
            SharedDryElement::Dry(dry) => {
                dry.next_sibling.as_ref().expect("No more siblings").clone()
            }
            SharedDryElement::Wet(wet) => DryNode::Wet(WetNode::from(wet.clone()).next_sibling()),
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn set_next_sibling(&self, next_sibling: Option<DryNode>) {
        match &mut *self.borrow_mut() {
            SharedDryElement::Dry(dry) => dry.next_sibling = next_sibling,
            SharedDryElement::Wet(_) => (),
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn hydrate_child(
        self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut HydrationStats,
    ) -> WetNode {
        let wet = match self.0.replace(SharedDryElement::Unreachable) {
            SharedDryElement::Dry(dry) => dry.hydrate_child(parent, child, tracker),
            SharedDryElement::Wet(wet) => wet,
            SharedDryElement::Unreachable => unreachable!(),
        };

        self.0.replace(SharedDryElement::Wet(wet.clone()));
        wet.into()
    }

    fn borrow(&self) -> Ref<SharedDryElement> {
        self.0.as_ref().borrow()
    }

    fn borrow_mut(&self) -> RefMut<SharedDryElement> {
        self.0.as_ref().borrow_mut()
    }
}

// TODO: Renaming: Hydro should be hydratable node/text/element. Dry should be a
// dry element (so `DryElementData` renames to `DryElement`)
enum SharedDryElement {
    /// Box is used to keep the enum variant small
    Dry(Box<DryElementData>),
    Wet(WetElement),
    /// Used only for swapping from `Dry` to `Wet`
    Unreachable,
}

// TODO: Parameterize `children` and `next_sibling` types and make this form the
// basis of a dry dom?
struct DryElementData {
    namespace: Namespace,
    tag: String,
    attributes: IndexMap<String, String>,
    children: Vec<DryNode>,
    hydrate_actions: Vec<LazyElementAction>,
    next_sibling: Option<DryNode>,
}

impl DryElementData {
    fn hydrate_child(
        self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut HydrationStats,
    ) -> WetElement {
        clone!(mut child);

        loop {
            if let Some(elem_child) = child.dyn_ref::<web_sys::Element>() {
                let dom_namespace = elem_child.namespace_uri().unwrap_or_default();
                let dry_namespace = self.namespace.as_str();

                if dry_namespace == dom_namespace
                    && default_caseless_match_str(&elem_child.tag_name(), &self.tag)
                {
                    return self.hydrate_element(elem_child, tracker);
                }
            }

            let next = child.next_sibling();
            tracker.node_removed(&child);
            parent.remove_child(&child).unwrap_throw();

            if let Some(next_child) = next {
                child = next_child;
            } else {
                break;
            }
        }

        let wet_child: WetElement = self.into();
        let new_element = wet_child.dom_element();
        parent.append_child(&new_element).unwrap_throw();
        tracker.node_added(&new_element);

        wet_child
    }

    fn hydrate_element(
        self,
        dom_elem: &web_sys::Element,
        tracker: &mut HydrationStats,
    ) -> WetElement {
        self.reconcile_attributes(dom_elem, tracker);
        let mut elem = WetElement::from_element(dom_elem.clone());
        let mut current_child = dom_elem.first_child();

        let mut children = self.children.into_iter();

        for child in children.by_ref() {
            if let Some(node) = &current_child {
                let hydrated_elem = child.hydrate_child(dom_elem, node, tracker);
                current_child = hydrated_elem.dom_node().next_sibling();
            } else {
                Self::hydrate_with_new(dom_elem, child, tracker);
                break;
            }
        }

        for child in children {
            Self::hydrate_with_new(dom_elem, child, tracker);
        }

        remove_following_siblings(dom_elem, current_child);

        for event in self.hydrate_actions {
            event(&mut elem);
        }

        elem
    }

    fn hydrate_with_new(parent: &web_sys::Element, child: DryNode, tracker: &mut HydrationStats) {
        let child = WetNode::from(child);
        let new_child = child.dom_node();
        parent.append_child(new_child).unwrap_throw();
        tracker.node_added(new_child);
    }

    fn reconcile_attributes(&self, dom_elem: &web_sys::Element, tracker: &mut HydrationStats) {
        let dom_attributes = dom_elem.attributes();
        let mut dom_attr_map = HashMap::new();

        for item_index in 0.. {
            if let Some(attr) = dom_attributes.item(item_index) {
                dom_attr_map.insert(attr.name(), attr.value());
            } else {
                break;
            }
        }

        for (name, value) in &self.attributes {
            let value = value.as_ref();

            let set_attr = if let Some(existing_value) = dom_attr_map.remove(name) {
                value != existing_value
            } else {
                true
            };

            if set_attr {
                dom_elem.set_attribute(name, value).unwrap_throw();
                tracker.attribute_set(dom_elem, name, value);
            }
        }

        for name in dom_attr_map.into_keys() {
            if !name.starts_with("data-silkenweb") {
                tracker.attribute_removed(dom_elem, &name);
                dom_elem.remove_attribute(&name).unwrap_throw();
            }
        }
    }
}

impl From<DryElementData> for WetElement {
    fn from(dry: DryElementData) -> Self {
        let mut wet = WetElement::new(dry.namespace, &dry.tag);

        for (name, value) in dry.attributes {
            wet.attribute(&name, value);
        }

        for child in dry.children {
            wet.append_child(&child.into());
        }

        for action in dry.hydrate_actions {
            action(&mut wet);
        }

        wet
    }
}

type LazyElementAction = Box<dyn FnOnce(&mut WetElement)>;

impl DomElement for DryElement {
    type Node = DryNode;

    fn new(namespace: Namespace, tag: &str) -> Self {
        Self::from_shared(SharedDryElement::Dry(Box::new(DryElementData {
            namespace,
            tag: tag.to_owned(),
            attributes: IndexMap::new(),
            children: Vec::new(),
            hydrate_actions: Vec::new(),
            next_sibling: None,
        })))
    }

    fn append_child(&mut self, child: &DryNode) {
        match &mut *self.borrow_mut() {
            SharedDryElement::Dry(dry) => {
                if let Some(last) = dry.children.last_mut() {
                    last.set_next_sibling(Some(child));
                }

                dry.children.push(child.clone());
            }
            SharedDryElement::Wet(wet) => {
                wet.append_child(&child.wet());
            }
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn insert_child_before(&mut self, index: usize, child: &DryNode, next_child: Option<&DryNode>) {
        match &mut *self.borrow_mut() {
            SharedDryElement::Dry(dry) => {
                if index > 0 {
                    dry.children[index - 1].set_next_sibling(Some(child));
                }

                child.set_next_sibling(next_child);

                dry.children.insert(index, child.clone());
            }
            SharedDryElement::Wet(wet) => {
                wet.insert_child_before(index, &child.wet(), next_child.map(|c| c.wet()).as_ref());
            }
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn replace_child(&mut self, index: usize, new_child: &DryNode, old_child: &DryNode) {
        match &mut *self.borrow_mut() {
            SharedDryElement::Dry(dry) => {
                old_child.set_next_sibling(None);

                if index > 0 {
                    dry.children[index - 1].set_next_sibling(Some(new_child));
                }

                new_child.set_next_sibling(dry.children.get(index + 1));

                dry.children[index] = new_child.clone();
            }
            SharedDryElement::Wet(wet) => {
                wet.replace_child(index, &new_child.wet(), &old_child.wet());
            }
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn remove_child(&mut self, index: usize, child: &DryNode) {
        match &mut *self.borrow_mut() {
            SharedDryElement::Dry(dry) => {
                child.set_next_sibling(None);
                if index > 0 {
                    dry.children[index - 1].set_next_sibling(dry.children.get(index + 1));
                }

                dry.children.remove(index);
            }
            SharedDryElement::Wet(wet) => {
                wet.remove_child(index, &child.wet());
            }
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn clear_children(&mut self) {
        match &mut *self.borrow_mut() {
            SharedDryElement::Dry(dry) => {
                for child in &dry.children {
                    child.set_next_sibling(None);
                }

                dry.children.clear();
            }
            SharedDryElement::Wet(wet) => {
                wet.clear_children();
            }
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn attach_shadow_children(&self, children: impl IntoIterator<Item = Self::Node>) {
        // TODO: We need to support shadow root in dry nodes, we just don't print it yet
        // (as there's no way to).
        match &*self.borrow() {
            SharedDryElement::Dry(_) => todo!(),
            SharedDryElement::Wet(wet) => {
                wet.attach_shadow_children(children.into_iter().map(Self::Node::into))
            }
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn add_class(&mut self, name: &str) {
        match &mut *self.borrow_mut() {
            SharedDryElement::Dry(dry) => {
                dry.attributes
                    .entry("class".to_owned())
                    .and_modify(|class| {
                        if !class.split_ascii_whitespace().any(|c| c == name) {
                            if !class.is_empty() {
                                class.push(' ');
                            }

                            class.push_str(name);
                        }
                    })
                    .or_insert_with(|| name.to_owned());
            }
            SharedDryElement::Wet(wet) => wet.add_class(name),
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn remove_class(&mut self, name: &str) {
        match &mut *self.borrow_mut() {
            SharedDryElement::Dry(dry) => {
                if let Some(class) = dry.attributes.get_mut("class") {
                    *class = class
                        .split_ascii_whitespace()
                        .filter(|&c| c != name)
                        .join(" ");
                }
            }
            SharedDryElement::Wet(wet) => wet.remove_class(name),
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn attribute<A>(&mut self, name: &str, value: A)
    where
        A: crate::attribute::Attribute,
    {
        assert_ne!(
            name, "xmlns",
            "\"xmlns\" must be set via a namespace at tag creation time"
        );

        match &mut *self.borrow_mut() {
            SharedDryElement::Dry(dry) => {
                if let Some(value) = value.text() {
                    dry.attributes.insert(name.to_owned(), value.into_owned());
                } else {
                    dry.attributes.remove(name);
                }
            }
            SharedDryElement::Wet(wet) => wet.attribute(name, value),
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        match &mut *self.borrow_mut() {
            SharedDryElement::Dry(dry) => dry
                .hydrate_actions
                .push(Box::new(move |element| element.on(name, f))),
            SharedDryElement::Wet(wet) => wet.on(name, f),
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn try_dom_element(&self) -> Option<web_sys::Element> {
        match &*self.borrow_mut() {
            SharedDryElement::Dry(_) => None,
            SharedDryElement::Wet(wet) => wet.try_dom_element(),
            SharedDryElement::Unreachable => unreachable!(),
        }
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        match &mut *self.borrow_mut() {
            SharedDryElement::Dry(dry) => dry
                .hydrate_actions
                .push(Box::new(move |element| element.effect(f))),
            SharedDryElement::Wet(wet) => wet.effect(f),
            SharedDryElement::Unreachable => unreachable!(),
        }
    }
}

impl InstantiableDomElement for DryElement {
    fn clone_node(&self) -> Self {
        Self::from_shared(match &*self.borrow() {
            SharedDryElement::Dry(dry) => {
                let children = dry.children.clone();

                for (index, child) in children.iter().enumerate() {
                    child.set_next_sibling(children.get(index + 1));
                }

                SharedDryElement::Dry(Box::new(DryElementData {
                    namespace: dry.namespace,
                    tag: dry.tag.clone(),
                    attributes: dry.attributes.clone(),
                    children,
                    hydrate_actions: Vec::new(),
                    next_sibling: None,
                }))
            }
            SharedDryElement::Wet(wet) => SharedDryElement::Wet(wet.clone_node()),
            SharedDryElement::Unreachable => unreachable!(),
        })
    }
}

impl From<DryElement> for WetNode {
    fn from(elem: DryElement) -> Self {
        let wet = match elem.0.replace(SharedDryElement::Unreachable) {
            SharedDryElement::Dry(dry) => (*dry).into(),
            SharedDryElement::Wet(wet) => wet,
            SharedDryElement::Unreachable => unreachable!(),
        };

        elem.0.replace(SharedDryElement::Wet(wet.clone()));
        wet.into()
    }
}

#[derive(Clone)]
pub struct DryText(Rc<RefCell<SharedDryText>>);

impl DryText {
    fn borrow(&self) -> Ref<SharedDryText> {
        self.0.as_ref().borrow()
    }

    fn borrow_mut(&self) -> RefMut<SharedDryText> {
        self.0.as_ref().borrow_mut()
    }

    fn next_sibling(&self) -> DryNode {
        match &*self.borrow() {
            SharedDryText::Dry { next_sibling, .. } => {
                next_sibling.as_ref().expect("No more siblings").clone()
            }
            SharedDryText::Wet(wet) => DryNode::Wet(WetNode::from(wet.clone()).next_sibling()),
            SharedDryText::Unreachable => unreachable!(),
        }
    }

    fn set_next_sibling(&self, new_next_sibling: Option<DryNode>) {
        match &mut *self.borrow_mut() {
            SharedDryText::Dry { next_sibling, .. } => *next_sibling = new_next_sibling,
            SharedDryText::Wet(_) => (),
            SharedDryText::Unreachable => unreachable!(),
        }
    }

    fn hydrate_child(
        self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut HydrationStats,
    ) -> WetNode {
        let wet = match self.0.replace(SharedDryText::Unreachable) {
            SharedDryText::Dry { text, .. } => {
                Self::hydrate_child_text(text, child, parent, tracker)
            }
            SharedDryText::Wet(wet) => wet,
            SharedDryText::Unreachable => unreachable!(),
        };

        self.0.replace(SharedDryText::Wet(wet.clone()));

        wet.into()
    }

    fn hydrate_child_text(
        text: String,
        child: &web_sys::Node,
        parent: &web_sys::Node,
        tracker: &mut HydrationStats,
    ) -> WetText {
        let matching_node =
            child
                .dyn_ref::<web_sys::Text>()
                .and_then(|dom_text| match dom_text.text_content() {
                    Some(current_text) if text == current_text => Some(dom_text),
                    None if text.is_empty() => Some(dom_text),
                    _ => None,
                });

        if let Some(dom_text) = matching_node {
            WetText::from_dom(dom_text.clone())
        } else {
            let new_text = WetText::new(&text);

            let dom_text = new_text.dom_text();
            parent.insert_before(dom_text, Some(child)).unwrap_throw();
            tracker.node_added(dom_text);

            new_text
        }
    }
}

enum SharedDryText {
    Dry {
        text: String,
        next_sibling: Option<DryNode>,
    },
    Wet(WetText),
    /// Used for swapping `Dry` for `Wet`
    Unreachable,
}

impl DomText for DryText {
    fn new(text: &str) -> Self {
        Self(Rc::new(RefCell::new(SharedDryText::Dry {
            text: text.to_owned(),
            next_sibling: None,
        })))
    }

    fn set_text(&mut self, new_text: &str) {
        match &mut *self.borrow_mut() {
            SharedDryText::Dry { text, .. } => *text = new_text.to_string(),
            SharedDryText::Wet(wet) => wet.set_text(new_text),
            SharedDryText::Unreachable => unreachable!(),
        }
    }
}

impl fmt::Display for DryText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.borrow() {
            SharedDryText::Dry { text, .. } => f.write_str(text),
            SharedDryText::Wet(wet) => f.write_str(&wet.text()),
            SharedDryText::Unreachable => unreachable!(),
        }
    }
}

impl From<DryText> for WetNode {
    fn from(text: DryText) -> Self {
        let wet = match text.0.replace(SharedDryText::Unreachable) {
            SharedDryText::Dry { text, .. } => WetText::new(&text),
            SharedDryText::Wet(wet) => wet,
            SharedDryText::Unreachable => unreachable!(),
        };

        text.0.replace(SharedDryText::Wet(wet.clone()));
        wet.into()
    }
}

#[derive(Clone)]
pub enum DryNode {
    Text(DryText),
    Element(DryElement),
    Wet(WetNode),
}

impl DryNode {
    pub fn wet(&self) -> WetNode {
        self.clone().into()
    }

    pub fn hydrate_child(
        self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut HydrationStats,
    ) -> WetNode {
        match self {
            Self::Text(text) => text.hydrate_child(parent, child, tracker),
            Self::Element(elem) => elem.hydrate_child(parent, child, tracker),
            Self::Wet(wet) => wet,
        }
    }

    fn set_next_sibling(&self, next_sibling: Option<&DryNode>) {
        let next_sibling = next_sibling.map(DryNode::clone);

        match self {
            Self::Text(text) => text.set_next_sibling(next_sibling),
            Self::Element(element) => element.set_next_sibling(next_sibling),
            Self::Wet(_) => (),
        }
    }
}

impl From<DryNode> for WetNode {
    fn from(node: DryNode) -> Self {
        match node {
            DryNode::Text(text) => text.into(),
            DryNode::Element(elem) => elem.into(),
            DryNode::Wet(wet) => wet,
        }
    }
}

impl fmt::Display for DryNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(text) => text.fmt(f),
            Self::Element(elem) => elem.fmt(f),
            Self::Wet(wet) => wet.fmt(f),
        }
    }
}

impl InstantiableDomNode for DryNode {
    type DomType = Dry;

    fn into_element(self) -> DryElement {
        match self {
            Self::Element(element) => element,
            Self::Text(_) => panic!("Type is `Text`, not `Element`"),
            Self::Wet(node) => DryElement::from_shared(SharedDryElement::Wet(node.into_element())),
        }
    }

    fn first_child(&self) -> Self {
        match self {
            Self::Text(_) => panic!("Text elements don't have children"),
            Self::Element(element) => element.first_child(),
            Self::Wet(wet) => Self::Wet(wet.first_child()),
        }
    }

    fn next_sibling(&self) -> Self {
        match self {
            Self::Text(text) => text.next_sibling(),
            Self::Element(element) => element.next_sibling(),
            Self::Wet(wet) => Self::Wet(wet.next_sibling()),
        }
    }
}

impl From<DryElement> for DryNode {
    fn from(element: DryElement) -> Self {
        Self::Element(element)
    }
}

impl From<DryText> for DryNode {
    fn from(text: DryText) -> Self {
        Self::Text(text)
    }
}
