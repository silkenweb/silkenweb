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
    avatar::{self, Avatar},
    badge::{self, Badge},
    chrono::{ui5_calendar, SelectionMode, Ui5Calendar},
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
            item(Selected::Badge).text("Badge"),
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
                    Selected::Badge => badge().into(),
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

fn calendar() -> Ui5Calendar {
    ui5_calendar()
        .format_pattern("yyyy-MM-dd")
        .selected_date("2000-01-01".to_string())
        .selection_mode(SelectionMode::Multiple)
        .on_selected_dates_change(|event, _target| {
            for d in event.selected_dates() {
                web_log::println!("{}", d);
            }
        })
        .build()
}

fn badge() -> Badge {
    badge::badge().color_scheme(2).text("Badge").icon(icon())
}

fn icon() -> Ui5Icon {
    ui5_icon().name(Icon::Activate).build()
}

#[derive(Display, FromStr, Copy, Clone)]
enum Selected {
    Avatar,
    Badge,
    Icon,
    Calendar,
}

css_classes!("styles.css");
