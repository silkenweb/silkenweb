use silkenweb_macros::cfg_browser;

use crate::{document, router};

#[derive(Default)]
pub struct TaskLocal {
    pub(crate) task: super::TaskLocal,
    pub(crate) document: document::TaskLocal,
    pub(crate) router: router::TaskLocal,
}

#[cfg_browser(true)]
mod arch {
    use super::TaskLocal;

    thread_local! {
        pub static TASK_LOCAL: TaskLocal = TaskLocal::default();
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
            Err(_) => panic!("Must be run from within `silkenweb::task::scope`"),
        }
    }
}

pub use arch::{with, TASK_LOCAL};
