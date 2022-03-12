use futures_signals::signal::{Mutable, SignalExt};
use parse_display::{Display, FromStr};
use silkenweb::{
    css_classes,
    elements::html::div,
    mount,
    node::element::{Element, ElementBuilder},
    prelude::{HtmlElement, ParentBuilder},
};
use silkenweb_ui5::{
    avatar::{self, Avatar, AvatarGroup},
    badge::{self, Badge},
    bar::{self, Bar, BarDesign},
    breadcrumbs::{self, breadcrumbs_item, Breadcrumbs},
    busy_indicator::{self, BusyIndicator},
    button::{self, Button},
    calendar::{self, Calendar, SelectionMode},
    icon::{ui5_icon, Icon, Ui5Icon},
    side_navigation::{self, side_navigation},
};
use wasm_bindgen::prelude::JsValue;

pub fn main() -> Result<(), JsValue> {
    use side_navigation::item;

    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let selected = Mutable::new(Selected::Calendar);
    let selected_signal = selected.signal();

    let side_bar = side_navigation()
        .children([
            item(Selected::Avatar).text("Avatar"),
            item(Selected::AvatarGroup).text("Avatar Group"),
            item(Selected::Badge).text("Badge"),
            item(Selected::Bar).text("Bar"),
            item(Selected::Breadcrumbs).text("Breadcrumbs"),
            item(Selected::BusyIndicator).text("BusyIndicator"),
            item(Selected::Button).text("Button"),
            item(Selected::Calendar).text("Calendar").selected(),
            item(Selected::Icon).text("Icon"),
        ])
        .on_selection_change(move |_, new_selection| selected.set(new_selection));

    mount(
        "app",
        div()
            .class([FLEX])
            .child(side_bar)
            .child_signal(selected_signal.map(move |selection| -> Element {
                match selection {
                    Selected::Avatar => avatar().into(),
                    Selected::AvatarGroup => avatar_group().into(),
                    Selected::Badge => badge().into(),
                    Selected::Bar => bar().into(),
                    Selected::BusyIndicator => busy_indicator().into(),
                    Selected::Breadcrumbs => breadcrumbs().into(),
                    Selected::Button => button().into(),
                    Selected::Calendar => calendar().into(),
                    Selected::Icon => icon().into(),
                }
            })),
    );

    Ok(())
}

fn avatar() -> Avatar {
    avatar::avatar().initials("SB").build()
}

fn avatar_group() -> AvatarGroup {
    #[derive(Display, FromStr)]
    enum Avatar {
        Sb,
        Bb,
    }

    avatar::avatar_group()
        .children([
            (
                Avatar::Sb,
                avatar::avatar().initials("SB").icon(Icon::Employee),
            ),
            (Avatar::Bb, avatar::avatar().initials("BB")),
        ])
        .on_overflow(|_, _| web_log::println!("Visible avatars changed"))
        .on_click(|_, _, id| {
            web_log::println!(
                "Avatar clicked: {}",
                id.map_or_else(|| "Overflow button".to_string(), |av| av.to_string())
            )
        })
        .build()
}

fn badge() -> Badge {
    badge::badge()
        .color_scheme(2)
        .text("Badge")
        .icon(icon())
        .build()
}

fn bar() -> Bar {
    bar::bar()
        .start_content(div().text("Start"))
        .middle_content(div().text("Middle"))
        .end_content(div().text("End"))
        .design(BarDesign::Header)
        .build()
}

fn breadcrumbs() -> Breadcrumbs {
    #[derive(FromStr, Display)]
    enum BreadcrumbId {
        Item1,
        Item2,
        Item3,
        Item4,
        Item5,
    }

    breadcrumbs::breadcrumbs()
        .children([
            (BreadcrumbId::Item1, breadcrumbs_item().text("Item1")),
            (BreadcrumbId::Item2, breadcrumbs_item().text("Item2")),
            (BreadcrumbId::Item3, breadcrumbs_item().text("Item3")),
            (BreadcrumbId::Item4, breadcrumbs_item().text("Item4")),
            (BreadcrumbId::Item5, breadcrumbs_item().text("Item5")),
        ])
        .on_item_click(|_, id| web_log::println!("{} clicked", id))
        .build()
}

fn busy_indicator() -> BusyIndicator {
    busy_indicator::busy_indicator().active(true).build()
}

fn button() -> Button {
    button::button()
        .text("Press Me")
        .on_click(|_, _| web_log::println!("Button clicked"))
        .build()
}

fn calendar() -> Calendar {
    calendar::calendar()
        .format_pattern("yyyy-MM-dd")
        .selected_dates(["2000-01-01".to_string()])
        .selection_mode(SelectionMode::Multiple)
        .on_selected_dates_change(|event, _target| {
            for d in event.dates() {
                web_log::println!("{}", d);
            }
        })
        .build()
}

fn icon() -> Ui5Icon {
    ui5_icon().name(Icon::Activate).build()
}

#[derive(Display, FromStr, Copy, Clone)]
enum Selected {
    Avatar,
    AvatarGroup,
    Badge,
    Bar,
    Breadcrumbs,
    BusyIndicator,
    Button,
    Icon,
    Calendar,
}

css_classes!("styles.css");
