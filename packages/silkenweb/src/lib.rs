//! A library for building reactive web apps
//!
//! # Quick Start
//!
//! First off, we'll need [trunk] to build our app. Install it with:
//!
//! ```bash
//! cargo install trunk
//! ```
//!
//! Once that's completed, lets jump right in and have a play around with the
//! example counter app. The full code is available [here][counter]. To run it:
//!
//! ```bash
//! cd examples/counter
//! trunk serve --open
//! ```
//!
//! It's not the most complex app, but it'll serve as a good example to show how
//! we can build an interactive web app. Lets go through the code, step by step.
//!
//! Firstly, we create a new `Mutable` and an
//! associated `Signal`. `Mutable` represents a variable, and `Signal`
//! represents values of that variable across time. Here we `map` a function
//! over values of `count`, to convert it to text.
//!
//! ```rust
//! # use futures_signals::signal::{Mutable, SignalExt};
//! # use silkenweb::{elements::html::*, prelude::*};
//!
//! let count = Mutable::new(0);
//! let count_text = count.signal().map(|i| format!("{}", i));
//! ```
//!
//! See the [futures-signals tutorial] for more detail on `Mutable`s and
//! `Signal`s.
//!
//! Next, we create a closure, `inc`, to increment `count`. Then we build the
//! DOM for our counter. `on_click` installs `inc` as an event handler to
//! increment the counter.
//!
//! ```no_run
//! # use futures_signals::signal::{Mutable, SignalExt};
//! # use silkenweb::{elements::html::*, prelude::*};
//! #
//! # let count = Mutable::new(0);
//! # let count_text = count.signal().map(|i| format!("{}", i));
//!
//! let inc = move |_, _| {
//!     count.replace_with(|i| *i + 1);
//! };
//!
//! let app = div()
//!     .child(button().on_click(inc).text("+"))
//!     .child(p().text_signal(count_text));
//! ```
//!
//! Finally, we `mount` our app on the DOM. This will find the element with `id
//! = "app"` and mount `app` there.
//!
//! ```no_run
//! # use futures_signals::signal::{Mutable, SignalExt};
//! # use silkenweb::{elements::html::*, prelude::*};
//! #
//! # let count = Mutable::new(0);
//! # let count_text = count.signal().map(|i| format!("{}", i));
//! #
//! # let inc = move |_, _| {
//! #     count.replace_with(|i| *i + 1);
//! # };
//! #
//! # let app = div()
//! #     .child(button().on_click(inc).text("+"))
//! #     .child(p().text_signal(count_text));
//! mount("app", app);
//! ```
//!
//! # Features
//!
//! See the associated module for documentation on how to use each feature:
//!
//! - [animation]
//! - [router]
//! - [storage]
//!
//! For documentation on building DOM trees, see [elements].
//!
//! [trunk]: https://trunkrs.dev/
//! [futures-signals tutorial]: https://docs.rs/futures-signals/0.3.24/futures_signals/tutorial/index.html
//! [counter]: https://github.com/silkenweb/silkenweb/tree/main/examples/counter
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

pub mod prelude {
    pub use crate::{
        clone,
        dom::{mount, node::element::ParentBuilder},
        elements::{ElementEvents, HtmlElement, HtmlElementEvents},
    };
}
