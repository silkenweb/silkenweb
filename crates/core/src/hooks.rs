pub mod memo;
pub mod reference;
pub mod state;
pub mod list_state;

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

use crate::{window, Builder, Element, OwnedChild};

#[derive(Default)]
pub struct Scope<T>(T);

impl<T: Scopeable> Scope<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn with<ElemBuilder, Elem, Gen>(&mut self, mut generate: Gen) -> Elem
    where
        ElemBuilder: Builder<Target = Elem>,
        Elem: Into<Element>,
        Element: Into<Elem>,
        Gen: FnMut(T) -> ElemBuilder,
    {
        let element = generate(self.0.clone()).build().into();

        let dom_element = element.dom_element.clone();

        element.set_parents(self.0.as_child().clone());
        self.0.add_child(element);

        Element {
            dom_element,
            states: vec![self.0.as_child().clone()],
            event_callbacks: Vec::new(),
        }
        .into()
    }
}

pub trait Scopeable: Clone {
    fn add_child(&mut self, element: Element);

    fn as_child(&self) -> Rc<RefCell<dyn OwnedChild>>;
}

trait AnyStateUpdater {
    fn dom_depth(&self) -> usize;

    fn apply(&self);
}

trait Effect {
    fn apply(&self);
}

fn queue_update(x: impl 'static + AnyStateUpdater) {
    let len = {
        UPDATE_QUEUE.with(|update_queue| {
            let mut update_queue = update_queue.borrow_mut();

            update_queue.push(Box::new(x));
            update_queue.len()
        })
    };

    if len == 1 {
        request_process_updates();
    }
}

fn request_process_updates() {
    PROCESS_UPDATES.with(|process_updates| {
        window()
            .request_animation_frame(process_updates.as_ref().unchecked_ref())
            .unwrap()
    });
}

fn process_updates() {
    UPDATE_QUEUE.with(|update_queue| {
        let mut update_queue = update_queue.take();

        if update_queue.len() != 1 {
            let mut updates_by_depth: Vec<_> = update_queue
                .into_iter()
                .map(|u| (u.dom_depth(), u))
                .collect();

            updates_by_depth.sort_unstable_by_key(|(key, _)| *key);

            update_queue = updates_by_depth
                .into_iter()
                .map(|(_, value)| value)
                .collect();
        }

        for update in update_queue {
            update.apply();
        }
    });

    EFFECT_STACK.with(|effect_queue| {
        for effect in effect_queue.take() {
            effect.apply();
        }
    });
}

thread_local!(
    static UPDATE_QUEUE: RefCell<Vec<Box<dyn AnyStateUpdater>>> = RefCell::new(Vec::new());
    static EFFECT_STACK: RefCell<Vec<Box<dyn Effect>>> = RefCell::new(Vec::new());
    static PROCESS_UPDATES: Closure<dyn FnMut(JsValue)> =
        Closure::wrap(Box::new(move |_time_stamp: JsValue| {
            process_updates();
        }));
);
