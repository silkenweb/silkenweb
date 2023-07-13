use std::{
    cell::{Ref, RefCell, RefMut},
    fmt,
    rc::Rc,
};

use silkenweb_base::clone;
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
            SharedHydroElement::Dry { elem, .. } => elem.fmt(f),
            SharedHydroElement::Wet(elem) => elem.fmt(f),
            SharedHydroElement::Unreachable => Ok(()),
        }
    }
}

impl HydroElement {
    pub fn hydrate(self, element: &web_sys::Element, tracker: &mut HydrationStats) -> WetElement {
        let wet = match self.0.replace(SharedHydroElement::Unreachable) {
            SharedHydroElement::Dry {
                elem,
                hydrate_actions,
            } => elem.hydrate(element, hydrate_actions, tracker),
            SharedHydroElement::Wet(elem) => elem,
            SharedHydroElement::Unreachable => unreachable!(),
        };

        self.0.replace(SharedHydroElement::Wet(wet.clone()));
        wet
    }

    fn from_shared(shared: SharedHydroElement) -> Self {
        Self(Rc::new(RefCell::new(shared)))
    }

    fn first_child(&self) -> HydroNode {
        match &*self.borrow() {
            SharedHydroElement::Dry { elem, .. } => elem.first_child().unwrap().clone(),
            SharedHydroElement::Wet(wet) => {
                HydroNode::Wet(WetNode::from(wet.clone()).first_child())
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn next_sibling(&self) -> HydroNode {
        match &*self.borrow() {
            SharedHydroElement::Dry { elem, .. } => {
                elem.next_sibling().expect("No more siblings").clone()
            }
            SharedHydroElement::Wet(wet) => {
                HydroNode::Wet(WetNode::from(wet.clone()).next_sibling())
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn set_next_sibling(&self, next_sibling: Option<HydroNode>) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry { elem, .. } => elem.set_next_sibling(next_sibling),
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
            SharedHydroElement::Dry {
                elem,
                hydrate_actions,
            } => elem.hydrate_child(parent, child, hydrate_actions, tracker),
            SharedHydroElement::Wet(elem) => elem,
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
    Dry {
        elem: Box<SharedDryElement<HydroNode>>,
        hydrate_actions: Vec<HydrateAction>,
    },
    Wet(WetElement),
    /// Used only for swapping from `Dry` to `Wet`
    Unreachable,
}

impl DomElement for HydroElement {
    type Node = HydroNode;

    fn new(namespace: Namespace, tag: &str) -> Self {
        Self::from_shared(SharedHydroElement::Dry {
            elem: Box::new(SharedDryElement::new(namespace, tag)),
            hydrate_actions: Vec::new(),
        })
    }

    fn append_child(&mut self, child: &HydroNode) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry { elem, .. } => elem.append_child(child),
            SharedHydroElement::Wet(elem) => elem.append_child(&child.wet()),
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
            SharedHydroElement::Dry { elem, .. } => {
                elem.insert_child_before(index, child, next_child)
            }
            SharedHydroElement::Wet(elem) => {
                elem.insert_child_before(index, &child.wet(), next_child.map(|c| c.wet()).as_ref())
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn replace_child(&mut self, index: usize, new_child: &HydroNode, old_child: &HydroNode) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry { elem, .. } => elem.replace_child(index, new_child, old_child),
            SharedHydroElement::Wet(elem) => {
                elem.replace_child(index, &new_child.wet(), &old_child.wet())
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn remove_child(&mut self, index: usize, child: &HydroNode) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry { elem, .. } => elem.remove_child(index, child),
            SharedHydroElement::Wet(elem) => elem.remove_child(index, &child.wet()),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn clear_children(&mut self) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry { elem, .. } => elem.clear_children(),
            SharedHydroElement::Wet(elem) => elem.clear_children(),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn add_class(&mut self, name: &str) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry { elem, .. } => elem.add_class(name),
            SharedHydroElement::Wet(elem) => elem.add_class(name),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn remove_class(&mut self, name: &str) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry { elem, .. } => elem.remove_class(name),
            SharedHydroElement::Wet(elem) => elem.remove_class(name),
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
            SharedHydroElement::Dry { elem, .. } => elem.attribute(name, value),
            SharedHydroElement::Wet(elem) => elem.attribute(name, value),
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
            SharedHydroElement::Dry {
                hydrate_actions, ..
            } => {
                clone!(mut events);

                hydrate_actions.push(Box::new(move |element| element.on(name, f, &mut events)))
            }
            SharedHydroElement::Wet(elem) => elem.on(name, f, events),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn try_dom_element(&self) -> Option<web_sys::Element> {
        match &*self.borrow_mut() {
            SharedHydroElement::Dry { .. } => None,
            SharedHydroElement::Wet(elem) => elem.try_dom_element(),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn style_property(&mut self, name: &str, value: &str) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry { elem, .. } => elem.style_property(name, value),
            SharedHydroElement::Wet(elem) => elem.style_property(name, value),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry {
                hydrate_actions, ..
            } => hydrate_actions.push(Box::new(move |element| element.effect(f))),
            SharedHydroElement::Wet(elem) => elem.effect(f),
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }
}

impl InstantiableDomElement for HydroElement {
    fn attach_shadow_children(&mut self, children: impl IntoIterator<Item = Self::Node>) {
        match &mut *self.borrow_mut() {
            SharedHydroElement::Dry { elem, .. } => elem.attach_shadow_children(children),
            SharedHydroElement::Wet(wet) => {
                wet.attach_shadow_children(children.into_iter().map(Self::Node::into))
            }
            SharedHydroElement::Unreachable => unreachable!(),
        }
    }

    fn clone_node(&self) -> Self {
        Self::from_shared(match &*self.borrow() {
            SharedHydroElement::Dry { elem, .. } => SharedHydroElement::Dry {
                elem: Box::new(elem.clone_node()),
                hydrate_actions: Vec::new(),
            },
            SharedHydroElement::Wet(wet) => SharedHydroElement::Wet(wet.clone_node()),
            SharedHydroElement::Unreachable => unreachable!(),
        })
    }
}

impl From<HydroElement> for WetNode {
    fn from(elem: HydroElement) -> Self {
        let wet = match elem.0.replace(SharedHydroElement::Unreachable) {
            SharedHydroElement::Dry {
                elem,
                hydrate_actions,
            } => (*elem).into_wet(hydrate_actions),
            SharedHydroElement::Wet(elem) => elem,
            SharedHydroElement::Unreachable => unreachable!(),
        };

        elem.0.replace(SharedHydroElement::Wet(wet.clone()));
        wet.into()
    }
}

pub type HydrateAction = Box<dyn FnOnce(&mut WetElement)>;

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
