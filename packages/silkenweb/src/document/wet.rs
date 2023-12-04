use std::{cell::RefCell, collections::HashMap};

use silkenweb_base::document;
use wasm_bindgen::UnwrapThrowExt;

use super::{Document, DocumentHead, HeadNotFound};
use crate::{
    dom::{self, Wet},
    insert_element, mount_point,
    node::element::{
        child_vec::{ChildVec, ChildVecHandle, ParentShared},
        Const, GenericElement,
    },
    ELEMENTS,
};

impl Document for Wet {
    fn mount(id: &str, element: impl Into<GenericElement<Self, Const>>) {
        let element = element.into();

    mount_point(id)
            .replace_with_with_node_1(&element.dom_element())
            .unwrap_throw();
        insert_element(element);
    }

    fn mount_in_head(id: &str, head: DocumentHead<Self>) -> Result<(), HeadNotFound> {
        let head_elem = <Wet as dom::private::Dom>::Element::from_element(
            document::head().ok_or(HeadNotFound)?.into(),
        );

        let child_vec = ChildVec::<Wet, ParentShared>::new(head_elem, 0);
        let child_vec_handle = child_vec.run(head.child_vec);

        MOUNTED_IN_HEAD.with(|mounted| {
            mounted
                .borrow_mut()
                .insert(id.to_string(), child_vec_handle)
        });

        Ok(())
    }

    fn unmount_all() {
        ELEMENTS.with(|elements| {
            for element in elements.take().into_values() {
                element.dom_element().remove()
            }
        });

        for element in MOUNTED_IN_HEAD.with(|mounted| mounted.take().into_values()) {
            element.clear();
        }
    }

    fn head_inner_html() -> String {
        let mut html = String::new();

        MOUNTED_IN_HEAD.with(|mounted| {
            for elem in mounted.borrow().values() {
                html.push_str(&elem.inner_html());
            }
        });

        html
    }
}

thread_local! {
    static MOUNTED_IN_HEAD: RefCell<Vec<String, ChildVecHandle<Wet, ParentShared>>> = RefCell::new(HashMap::new());
}
