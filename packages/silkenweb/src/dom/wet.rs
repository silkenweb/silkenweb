use silkenweb_base::{document, intern_str};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

use super::{Dom, DomElement};
use crate::{hydration::node::Namespace, task::on_animation_frame};

pub struct Wet;

impl Dom for Wet {
    type Element = WetElement;
    type Node = WetNode;
    type Text = WetText;
}

#[derive(Clone)]
pub struct WetElement {
    element: web_sys::Element,
}

impl DomElement for WetElement {
    fn new(ns: Namespace, tag: &str) -> Self {
        let element = match ns {
            Namespace::Html => document::create_element(tag),
            Namespace::Other(ns) => document::create_element_ns(ns.map(intern_str), tag),
        };

        Self { element }
    }

    fn append_child(&mut self, child: Self) {
        self.element.append_child(&child.element).unwrap_throw();
    }

    fn insert_child_before(&mut self, child: Self, next_child: Option<Self>) {
        self.element
            .insert_before(
                &child.element,
                next_child.map(|c| c.element.into()).as_ref(),
            )
            .unwrap_throw();
    }

    fn replace_child(&mut self, new_child: Self, old_child: Self) {
        self.element
            .replace_child(&new_child.element, &old_child.element)
            .unwrap_throw();
    }

    fn remove_child(&mut self, child: Self) {
        self.element.remove_child(&child.element).unwrap_throw();
    }

    fn clear_children(&mut self) {
        self.element.set_inner_html("")
    }

    fn add_class(&mut self, name: &str) {
        self.element.class_list().add_1(name).unwrap_throw()
    }

    fn remove_class(&mut self, name: &str) {
        self.element.class_list().remove_1(name).unwrap_throw()
    }

    fn attribute<A>(&mut self, name: &str, value: A)
    where
        A: crate::attribute::Attribute,
    {
        if let Some(attr) = value.text() {
            self.element.set_attribute(name, &attr)
        } else {
            self.element.remove_attribute(name)
        }
        .unwrap_throw()
    }

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static) {
        // TODO: This only works with weak-refs. We need to store the callback for
        // none-weak-refs
        self.element
            .add_event_listener_with_callback(name, Closure::new(f).into_js_value().unchecked_ref())
            .unwrap_throw();
    }

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static) {
        let element = self.element.clone();
        on_animation_frame(move || f(&element));
    }
}

pub struct WetText {}
pub struct WetNode {}
