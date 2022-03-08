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
    chrono::{ui5_calendar, SelectionMode},
    icon::ui5_icon,
    side_navigation::{ui5_side_navigation, ui5_side_navigation_item},
};
use wasm_bindgen::{prelude::JsValue, UnwrapThrowExt};

pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let icon = || ui5_icon().name("activate").build();
    let calendar = || {
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
    };

    let selected = Mutable::new(Selected::Calendar);
    let selected_signal = selected.signal();

    let side_bar = ui5_side_navigation()
        .children([
            ui5_side_navigation_item()
                .selected(true)
                .text("Calendar")
                .attribute(DATA_ID, &Selected::Calendar.to_string()),
            ui5_side_navigation_item()
                .text("Icon")
                .attribute(DATA_ID, &Selected::Icon.to_string()),
        ])
        .on_selection_change(move |event, _target| {
            selected.set(
                event
                    .item()
                    .get_attribute(DATA_ID)
                    .unwrap_throw()
                    .parse()
                    .unwrap_throw(),
            );
        });

    mount(
        "app",
        div()
            .class([FLEX])
            .child(side_bar)
            .child_signal(selected_signal.map(move |selection| match selection {
                Selected::Calendar => Element::from(calendar()),
                Selected::Icon => Element::from(icon()),
            })),
    );

    Ok(())
}

#[derive(Display, FromStr, Copy, Clone)]
enum Selected {
    Icon,
    Calendar,
}

const DATA_ID: &str = "data-id";

css_classes!("styles.css");
