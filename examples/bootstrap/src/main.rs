use silkenweb::{mount, prelude::ParentBuilder};
use silkenweb_bootstrap::{
    badge::badge,
    button::{button, ButtonStyle},
    column,
    utility::{Colour, FlexAlign, SetFlex, SetSpacing, Size},
};

fn main() {
    let margin = Some(Size::Size3);
    let app = column()
        .align_items(FlexAlign::Start)
        .margin(margin)
        .child(
            button()
                .appearance(ButtonStyle::Outline(Colour::Secondary))
                .text("Button")
                .margin(margin),
        )
        .child(badge("Badge", Colour::Primary).margin(margin));
    mount("app", app);
}
