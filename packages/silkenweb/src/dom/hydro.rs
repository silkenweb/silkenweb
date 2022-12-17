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
            SharedHydroElement::Dry(dry) => dry.fmt(f),
            SharedHydroElement::Wet(wet) => wet.fmt(f),
            SharedHydroElement::Unreachable => Ok(()),
        }
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
            SharedHydroElement::Dry(dry) => dry.first_child().unwrap().clone(),
            SharedHydroElement::Wet(wet) => {
                HydroNode::Wet(WetNode::from(wet.clone()).first_child())
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn next_sibling(&self) -> HydroNode {
        match &*self.borrow() {
            SharedHydroElement::Dry(dry) => dry.next_sibling().expect("No more siblings").clone(),
            SharedHydroElement::Wet(wet) => {
                HydroNode::Wet(WetNode::from(wet.clone()).next_sibling())
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn set_next_sibling(&self, next_sibling: Option<HydroNode>) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.set_next_sibling(next_sibling),
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
    Dry(Box<SharedDryElement<HydroNode>>),
    Wet(WetElement),
    /// Used only for swapping from `Dry` to `Wet`
    Unreachable,
}

// TODO: Parameterize `children` and `next_sibling` types and make this form the
// basis of a dry dom?
struct SharedDryElement<Child> {
    namespace: Namespace,
    tag: String,
    attributes: IndexMap<String, String>,
    children: Vec<Child>,
    hydrate_actions: Vec<LazyElementAction>,
    next_sibling: Option<Child>,
}

impl<Child: TrackSibling> SharedDryElement<Child> {
    fn new(namespace: Namespace, tag: &str) -> Self {
        Self {
            namespace,
            tag: tag.to_owned(),
            attributes: IndexMap::new(),
            children: Vec::new(),
            hydrate_actions: Vec::new(),
            next_sibling: None,
        }
    }

    fn first_child(&self) -> Option<&Child> {
        self.children.first()
    }

    fn next_sibling(&self) -> Option<&Child> {
        self.next_sibling.as_ref()
    }

    fn set_next_sibling(&mut self, next_sibling: Option<Child>) {
        self.next_sibling = next_sibling;
    }

    fn append_child(&mut self, child: &Child) {
        if let Some(last) = self.children.last_mut() {
            last.set_next_sibling(Some(child));
        }

        self.children.push(child.clone());
    }

    fn insert_child_before(&mut self, index: usize, child: &Child, next_child: Option<&Child>) {
        if index > 0 {
            self.children[index - 1].set_next_sibling(Some(child));
        }

        child.set_next_sibling(next_child);

        self.children.insert(index, child.clone());
    }

    fn replace_child(&mut self, index: usize, new_child: &Child, old_child: &Child) {
        old_child.set_next_sibling(None);

        if index > 0 {
            self.children[index - 1].set_next_sibling(Some(new_child));
        }

        new_child.set_next_sibling(self.children.get(index + 1));

        self.children[index] = new_child.clone();
    }

    fn remove_child(&mut self, index: usize, child: &Child) {
        child.set_next_sibling(None);
        if index > 0 {
            self.children[index - 1].set_next_sibling(self.children.get(index + 1));
        }

        self.children.remove(index);
    }

    fn clear_children(&mut self) {
        for child in &self.children {
            child.set_next_sibling(None);
        }

        self.children.clear();
    }

    fn attach_shadow_children(&self, _children: impl IntoIterator<Item = Child>) {
        // TODO: We need to support shadow root in dry nodes, we just don't print it yet
        // (as there's no way to).
        todo!()
    }

    fn add_class(&mut self, name: &str) {
        self.attributes
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

    fn remove_class(&mut self, name: &str) {
        if let Some(class) = self.attributes.get_mut("class") {
            *class = class
                .split_ascii_whitespace()
                .filter(|&c| c != name)
                .join(" ");
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

        if let Some(value) = value.text() {
            self.attributes.insert(name.to_owned(), value.into_owned());
        } else {
            self.attributes.remove(name);
        }
    }

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        self.hydrate_actions
            .push(Box::new(move |element| element.on(name, f)))
    }

    fn try_dom_element(&self) -> Option<web_sys::Element> {
        None
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        self.hydrate_actions
            .push(Box::new(move |element| element.effect(f)))
    }

    fn clone_node(&self) -> Self {
        let children = self.children.clone();

        for (index, child) in children.iter().enumerate() {
            child.set_next_sibling(children.get(index + 1));
        }

        Self {
            namespace: self.namespace,
            tag: self.tag.clone(),
            attributes: self.attributes.clone(),
            children,
            hydrate_actions: Vec::new(),
            next_sibling: None,
        }
    }
}

impl SharedDryElement<HydroNode> {
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

impl<Child: fmt::Display> fmt::Display for SharedDryElement<Child> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}", self.tag)?;

        for (name, value) in &self.attributes {
            write!(f, " {}=\"{}\"", name, encode_double_quoted_attribute(value))?;
        }

        f.write_str(">")?;

        for child in &self.children {
            child.fmt(f)?;
        }

        let has_children = !self.children.is_empty();
        let requires_closing_tag = !NO_CLOSING_TAG.contains(&self.tag.as_str());

        if requires_closing_tag || has_children {
            write!(f, "</{}>", self.tag)?;
        }

        Ok(())
    }
}
impl<Child: Into<WetNode>> From<SharedDryElement<Child>> for WetElement {
    fn from(dry: SharedDryElement<Child>) -> Self {
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
        Self::from_shared(SharedHydroElement::Dry(Box::new(SharedDryElement::new(
            namespace, tag,
        ))))
    }

    fn append_child(&mut self, child: &HydroNode) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.append_child(child),
            SharedHydroElement::Wet(wet) => wet.append_child(&child.wet()),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn insert_child_before(
        &mut self,
        index: usize,
        child: &HydroNode,
        next_child: Option<&HydroNode>,
    ) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.insert_child_before(index, child, next_child),
            SharedHydroElement::Wet(wet) => {
                wet.insert_child_before(index, &child.wet(), next_child.map(|c| c.wet()).as_ref())
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn replace_child(&mut self, index: usize, new_child: &HydroNode, old_child: &HydroNode) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.replace_child(index, new_child, old_child),
            SharedHydroElement::Wet(wet) => {
                wet.replace_child(index, &new_child.wet(), &old_child.wet())
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn remove_child(&mut self, index: usize, child: &HydroNode) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.remove_child(index, child),
            SharedHydroElement::Wet(wet) => wet.remove_child(index, &child.wet()),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn clear_children(&mut self) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.clear_children(),
            SharedHydroElement::Wet(wet) => wet.clear_children(),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn attach_shadow_children(&self, children: impl IntoIterator<Item = Self::Node>) {
        match &*self.borrow() {
            SharedHydroElement::Dry(dry) => dry.attach_shadow_children(children),
            SharedHydroElement::Wet(wet) => {
                wet.attach_shadow_children(children.into_iter().map(Self::Node::into))
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn add_class(&mut self, name: &str) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.add_class(name),
            SharedHydroElement::Wet(wet) => wet.add_class(name),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn remove_class(&mut self, name: &str) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.remove_class(name),
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
            SharedHydroElement::Dry(dry) => dry.attribute(name, value),
            SharedHydroElement::Wet(wet) => wet.attribute(name, value),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.on(name, f),
            SharedHydroElement::Wet(wet) => wet.on(name, f),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn try_dom_element(&self) -> Option<web_sys::Element> {
        match &*self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.try_dom_element(),
            SharedHydroElement::Wet(wet) => wet.try_dom_element(),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.effect(f),
            SharedHydroElement::Wet(wet) => wet.effect(f),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }
}

impl InstantiableDomElement for HydroElement {
    fn clone_node(&self) -> Self {
        Self::from_shared(match &*self.borrow() {
            SharedHydroElement::Dry(dry) => SharedHydroElement::Dry(Box::new(dry.clone_node())),
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
            SharedHydroText::Dry(dry) => {
                dry.next_sibling.as_ref().expect("No more siblings").clone()
            }
            SharedHydroText::Wet(wet) => HydroNode::Wet(WetNode::from(wet.clone()).next_sibling()),
            SharedHydroText::Unreachable => unreachable!(),
        }
    }

    fn set_next_sibling(&self, new_next_sibling: Option<HydroNode>) {
        match &mut *self.borrow_mut() {
            SharedHydroText::Dry(dry) => dry.next_sibling = new_next_sibling,
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
            SharedHydroText::Dry(dry) => Self::hydrate_child_text(dry.text, child, parent, tracker),
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
    Dry(SharedDryText),
    Wet(WetText),
    /// Used for swapping `Dry` for `Wet`
    Unreachable,
}

impl DomText for HydroText {
    fn new(text: &str) -> Self {
        Self(Rc::new(RefCell::new(SharedHydroText::Dry(SharedDryText {
            text: text.to_owned(),
            next_sibling: None,
        }))))
    }

    fn set_text(&mut self, new_text: &str) {
        match &mut *self.borrow_mut() {
            SharedHydroText::Dry(dry) => dry.text = new_text.to_string(),
            SharedHydroText::Wet(wet) => wet.set_text(new_text),
            SharedHydroText::Unreachable => unreachable!(),
        }
    }
}

impl fmt::Display for HydroText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.borrow() {
            SharedHydroText::Dry(dry) => f.write_str(&dry.text),
            SharedHydroText::Wet(wet) => f.write_str(&wet.text()),
            SharedHydroText::Unreachable => unreachable!(),
        }
    }
}

impl From<HydroText> for WetNode {
    fn from(text: HydroText) -> Self {
        let wet = match text.0.replace(SharedHydroText::Unreachable) {
            SharedHydroText::Dry(dry) => WetText::new(&dry.text),
            SharedHydroText::Wet(wet) => wet,
            SharedHydroText::Unreachable => unreachable!(),
        };

        text.0.replace(SharedHydroText::Wet(wet.clone()));
        wet.into()
    }
}

struct SharedDryText {
    text: String,
    next_sibling: Option<HydroNode>,
}

trait TrackSibling: Clone {
    fn set_next_sibling(&self, next_sibling: Option<&Self>);
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
}

impl TrackSibling for HydroNode {
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
            Self::Wet(node) => {
                HydroElement::from_shared(SharedHydroElement::Wet(node.into_element()))
            }
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
