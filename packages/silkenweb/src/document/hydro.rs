use super::{insert_mounted, Document};
use crate::{
    dom::{self, private::DomElement, Hydro, Wet},
    hydration::HydrationStats,
    mount_point,
    node::element::{
        child_vec::{ChildVec, ParentShared},
        Namespace,
    },
};

impl Document for Hydro {
    type MountOutput = HydrationStats;

    fn mount(
        id: &str,
        element: impl Into<crate::node::element::GenericElement<Self, crate::node::element::Const>>,
    ) -> Self::MountOutput {
        #[cfg(debug_assertions)]
        crate::log_panics();
        let element = element.into();
        let mut stats = HydrationStats::default();

        let mount_point = mount_point(id);
        let wet_element = element.hydrate(&mount_point, &mut stats);
        insert_mounted(id, wet_element);

        stats
    }

    fn mount_in_head(
        id: &str,
        _head: super::DocumentHead<Self>,
    ) -> Result<Self::MountOutput, super::HeadNotFound> {
        let hydro_head_elem = <Hydro as dom::private::Dom>::Element::new(&Namespace::Html, "head");
        let mut stats = HydrationStats::default();
        let _child_vec = ChildVec::<Hydro, ParentShared>::new(hydro_head_elem.clone(), 0);

        // TODO: Run child vec until pending
        hydro_head_elem.hydrate_in_head(id, &mut stats);

        // TODO: Run child vec
        // TODO: Store child vec handle

        Ok(stats)
    }

    fn unmount_all() {
        Wet::unmount_all()
    }

    fn head_inner_html() -> String {
        Wet::head_inner_html()
    }
}
