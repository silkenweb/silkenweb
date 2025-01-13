use futures::channel::oneshot;
use silkenweb_task::spawn_local;

use super::{
    children_with_id, document_head, wet_insert_mounted, wet_unmount, Document, MountHydro,
    MountHydroHead,
};
use crate::{
    document::HYDRO_MOUNTED_IN_HEAD,
    dom::{self, private::DomElement, Hydro},
    hydration::HydrationStats,
    mount_point,
    node::element::{
        child_vec::{ChildVec, ParentShared},
        Const, GenericElement, Namespace,
    },
};

impl Document for Hydro {
    type MountInHeadOutput = MountHydroHead;
    type MountOutput = MountHydro;

    /// See [`hydrate`] for more details.
    ///
    /// [`hydrate`] just calls [`Hydro::mount`].
    ///
    /// [`hydrate`]: crate::hydration::hydrate
    fn mount(id: &str, element: impl Into<GenericElement<Self, Const>>) -> Self::MountOutput {
        #[cfg(debug_assertions)]
        crate::log_panics();
        let element = element.into();
        let id = id.to_string();

        let (send, receive) = oneshot::channel();
        spawn_local(async move {
            let mut stats = HydrationStats::default();

            let mount_point = mount_point(&id);
            let wet_element = element.hydrate(&mount_point, &mut stats);
            wet_insert_mounted(&id, wet_element);
            let _ = send.send(stats);
        });

        MountHydro(receive)
    }

    fn mount_in_head(id: &str, head: super::DocumentHead<Self>) -> Self::MountInHeadOutput {
        let hydro_head_elem = <Hydro as dom::private::Dom>::Element::new(&Namespace::Html, "head");
        let child_vec = ChildVec::<Hydro, ParentShared>::new(hydro_head_elem.clone(), 0);

        HYDRO_MOUNTED_IN_HEAD.with(|m| m.mount(id, child_vec.run(children_with_id(head, id))));
        let id = id.to_string();
        let head_elem = document_head();

        let (send, receive) = oneshot::channel();
        spawn_local(async move {
            let mut stats = HydrationStats::default();
            hydro_head_elem.hydrate_in_head(head_elem, &id, &mut stats);
            let _ = send.send(stats);
        });

        MountHydroHead(receive)
    }

    fn unmount_all() {
        wet_unmount();
        HYDRO_MOUNTED_IN_HEAD.with(|m| m.unmount_all());
    }

    fn head_inner_html() -> String {
        HYDRO_MOUNTED_IN_HEAD.with(|m| m.inner_html())
    }
}
