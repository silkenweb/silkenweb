use silkenweb::{
    elements::html::{a, div, A},
    log_panics, mount,
    node::element::{ParentElement, TextParentElement},
};
use silkenweb_bootstrap::{
    badge::badge,
    button::{button, ButtonStyle},
    column,
    dropdown::dropdown_menu,
    icon::Icon,
    tab_bar::{self, tab_bar, tab_bar_unordered, TabBarItem},
    utility::{Active, Align, Colour, Disabled, SetDisplay, SetSpacing, Size},
};

fn main() {
    log_panics();

    let margin = Some(Size::Size3);
    let app = column()
        .align_items(Align::Start)
        .margin(margin)
        .child(
            button("button", "Button", ButtonStyle::Outline(Colour::Secondary))
                .icon(Icon::check_circle_fill())
                .margin(margin),
        )
        .child(badge("Badge", Colour::Primary).margin(margin))
        .child(
            div()
                .margin(margin)
                .child(Icon::circle().colour(Colour::Danger)),
        )
        .child(tab_bar().children(tab_bar_items()))
        .child(
            tab_bar()
                .style(tab_bar::Style::Tabs)
                .children(tab_bar_items()),
        )
        .child(
            tab_bar()
                .style(tab_bar::Style::Pills)
                .children(tab_bar_items()),
        )
        .child(
            tab_bar_unordered()
                .style(tab_bar::Style::Tabs)
                .children(tab_bar_items())
                .child(TabBarItem::dropdown(
                    a().href("#").text("Menu"),
                    dropdown_menu().children([
                        a().href("#").text("Menu item 1"),
                        a().href("#").text("Menu item 2"),
                    ]),
                )),
        );
    mount("app", app);
}

fn tab_bar_items() -> impl Iterator<Item = A> {
    [
        a().href("#").text("Active").active(true),
        a().href("#").text("Tab"),
        a().href("#").text("Disabled").disabled(true),
    ]
    .into_iter()
}
