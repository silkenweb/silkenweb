use silkenweb::{elements::html::*, prelude::*};

mod red {
    silkenweb::css!(path = "red.css", auto_mount, transpile = (modules));

    // TODO: What if we wanted to export the `mod class` as `pub`?
    // modules = {
    // pub use classes as my_mod;
    // pub use stylesheet as my_mod2;
    // },

    pub use class::*;
}

mod green {
    silkenweb::css!(path = "green.css", auto_mount, transpile = (modules));

    pub use class::*;
}

fn main() {
    // "red.css" and "green.css" use the same class name for text color, but this is
    // scoped using `modules`.
    let app = div().children([
        div().class(red::color()).text("Red text"),
        div().class(green::color()).text("Green text"),
    ]);
    mount("app", app);
}
