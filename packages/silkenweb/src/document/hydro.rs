use std::{cell::RefCell, collections::HashMap};

use futures::FutureExt;
use silkenweb_task::spawn_local;

use super::{insert_mounted, Document, MountHydro, MountHydroHead};
use crate::{
    dom::{self, private::DomElement, Hydro, Wet},
    hydration::HydrationStats,
    mount_point,
    node::element::{
        child_vec::{ChildVec, ChildVecHandle, ParentShared},
        Const, GenericElement, Namespace,
    },
};

impl Document for Hydro {
    // TODO: Wrap in an opaque type
    type MountInHeadOutput = MountHydroHead;
    // TODO: Use an opaque future
    type MountOutput = MountHydro;

    // TODO: Return a future and remove warning in docs
    /// See [`hydrate`] for more details.
    ///
    /// [`hydrate`] just calls [`Hydro::mount`]. In particular, this should be
    /// run from an `async` block as described in [`hydrate`].
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
            insert_mounted(&id, wet_element);
            stats
        }
        .remote_handle();
        spawn_local(future);

        MountHydro(remote_handle)
    }

    fn mount_in_head(
        id: &str,
        head: super::DocumentHead<Self>,
    ) -> Result<Self::MountInHeadOutput, super::HeadNotFound> {
        let hydro_head_elem = <Hydro as dom::private::Dom>::Element::new(&Namespace::Html, "head");
        let child_vec = ChildVec::<Hydro, ParentShared>::new(hydro_head_elem.clone(), 0);

        insert_mounted_in_head(id, child_vec.run(head.child_vec));
        let id = id.to_string();

        let (future, remote_handle) = async move {
            let mut stats = HydrationStats::default();
            hydro_head_elem.hydrate_in_head(&id, &mut stats);
            stats
        }
        .remote_handle();
        spawn_local(future);

        Ok(MountHydroHead(remote_handle))
    }

    fn unmount_all() {
        Wet::unmount_all()
    }

    fn head_inner_html() -> String {
        Wet::head_inner_html()
    }
}

fn insert_mounted_in_head(id: &str, child_vec: ChildVecHandle<Hydro, ParentShared>) {
    let existing =
        MOUNTED_IN_HEAD.with(|mounted| mounted.borrow_mut().insert(id.to_string(), child_vec));

    assert!(
        existing.is_none(),
        "Attempt to insert duplicate id ({id}) into head"
    );
}

thread_local! {
    static MOUNTED_IN_HEAD: RefCell<HashMap<String, ChildVecHandle<Hydro, ParentShared>>> = RefCell::new(HashMap::new());
}
