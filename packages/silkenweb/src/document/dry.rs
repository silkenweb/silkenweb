use super::{Document, DocumentHead, HeadNotFound, MountHandle};
use crate::{
    dom::{self, private::DomElement, Dry},
    node::element::{
        child_vec::{ChildVec, ParentUnique},
        Const, GenericElement, Namespace,
    },
    task,
};

impl Document for Dry {
    fn mount(_id: &str, _element: impl Into<GenericElement<Self, Const>>) -> MountHandle {
        panic!("`mount` is not supported on `Dry` DOMs")
    }

    fn unmount_all() {
        task::local::with(|local| local.document.mounted_in_dry_head.take());
    }

    fn mount_in_head(id: &str, head: DocumentHead<Self>) -> Result<(), HeadNotFound> {
        let head_elem = <Dry as dom::private::Dom>::Element::new(&Namespace::Html, "head");
        let child_vec = ChildVec::<Dry, ParentUnique>::new(head_elem, 0);
        let child_vec_handle = child_vec.run(head.child_vec);

        task::local::with(|local| {
            local
                .document
                .mounted_in_dry_head
                .borrow_mut()
                .insert(id.to_string(), child_vec_handle)
        });

        Ok(())
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