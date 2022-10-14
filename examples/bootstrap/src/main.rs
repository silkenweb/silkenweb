use silkenweb::{elements::html::div, mount, prelude::ParentBuilder};
use silkenweb_bootstrap::{
    badge::Badge,
    colour::Colour,
    utility::{Size::Size1, Spacing},
};

fn main() {
    let app = div().children([
        Badge::new("Normal badge", Colour::Primary).margin(Some(Size1)),
        Badge::new("Rounded pill badge", Colour::Primary)
            .rounded_pill()
            .margin(Some(Size1)),
    ]);

    mount("app", app);
}
