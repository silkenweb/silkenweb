use super::{Document, DocumentHead};
use crate::{
    document::children_with_id,
    dom::{self, private::DomElement, Dry},
    node::element::{
        child_vec::{ChildVec, ParentShared},
        Const, GenericElement, Namespace,
    },
    task,
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
        let child_vec_handle = child_vec.run(children_with_id(head, id));

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
