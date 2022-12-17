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
    private::{self, DomElement, DomText, InstantiableDomElement, InstantiableDomNode},
    wet::{WetElement, WetNode, WetText},
    Dom, InstantiableDom,
};
use crate::{
    hydration::{remove_following_siblings, HydrationStats},
    node::element::Namespace,
};

pub struct Hydro;

impl Dom for Hydro {}

impl private::Dom for Hydro {
    type Element = HydroElement;
    type Node = HydroNode;
    type Text = HydroText;
}

impl InstantiableDom for Hydro {}

impl private::InstantiableDom for Hydro {
    type InstantiableElement = HydroElement;
    type InstantiableNode = HydroNode;
}

// TODO: Come up with better names than wet and dry. "Fresh" for wet? "Dry"
// represents either wet or dry really.
#[derive(Clone)]
pub struct HydroElement(Rc<RefCell<SharedHydroElement>>);

impl fmt::Display for HydroElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.borrow() {
            SharedHydroElement::Dry(dry) => {
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
            SharedHydroElement::Wet(wet) => f.write_str(&wet.dom_element().outer_html())?,
            SharedHydroElement::Unreachable => (),
        }

        Ok(())
    }
}

const NO_CLOSING_TAG: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "keygen", "link", "meta", "param",
    "source", "track", "wbr",
];

impl HydroElement {
    fn from_shared(shared: SharedHydroElement) -> Self {
        Self(Rc::new(RefCell::new(shared)))
    }

