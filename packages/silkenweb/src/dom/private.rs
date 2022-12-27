use std::fmt::Display;

use wasm_bindgen::JsValue;

use crate::{attribute::Attribute, node::element::Namespace};

pub trait Dom: 'static {
    type Element: DomElement<Node = Self::Node>;
    type Text: DomText + Into<Self::Node>;
    type Node: Clone + Display + 'static;
}

pub trait InstantiableDom:
    Dom<Element = Self::InstantiableElement, Node = Self::InstantiableNode>
{
    type InstantiableElement: InstantiableDomElement<Node = Self::InstantiableNode>;
    type InstantiableNode: InstantiableDomNode<DomType = Self>;
}

pub trait DomElement: Into<Self::Node> + Clone + 'static {
    type Node;

    fn new(ns: Namespace, tag: &str) -> Self;

    fn append_child(&mut self, child: &Self::Node);

    fn insert_child_before(
        &mut self,
        index: usize,
        child: &Self::Node,
        next_child: Option<&Self::Node>,
    );

    fn replace_child(&mut self, index: usize, new_child: &Self::Node, old_child: &Self::Node);

    fn remove_child(&mut self, index: usize, child: &Self::Node);

    fn clear_children(&mut self);

    fn add_class(&mut self, name: &str);

    fn remove_class(&mut self, name: &str);

    fn attribute<A>(&mut self, name: &str, value: A)
    where
        A: Attribute;

    fn on(&mut self, name: &'static str, f: impl FnMut(JsValue) + 'static, events: &mut EventStore);

    fn dom_element(&self) -> web_sys::Element {
        self.try_dom_element().unwrap()
    }

    fn try_dom_element(&self) -> Option<web_sys::Element>;

    fn effect(&mut self, f: impl FnOnce(&web_sys::Element) + 'static);
}

pub trait DomText: Clone + 'static {
    fn new(text: &str) -> Self;

    fn set_text(&mut self, text: &str);
}

pub trait InstantiableDomElement: DomElement {
    fn attach_shadow_children(&mut self, children: impl IntoIterator<Item = Self::Node>);

    fn clone_node(&self) -> Self;
}

pub trait InstantiableDomNode: Display + Clone {
    type DomType: Dom;

    fn into_element(self) -> <Self::DomType as Dom>::Element;

    fn first_child(&self) -> Self;

    fn next_sibling(&self) -> Self;
}

#[cfg(feature = "weak-refs")]
mod event {
    use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

    #[derive(Default, Clone)]
    pub struct EventStore {}

    impl EventStore {
        pub fn add_listener(
            &mut self,
            element: &web_sys::Element,
            name: &'static str,
            f: impl FnMut(JsValue) + 'static,
        ) {
            element
                .add_event_listener_with_callback(
                    name,
                    Closure::new(f).into_js_value().unchecked_ref(),
                )
                .unwrap_throw();
        }

        pub fn combine(&mut self, _other: Self) {}
    }
}

#[cfg(not(feature = "weak-refs"))]
mod event {
    use std::{cell::RefCell, rc::Rc};

    use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

    #[derive(Default, Clone)]
    pub struct EventStore(Rc<RefCell<(Vec<EventCallback>, Vec<Self>)>>);

    impl EventStore {
        /// `f` must be `'static` as JS callbacks are called once the stack
        /// frame is finished. See the [Closure::wrap] and
        /// <https://github.com/rustwasm/wasm-bindgen/issues/1914#issuecomment-566488497>
        pub fn add_listener(
            &mut self,
            element: &web_sys::Element,
            name: &'static str,
            f: impl FnMut(JsValue) + 'static,
        ) {
            let callback = Closure::new(f);
            element
                .add_event_listener_with_callback(name, callback.as_ref().unchecked_ref())
                .unwrap_throw();

            self.0.borrow_mut().0.push(EventCallback {
                element: element.clone(),
                name,
                callback,
            });
        }

        pub fn combine(&mut self, other: Self) {
            // We need to keep track of any events subsequently added to `other`, so we need
            // to keep track of it's `Rc`.
            self.0.borrow_mut().1.push(other);
        }
    }

    pub struct EventCallback {
        element: web_sys::Element,
        name: &'static str,
        callback: Closure<dyn FnMut(JsValue)>,
    }

    impl Drop for EventCallback {
        fn drop(&mut self) {
            self.element
                .remove_event_listener_with_callback(
                    self.name,
                    self.callback.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap_throw();
        }
    }
}

pub use event::EventStore;
