use super::{Document, MountHandle};
use crate::{
    dom::Dry,
    node::element::{Const, Element, GenericElement, Mut},
    task,
};

impl Document for Dry {
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
