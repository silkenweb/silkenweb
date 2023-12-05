use futures::FutureExt;
use silkenweb_task::spawn_local;

use super::{document_head, wet_insert_mounted, wet_unmount, Document, MountHydro, MountHydroHead};
use crate::{
    document::MountedInHead,
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

        let (future, remote_handle) = async move {
            let mut stats = HydrationStats::default();

            let mount_point = mount_point(&id);
            let wet_element = element.hydrate(&mount_point, &mut stats);
            wet_insert_mounted(&id, wet_element);
            stats
        }
        .remote_handle();
        spawn_local(future);

        MountHydro(remote_handle)
    }

    fn mount_in_head(id: &str, head: super::DocumentHead<Self>) -> Self::MountInHeadOutput {
        let hydro_head_elem = <Hydro as dom::private::Dom>::Element::new(&Namespace::Html, "head");
        let child_vec = ChildVec::<Hydro, ParentShared>::new(hydro_head_elem.clone(), 0);

        MOUNTED_IN_HEAD.with(|m| m.mount(id, child_vec.run(head.child_vec)));
        let id = id.to_string();
        let head_elem = document_head();

        let (future, remote_handle) = async move {
            let mut stats = HydrationStats::default();
            hydro_head_elem.hydrate_in_head(head_elem, &id, &mut stats);
            stats
        }
        .remote_handle();
        spawn_local(future);

        MountHydroHead(remote_handle)
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
    static MOUNTED_IN_HEAD: MountedInHead<Hydro> = MountedInHead::new();
}
