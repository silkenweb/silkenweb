use std::cell::RefCell;

use silkenweb_base::document;
use wasm_bindgen::UnwrapThrowExt;

use super::{Document, MountHandle};
use crate::{
    dom::Wet,
    mount_point,
    node::element::{Const, Element, GenericElement, Mut},
    ELEMENTS,
};

impl Document for Wet {
    fn mount(id: &str, element: impl Into<GenericElement<Self, Const>>) -> MountHandle {
        let element = element.into();

        let mount_point = mount_point(id);
        mount_point
            .replace_with_with_node_1(&element.dom_element())
            .unwrap_throw();
        MountHandle::new(mount_point, element)
    }

    fn unmount_all() {
        ELEMENTS.with(|elements| {
            for element in elements.take().into_values() {
                element.dom_element().remove()
            }
        });

        for element in MOUNTED_IN_WET_HEAD.with(|mounted| mounted.take()) {
            element.dom_element().remove()
        }
    }

    fn mount_in_head(id: &str, element: impl Into<GenericElement<Self, Mut>>) -> bool {
        if document::query_selector(&format!("#{}", web_sys::css::escape(id)))
            .unwrap_throw()
            .is_some()
        {
            return false;
        }

        let element = element.into().attribute("id", id).freeze();
        let dom_element = element.dom_element();
        document::head()
            .map(|head| {
                head.append_with_node_1(&dom_element).unwrap_throw();
                MOUNTED_IN_WET_HEAD.with(|mounted| mounted.borrow_mut().push(element));
            })
            .is_some()
    }

    fn head_inner_html() -> String {
        let mut html = String::new();

        MOUNTED_IN_WET_HEAD.with(|mounted| {
            for elem in &*mounted.borrow() {
                html.push_str(&elem.to_string());
            }
        });

        html
    }
}

thread_local! {
    static MOUNTED_IN_WET_HEAD: RefCell<Vec<GenericElement<Wet, Const>>> = RefCell::new(Vec::new());
}
