use super::Document;
use crate::{
    dom::{self, private::DomElement, Hydro},
    node::element::{
        child_vec::{ChildVec, ParentShared},
        Namespace,
    },
};

impl Document for Hydro {
    fn mount(
        _id: &str,
        _element: impl Into<crate::node::element::GenericElement<Self, crate::node::element::Const>>,
    ) {
        todo!()
    }

    fn unmount_all() {
        todo!()
    }

    fn mount_in_head(
        id: &str,
        _head: super::DocumentHead<Self>,
    ) -> Result<(), super::HeadNotFound> {
        let hydro_head_elem = <Hydro as dom::private::Dom>::Element::new(&Namespace::Html, "head");
        let _child_vec = ChildVec::<Hydro, ParentShared>::new(hydro_head_elem.clone(), 0);

        // TODO: Run child vec until pending
        hydro_head_elem.hydrate_in_head(id);

        // TODO: Run child vec

        Ok(())
    }

    fn head_inner_html() -> String {
        todo!()
    }
}
