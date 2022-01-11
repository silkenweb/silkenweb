//! A library for building reactive single page web apps
//!
//! # Quick Start
//!
//! The best way to get started is to look at the examples. You'll need [trunk]
//! to run them. For example, to run [hello-world]:
//!
//! ```bash
//! cd examples/hello-world
//! trunk serve --open
//! ```
//!
//! - [hello-world] is a minimal example
//! - [counter] is a minimal interactive example
//! - [router] is a simple routing example
//! - [animation] is a simple animation example
//! - [todomvc] is an example of a simple app
//!
//! [trunk]: https://trunkrs.dev/
//! [hello-world]: https://github.com/silkenweb/silkenweb/tree/main/examples/hello-world
//! [counter]: https://github.com/silkenweb/silkenweb/tree/main/examples/counter
//! [router]: https://github.com/silkenweb/silkenweb/tree/main/examples/router
//! [animation]: https://github.com/silkenweb/silkenweb/tree/main/examples/animation
//! [todomvc]: https://github.com/silkenweb/silkenweb/tree/main/examples/todomvc
#[doc(inline)]
pub use silkenweb_dom as dom;
#[doc(inline)]
pub use silkenweb_elements as elements;
#[doc(inline)]
pub use silkenweb_signals_ext as signals_ext;

pub mod animation;
pub mod router;
pub mod storage;

#[doc(inline)]
pub use crate::dom::clone;
#[doc(inline)]
pub use crate::storage::Storage;
