use std::{
    cell::{Ref, RefCell, RefMut},
    fmt,
    rc::Rc,
};

use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use super::{
    dry::{DryChild, SharedDryElement, SharedDryText},
    private::{DomElement, DomText, EventStore, InstantiableDomElement, InstantiableDomNode},
    wet::{WetElement, WetNode, WetText},
    Hydro,
};
use crate::{hydration::HydrationStats, node::element::Namespace};

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

impl HydroElement {
    pub fn hydrate(self, element: &web_sys::Element, tracker: &mut HydrationStats) -> WetElement {
        let wet = match self.0.replace(SharedHydroElement::Unreachable) {
            SharedHydroElement::Dry(dry) => dry.hydrate(element, tracker),
            SharedHydroElement::Wet(wet) => wet,
            SharedHydroElement::Unreachable => unreachable!(),
        };

        self.0.replace(SharedHydroElement::Wet(wet.clone()));
        wet
    }

    pub fn hydrate_in_head(self, _id: &str, _tracker: &mut HydrationStats) {
        todo!()
    }

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

enum SharedHydroElement {
    /// Box is used to keep the enum variant small
    Dry(Box<SharedDryElement<HydroNode>>),
    Wet(WetElement),
    /// Used only for swapping from `Dry` to `Wet`
    Unreachable,
}

impl DomElement for HydroElement {
    type Node = HydroNode;

    fn new(namespace: &Namespace, tag: &str) -> Self {
        Self::from_shared(SharedHydroElement::Dry(Box::new(SharedDryElement::new(
            namespace.clone(),
            tag,
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

    fn on(
        &mut self,
        name: &'static str,
        f: impl FnMut(JsValue) + 'static,
        events: &mut EventStore,
    ) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.on(name, f, events),
            SharedHydroElement::Wet(wet) => wet.on(name, f, events),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn dom_element(&self) -> web_sys::Element {
        self.try_dom_element()
            .expect("Can't get raw dom element from `Hydro` dom element until it's been hydrated")
    }

    fn try_dom_element(&self) -> Option<web_sys::Element> {
        match &*self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.try_dom_element(),
            SharedHydroElement::Wet(wet) => wet.try_dom_element(),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn style_property(&mut self, name: &str, value: &str) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.style_property(name, value),
            SharedHydroElement::Wet(wet) => wet.style_property(name, value),
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
    fn attach_shadow_children(&mut self, children: impl IntoIterator<Item = Self::Node>) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry(dry) => dry.attach_shadow_children(children),
            SharedHydroElement::Wet(wet) => {
                wet.attach_shadow_children(children.into_iter().map(Self::Node::into))
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

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
            SharedHydroText::Dry(dry) => dry.next_sibling().expect("No more siblings").clone(),
            SharedHydroText::Wet(wet) => HydroNode::Wet(WetNode::from(wet.clone()).next_sibling()),
            SharedHydroText::Unreachable => unreachable!(),
        }
    }

    fn set_next_sibling(&self, new_next_sibling: Option<HydroNode>) {
        match &mut *self.borrow_mut() {
            SharedHydroText::Dry(dry) => dry.set_next_sibling(new_next_sibling),
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
            SharedHydroText::Dry(dry) => {
                Self::hydrate_child_text(dry.into(), child, parent, tracker)
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

    fn clone_node(&self) -> Self {
        Self(Rc::new(RefCell::new(self.borrow().clone_node())))
    }
}

enum SharedHydroText {
    Dry(SharedDryText<HydroNode>),
    Wet(WetText),
    /// Used for swapping `Dry` for `Wet`
    Unreachable,
}

impl SharedHydroText {
    fn clone_node(&self) -> Self {
        match self {
            SharedHydroText::Dry(text) => SharedHydroText::Dry(text.clone_node()),
            SharedHydroText::Wet(text) => SharedHydroText::Wet(text.clone_node()),
            SharedHydroText::Unreachable => unreachable!(),
        }
    }
}

impl DomText for HydroText {
    fn new(text: &str) -> Self {
        Self(Rc::new(RefCell::new(SharedHydroText::Dry(
            SharedDryText::new(text.to_string()),
        ))))
    }

    fn set_text(&mut self, new_text: &str) {
        match &mut *self.borrow_mut() {
            SharedHydroText::Dry(dry) => dry.set_text(new_text.to_string()),
            SharedHydroText::Wet(wet) => wet.set_text(new_text),
            SharedHydroText::Unreachable => unreachable!(),
        }
    }
}

impl fmt::Display for HydroText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.borrow() {
            SharedHydroText::Dry(dry) => f.write_str(dry.text()),
            SharedHydroText::Wet(wet) => f.write_str(&wet.text()),
            SharedHydroText::Unreachable => unreachable!(),
        }
    }
}

impl From<HydroText> for WetNode {
    fn from(text: HydroText) -> Self {
        let wet = match text.0.replace(SharedHydroText::Unreachable) {
            SharedHydroText::Dry(dry) => WetText::new(dry.text()),
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
}

impl DryChild for HydroNode {
    fn clone_node(&self) -> Self {
        match self {
            HydroNode::Text(text) => HydroNode::Text(text.clone_node()),
            HydroNode::Element(element) => HydroNode::Element(element.clone_node()),
            HydroNode::Wet(node) => HydroNode::Wet(node.clone_node()),
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
