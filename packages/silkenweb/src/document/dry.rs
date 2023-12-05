use futures_signals::signal_vec::SignalVecExt;

use super::{Document, DocumentHead};
use crate::{
    dom::{self, private::DomElement, Dry},
    node::element::{
        child_vec::{ChildVec, ParentShared},
        Const, Element, GenericElement, Namespace,
    },
    task, HEAD_ID_ATTRIBUTE,
};

impl Document for Dry {
    type MountInHeadOutput = ();
    type MountOutput = ();

    fn mount(_id: &str, _element: impl Into<GenericElement<Self, Const>>) -> Self::MountOutput {
        panic!("`mount` is not supported on `Dry` DOMs")
    }

    fn mount_in_head(id: &str, head: DocumentHead<Self>) -> Self::MountInHeadOutput {
        let head_elem = <Dry as dom::private::Dom>::Element::new(&Namespace::Html, "head");
        let child_vec = ChildVec::<Dry, ParentShared>::new(head_elem, 0);
        let children_with_id = head.child_vec.map({
            let id = id.to_string();
            move |child| child.attribute(HEAD_ID_ATTRIBUTE, id.clone()).into()
        });
        let child_vec_handle = child_vec.run(children_with_id);

        let existing = task::local::with(|local| {
            local
                .document
                .mounted_in_dry_head
                .borrow_mut()
                .insert(id.to_string(), child_vec_handle)
        });

        assert!(
            existing.is_none(),
            "Attempt to insert duplicate id ({id}) into head"
        );
    }

    fn unmount_all() {
        task::local::with(|local| local.document.mounted_in_dry_head.take());
    }

    fn head_inner_html() -> String {
        let mut html = String::new();

        task::local::with(|local| {
            for elem in local.document.mounted_in_dry_head.borrow().values() {
                html.push_str(&elem.inner_html());
            }
        });

        html
    }
}
