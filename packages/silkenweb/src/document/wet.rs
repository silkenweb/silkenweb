use silkenweb_base::document;
use wasm_bindgen::UnwrapThrowExt;

use super::{wet_insert_mounted, wet_unmount, Document, DocumentHead, HeadNotFound};
use crate::{
    document::MountedInHead,
    dom::{self, Wet},
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

    fn mount_in_head(
        id: &str,
        head: DocumentHead<Self>,
    ) -> Result<Self::MountInHeadOutput, HeadNotFound> {
        let head_elem = <Wet as dom::private::Dom>::Element::from_element(
            document::head().ok_or(HeadNotFound)?.into(),
        );

        let child_vec = ChildVec::<Wet, ParentShared>::new(head_elem, 0);
        let child_vec_handle = child_vec.run(head.child_vec);

        MOUNTED_IN_HEAD.with(|m| m.mount(id, child_vec_handle));

        Ok(())
    }

    fn unmount_all() {
        wet_unmount();
        MOUNTED_IN_HEAD.with(|m| m.unmount_all());
    }

    fn head_inner_html() -> String {
        MOUNTED_IN_HEAD.with(|m| m.inner_html())
    }
}

thread_local! {
    static MOUNTED_IN_HEAD: MountedInHead<Wet> = MountedInHead::new();
}
