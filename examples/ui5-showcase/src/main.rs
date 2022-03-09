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
    icon::{ui5_icon, Icon},
    side_navigation::{self, side_navigation},
};
use wasm_bindgen::prelude::JsValue;

pub fn main() -> Result<(), JsValue> {
    use side_navigation::item;

    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let icon = || -> silkenweb_ui5::icon::Ui5Icon { ui5_icon().name(Icon::Activate).build() };
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

    let side_bar = side_navigation()
        .children([
            item(Selected::Calendar).text("Calendar").selected(),
            item(Selected::Icon).text("Icon"),
        ])
        .on_selection_change(move |new_selection| selected.set(new_selection));

    mount(
        "app",
        div()
            .class([FLEX])
            .child(side_bar)
            .child_signal(selected_signal.map(move |selection| -> Element {
                match selection {
                    Selected::Calendar => calendar().into(),
                    Selected::Icon => icon().into(),
                }
            })),
    );

    Ok(())
}

#[derive(Display, FromStr, Copy, Clone)]
enum Selected {
    Icon,
    Calendar,
}

css_classes!("styles.css");