    fn first_child(&self) -> HydroNode {
        match &*self.borrow() {
            SharedHydroElement::Dry(dry) => dry.children.first().unwrap().clone(),
            SharedHydroElement::Wet(wet) => HydroNode::Wet(WetNode::from(wet.clone()).first_child()),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn next_sibling(&self) -> HydroNode {
        match &*self.borrow() {
            SharedHydroElement::Dry(dry) => {
                dry.next_sibling.as_ref().expect("No more siblings").clone()
            }
            SharedHydroElement::Wet(wet) => HydroNode::Wet(WetNode::from(wet.clone()).next_sibling()),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn set_next_sibling(&self, next_sibling: Option<HydroNode>) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.next_sibling = next_sibling,
            SharedHydroElement::Wet(_) => (),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn hydrate_child(
        self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut HydrationStats,
    ) -> WetNode {
        let wet = match self.0.replace(SharedHydroElement::Unreachable) {
            SharedHydroElement::Dry(dry) => dry.hydrate_child(parent, child, tracker),
            SharedHydroElement::Wet(wet) => wet,
            SharedHydroElement::Unreachable => unreachable!(),
        };

        self.0.replace(SharedHydroElement::Wet(wet.clone()));
        wet.into()
    }

    fn borrow(&self) -> Ref<SharedHydroElement> {
        self.0.as_ref().borrow()
    }

    fn borrow_mut(&self) -> RefMut<SharedHydroElement> {
        self.0.as_ref().borrow_mut()
    }
}

// TODO: Renaming: Hydro should be hydratable node/text/element. Dry should be a
// dry element (so `DryElementData` renames to `DryElement`)
enum SharedHydroElement {
    /// Box is used to keep the enum variant small
    Dry(Box<SharedDryElement>),
    Wet(WetElement),
    /// Used only for swapping from `Dry` to `Wet`
    Unreachable,
}

// TODO: Parameterize `children` and `next_sibling` types and make this form the
// basis of a dry dom?
struct SharedDryElement {
    namespace: Namespace,
    tag: String,
    attributes: IndexMap<String, String>,
    children: Vec<HydroNode>,
    hydrate_actions: Vec<LazyElementAction>,
    next_sibling: Option<HydroNode>,
}

impl SharedDryElement {
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

    fn hydrate_with_new(parent: &web_sys::Element, child: HydroNode, tracker: &mut HydrationStats) {
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

impl From<SharedDryElement> for WetElement {
    fn from(dry: SharedDryElement) -> Self {
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

impl DomElement for HydroElement {
    type Node = HydroNode;

    fn new(namespace: Namespace, tag: &str) -> Self {
        Self::from_shared(SharedHydroElement::Dry(Box::new(SharedDryElement {
            namespace,
            tag: tag.to_owned(),
            attributes: IndexMap::new(),
            children: Vec::new(),
            hydrate_actions: Vec::new(),
            next_sibling: None,
        })))
    }

    fn append_child(&mut self, child: &HydroNode) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => {
                if let Some(last) = dry.children.last_mut() {
                    last.set_next_sibling(Some(child));
                }

                dry.children.push(child.clone());
            }
            SharedHydroElement::Wet(wet) => {
                wet.append_child(&child.wet());
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn insert_child_before(&mut self, index: usize, child: &HydroNode, next_child: Option<&HydroNode>) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => {
                if index > 0 {
                    dry.children[index - 1].set_next_sibling(Some(child));
                }

                child.set_next_sibling(next_child);

                dry.children.insert(index, child.clone());
            }
            SharedHydroElement::Wet(wet) => {
                wet.insert_child_before(index, &child.wet(), next_child.map(|c| c.wet()).as_ref());
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn replace_child(&mut self, index: usize, new_child: &HydroNode, old_child: &HydroNode) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => {
                old_child.set_next_sibling(None);

                if index > 0 {
                    dry.children[index - 1].set_next_sibling(Some(new_child));
                }

                new_child.set_next_sibling(dry.children.get(index + 1));

                dry.children[index] = new_child.clone();
            }
            SharedHydroElement::Wet(wet) => {
                wet.replace_child(index, &new_child.wet(), &old_child.wet());
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn remove_child(&mut self, index: usize, child: &HydroNode) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => {
                child.set_next_sibling(None);
                if index > 0 {
                    dry.children[index - 1].set_next_sibling(dry.children.get(index + 1));
                }

                dry.children.remove(index);
            }
            SharedHydroElement::Wet(wet) => {
                wet.remove_child(index, &child.wet());
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn clear_children(&mut self) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => {
                for child in &dry.children {
                    child.set_next_sibling(None);
                }

                dry.children.clear();
            }
            SharedHydroElement::Wet(wet) => {
                wet.clear_children();
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn attach_shadow_children(&self, children: impl IntoIterator<Item = Self::Node>) {
        // TODO: We need to support shadow root in dry nodes, we just don't print it yet
        // (as there's no way to).
        match &*self.borrow() {
            SharedHydroElement::Dry(_) => todo!(),
            SharedHydroElement::Wet(wet) => {
                wet.attach_shadow_children(children.into_iter().map(Self::Node::into))
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn add_class(&mut self, name: &str) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => {
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
            SharedHydroElement::Wet(wet) => wet.add_class(name),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn remove_class(&mut self, name: &str) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => {
                if let Some(class) = dry.attributes.get_mut("class") {
                    *class = class
                        .split_ascii_whitespace()
                        .filter(|&c| c != name)
                        .join(" ");
                }
            }
            SharedHydroElement::Wet(wet) => wet.remove_class(name),
            SharedHydroElement::Unreachable => unreachable!(),
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
            SharedHydroElement::Dry(dry) => {
                if let Some(value) = value.text() {
                    dry.attributes.insert(name.to_owned(), value.into_owned());
                } else {
                    dry.attributes.remove(name);
                }
            }
            SharedHydroElement::Wet(wet) => wet.attribute(name, value),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry
                .hydrate_actions
                .push(Box::new(move |element| element.on(name, f))),
            SharedHydroElement::Wet(wet) => wet.on(name, f),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn try_dom_element(&self) -> Option<web_sys::Element> {
        match &*self.borrow_mut() {
            SharedHydroElement::Dry(_) => None,
            SharedHydroElement::Wet(wet) => wet.try_dom_element(),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry
                .hydrate_actions
                .push(Box::new(move |element| element.effect(f))),
            SharedHydroElement::Wet(wet) => wet.effect(f),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }
}

impl InstantiableDomElement for HydroElement {
    fn clone_node(&self) -> Self {
        Self::from_shared(match &*self.borrow() {
            SharedHydroElement::Dry(dry) => {
                let children = dry.children.clone();

                for (index, child) in children.iter().enumerate() {
                    child.set_next_sibling(children.get(index + 1));
                }

                SharedHydroElement::Dry(Box::new(SharedDryElement {
                    namespace: dry.namespace,
                    tag: dry.tag.clone(),
                    attributes: dry.attributes.clone(),
                    children,
                    hydrate_actions: Vec::new(),
                    next_sibling: None,
                }))
            }
            SharedHydroElement::Wet(wet) => SharedHydroElement::Wet(wet.clone_node()),
            SharedHydroElement::Unreachable => unreachable!(),
        })
    }
}

impl From<HydroElement> for WetNode {
    fn from(elem: HydroElement) -> Self {
        let wet = match elem.0.replace(SharedHydroElement::Unreachable) {
            SharedHydroElement::Dry(dry) => (*dry).into(),
            SharedHydroElement::Wet(wet) => wet,
            SharedHydroElement::Unreachable => unreachable!(),
        };

        elem.0.replace(SharedHydroElement::Wet(wet.clone()));
        wet.into()
    }
}

#[derive(Clone)]
pub struct HydroText(Rc<RefCell<SharedHydroText>>);

impl HydroText {
    fn borrow(&self) -> Ref<SharedHydroText> {
        self.0.as_ref().borrow()
    }

    fn borrow_mut(&self) -> RefMut<SharedHydroText> {
        self.0.as_ref().borrow_mut()
    }

    fn next_sibling(&self) -> HydroNode {
        match &*self.borrow() {
            SharedHydroText::Dry { next_sibling, .. } => {
                next_sibling.as_ref().expect("No more siblings").clone()
            }
            SharedHydroText::Wet(wet) => HydroNode::Wet(WetNode::from(wet.clone()).next_sibling()),
            SharedHydroText::Unreachable => unreachable!(),
        }
    }

    fn set_next_sibling(&self, new_next_sibling: Option<HydroNode>) {
        match &mut *self.borrow_mut() {
            SharedHydroText::Dry { next_sibling, .. } => *next_sibling = new_next_sibling,
            SharedHydroText::Wet(_) => (),
            SharedHydroText::Unreachable => unreachable!(),
        }
    }

    fn hydrate_child(
        self,
        parent: &web_sys::Node,
        child: &web_sys::Node,
        tracker: &mut HydrationStats,
    ) -> WetNode {
        let wet = match self.0.replace(SharedHydroText::Unreachable) {
            SharedHydroText::Dry { text, .. } => {
                Self::hydrate_child_text(text, child, parent, tracker)
            }
            SharedHydroText::Wet(wet) => wet,
            SharedHydroText::Unreachable => unreachable!(),
        };

        self.0.replace(SharedHydroText::Wet(wet.clone()));

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

enum SharedHydroText {
    // TODO: Factor out SharedDryText
    Dry {
        text: String,
        next_sibling: Option<HydroNode>,
    },
    Wet(WetText),
    /// Used for swapping `Dry` for `Wet`
    Unreachable,
}

impl DomText for HydroText {
    fn new(text: &str) -> Self {
        Self(Rc::new(RefCell::new(SharedHydroText::Dry {
            text: text.to_owned(),
            next_sibling: None,
        })))
    }

    fn set_text(&mut self, new_text: &str) {
        match &mut *self.borrow_mut() {
            SharedHydroText::Dry { text, .. } => *text = new_text.to_string(),
            SharedHydroText::Wet(wet) => wet.set_text(new_text),
            SharedHydroText::Unreachable => unreachable!(),
        }
    }
}

impl fmt::Display for HydroText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.borrow() {
            SharedHydroText::Dry { text, .. } => f.write_str(text),
            SharedHydroText::Wet(wet) => f.write_str(&wet.text()),
            SharedHydroText::Unreachable => unreachable!(),
        }
    }
}

impl From<HydroText> for WetNode {
    fn from(text: HydroText) -> Self {
        let wet = match text.0.replace(SharedHydroText::Unreachable) {
            SharedHydroText::Dry { text, .. } => WetText::new(&text),
            SharedHydroText::Wet(wet) => wet,
            SharedHydroText::Unreachable => unreachable!(),
        };

        text.0.replace(SharedHydroText::Wet(wet.clone()));
        wet.into()
    }
}

#[derive(Clone)]
pub enum HydroNode {
    Text(HydroText),
    Element(HydroElement),
    Wet(WetNode),
}

impl HydroNode {
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

    fn set_next_sibling(&self, next_sibling: Option<&HydroNode>) {
        let next_sibling = next_sibling.map(HydroNode::clone);

        match self {
            Self::Text(text) => text.set_next_sibling(next_sibling),
            Self::Element(element) => element.set_next_sibling(next_sibling),
            Self::Wet(_) => (),
        }
    }
}

impl From<HydroNode> for WetNode {
    fn from(node: HydroNode) -> Self {
        match node {
            HydroNode::Text(text) => text.into(),
            HydroNode::Element(elem) => elem.into(),
            HydroNode::Wet(wet) => wet,
        }
    }
}

impl fmt::Display for HydroNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(text) => text.fmt(f),
            Self::Element(elem) => elem.fmt(f),
            Self::Wet(wet) => wet.fmt(f),
        }
    }
}

impl InstantiableDomNode for HydroNode {
    type DomType = Hydro;

    fn into_element(self) -> HydroElement {
        match self {
            Self::Element(element) => element,
            Self::Text(_) => panic!("Type is `Text`, not `Element`"),
            Self::Wet(node) => HydroElement::from_shared(SharedHydroElement::Wet(node.into_element())),
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

impl From<HydroElement> for HydroNode {
    fn from(element: HydroElement) -> Self {
        Self::Element(element)
    }
}

impl From<HydroText> for HydroNode {
    fn from(text: HydroText) -> Self {
        Self::Text(text)
    }
}
