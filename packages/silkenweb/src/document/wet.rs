use wasm_bindgen::UnwrapThrowExt;

use super::{
    children_with_id, document_head, wet_insert_mounted, wet_unmount, Document, DocumentHead,
    WET_MOUNTED_IN_HEAD,
};
use crate::{
    dom::Wet,
    mount_point,
    node::element::{
        child_vec::{ChildVec, ParentShared},
        Const, GenericElement,
    },
};

impl Document for Wet {
    type MountInHeadOutput = ();
    type MountOutput = ();

    fn mount(id: &str, element: impl Into<GenericElement<Self, Const>>) -> Self::MountOutput {
        let element = element.into();

        mount_point(id)
            .replace_with_with_node_1(&element.dom_element())
            .unwrap_throw();
        wet_insert_mounted(id, element);
    }

    fn mount_in_head(id: &str, head: DocumentHead<Self>) -> Self::MountInHeadOutput {
        let head_elem = document_head();
        let child_vec = ChildVec::<Wet, ParentShared>::new(head_elem, 0);
        let child_vec_handle = child_vec.run(children_with_id(head, id));

        WET_MOUNTED_IN_HEAD.with(|m| m.mount(id, child_vec_handle));
    }

    fn unmount_all() {
        wet_unmount();
        WET_MOUNTED_IN_HEAD.with(|m| m.unmount_all());
    }

    fn head_inner_html() -> String {
        WET_MOUNTED_IN_HEAD.with(|m| m.inner_html())
    }
}
