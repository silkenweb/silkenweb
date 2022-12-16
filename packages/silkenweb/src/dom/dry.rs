use std::{
    cell::{Ref, RefCell, RefMut},
    fmt,
    rc::Rc,
};

use html_escape::encode_double_quoted_attribute;
use indexmap::IndexMap;
use itertools::Itertools;
use wasm_bindgen::JsValue;

use super::{
    wet::{WetElement, WetNode, WetText},
    Dom, DomElement, DomText, InstantiableDom, InstantiableDomElement, InstantiableDomNode,
};
use crate::node::element::Namespace;

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

    fn attach_shadow_children(&self, _children: impl IntoIterator<Item = Self::Node>) {}

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

    fn try_dom_element(&self) -> Option<&web_sys::Element> {
        todo!()
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

// TODO: Make this have an enum with wet and dry parts
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

    fn set_text(&mut self, text: &str) {
        match &mut *self.borrow_mut() {
            SharedDryText::Dry { text, .. } => *text = text.to_string(),
            SharedDryText::Wet(wet) => wet.set_text(text),
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
    pub fn into_wet(self) -> WetNode {
        self.wet()
    }

    pub fn wet(&self) -> WetNode {
        todo!()
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
