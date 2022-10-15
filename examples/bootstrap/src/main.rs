use silkenweb::{elements::html::div, mount, prelude::ParentBuilder};
use silkenweb_bootstrap::{
    badge::badge,
    button::{button, ButtonStyle},
    column,
    icon::Icon,
    utility::{Align, Colour, SetFlex, SetSpacing, Size},
};

fn main() {
    let margin = Some(Size::Size3);
    let app = column()
        .align_items(Align::Start)
        .margin(margin)
        .child(
            button("button")
                .appearance(ButtonStyle::Outline(Colour::Secondary))
                .icon(Icon::check_circle_fill())
                .text("Button")
                .margin(margin),
        )
        .child(badge("Badge", Colour::Primary).margin(margin))
        .child(
            div()
                .margin(margin)
                .child(Icon::circle().colour(Colour::Danger)),
        );
    mount("app", app);
}
