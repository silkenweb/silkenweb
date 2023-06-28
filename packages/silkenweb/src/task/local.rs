use std::{cell::RefCell, collections::HashMap};

use futures_signals::signal::Mutable;
use silkenweb_macros::cfg_browser;

use super::{arch::Runtime, Render};
use crate::{
    dom::{Dry, Wet},
    node::element::{Const, GenericElement},
    router::{self, UrlPath},
};

pub struct TaskLocal {
    pub mounted_in_dry_head: RefCell<HashMap<String, GenericElement<Dry, Const>>>,
    pub runtime: Runtime,
    pub element_handle_id: RefCell<u128>,
    pub elements: RefCell<HashMap<u128, GenericElement<Wet, Const>>>,
    pub url_path: Mutable<UrlPath>,
    pub(super) render: Render,
}

impl Default for TaskLocal {
    fn default() -> Self {
        Self {
            mounted_in_dry_head: Default::default(),
            runtime: Default::default(),
            element_handle_id: Default::default(),
            elements: Default::default(),
            render: Render::new(),
            url_path: router::new_url_path(),
        }
    }
}

#[cfg_browser(true)]
mod arch {
    use super::TaskLocal;

    thread_local! {
        static TASK_LOCAL: TaskLocal = TaskLocal::default();
    }

    pub fn with<R>(f: impl FnOnce(&TaskLocal) -> R) -> R {
        TASK_LOCAL.with(f)
    }
}

#[cfg_browser(false)]
mod arch {
    use super::TaskLocal;

    tokio::task_local! {
        pub static TASK_LOCAL: TaskLocal;
    }

    pub fn with<R>(f: impl FnOnce(&TaskLocal) -> R) -> R {
        match TASK_LOCAL.try_with(f) {
            Ok(r) => r,
            Err(_) => panic!("Must be run from within `silkenweb::task::server::scope`"),
        }
    }
}

pub use arch::{with, TASK_LOCAL};
