use std::{cell::RefCell, collections::HashMap};

use wasm_bindgen::UnwrapThrowExt;

use crate::{
    dom::{Dom, Dry, Wet},
    insert_element, mount_point,
    node::element::{Const, Element, GenericElement, Mut},
    remove_element, task, ELEMENTS,
};

pub trait DocumentRender: Dom + Sized {
    /// Mount an element on the document.
    ///
    /// `id` is the id of the mount point element. The element will replace
    /// the mount point. The returned `MountHandle` should usually just be
    /// discarded, but it can be used to restore the mount point if
    /// required. This can be useful for testing.
    fn mount(id: &str, element: impl Into<GenericElement<Self, Const>>) -> MountHandle;

    /// Remove all mounted elements.
    ///
    /// Mount points will not be restored. This is useful to ensure a clean
    /// environment for testing.
    fn unmount_all();

    /// Mount an element as a child of `<head>`
    ///
    /// This will search for `id` in the document. If it's found, no action is
    /// taken and `false` is returned. If there's no matching `id` in the
    /// document:
    ///
    /// - The `id` attribute  is set on `element`.
    /// - `element` is added as a child of `head`.
    /// - `true` is returned.
    fn mount_in_head(id: &str, element: impl Into<GenericElement<Self, Mut>>) -> bool;

    /// Get the inner HTML of `<head>`.
    ///
    /// This only includes elements added with `mount_in_head`. It's useful for
    /// server side rendering, where it can be used to add any stylesheets
    /// required for the HTML. The `id` attributes will be set on each element,
    /// so hydration can avoid adding duplicate stylesheets with
    /// [`Self::mount_in_head`].
    fn head_inner_html() -> String;
}

impl DocumentRender for Wet {
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
        if crate::document::query_selector(&format!("#{}", web_sys::css::escape(id)))
            .unwrap_throw()
            .is_some()
        {
            return false;
        }

        let element = element.into().attribute("id", id).freeze();
        let dom_element = element.dom_element();
        crate::document::head()
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

impl DocumentRender for Dry {
    fn mount(_id: &str, _element: impl Into<GenericElement<Self, Const>>) -> MountHandle {
        panic!("`mount` is not supported on `Dry` DOMs")
    }

    fn unmount_all() {
        task::local::with(|local| local.document.mounted_in_dry_head.take());
    }

    fn mount_in_head(id: &str, element: impl Into<GenericElement<Self, Mut>>) -> bool {
        task::local::with(|local| {
            let mut mounted = local.document.mounted_in_dry_head.borrow_mut();

            if mounted.contains_key(id) {
                return false;
            }

            mounted.insert(id.to_string(), element.into().attribute("id", id).freeze());
            true
        })
    }

    fn head_inner_html() -> String {
        let mut html = String::new();

        task::local::with(|local| {
            for elem in local.document.mounted_in_dry_head.borrow().values() {
                html.push_str(&elem.to_string());
            }
        });

        html
    }
}

/// Manage a mount point
pub struct MountHandle {
    id: u128,
    mount_point: web_sys::Element,
}

impl MountHandle {
    fn new(mount_point: web_sys::Element, element: GenericElement<Wet, Const>) -> Self {
        Self {
            id: insert_element(element),
            mount_point,
        }
    }

    /// Remove the mounted element and restore the mount point.
    pub fn unmount(self) {
        if let Some(element) = remove_element(self.id) {
            element
                .dom_element()
                .replace_with_with_node_1(&self.mount_point)
                .unwrap_throw();
        }
    }
}

thread_local! {
    static MOUNTED_IN_WET_HEAD: RefCell<Vec<GenericElement<Wet, Const>>> = RefCell::new(Vec::new());
}

#[derive(Default)]
pub(crate) struct TaskLocal {
    mounted_in_dry_head: RefCell<HashMap<String, GenericElement<Dry, Const>>>,
}
