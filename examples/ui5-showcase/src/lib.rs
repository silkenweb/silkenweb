use parse_display::{Display, FromStr};
use silkenweb::{elements::div, mount, signal::Signal, Builder, ParentBuilder};
use silkenweb_ui5::{
    chrono::{ui5_calendar, SelectionMode},
    icon::ui5_icon,
    side_navigation::{ui5_side_navigation, ui5_side_navigation_item},
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let icon = ui5_icon().name("activate").build();
    let calendar = ui5_calendar()
        .format_pattern("yyyy-MM-dd")
        .selected_date("2000-01-01".to_string())
        .selection_mode(SelectionMode::Multiple)
        .on_selected_dates_change(|event, _target| {
            for d in event.selected_dates() {
                web_log::println!("{}", d);
            }
        })
        .build();

    let selected = Signal::new(Selected::Calendar);
    let set_selected = selected.write();

    // TODO: Use "data-id" rather than "id". Can we make this more type safe?
    // TODO: Slots
    let side_bar = ui5_side_navigation()
        .child(
            ui5_side_navigation_item()
                .selected(true)
                .text("Calendar")
                .id(Selected::Calendar.to_string()),
        )
        .child(
            ui5_side_navigation_item()
                .text("Icon")
                .id(Selected::Icon.to_string()),
        )
        .on_selection_change(move |event, _target| {
            set_selected.set(
                event
                    .item()
                    .get_attribute("id")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
            );
        });

    mount(
        "app",
        div()
            .child(side_bar)
            .child(selected.read().map(move |selection| match selection {
                Selected::Calendar => calendar.clone().into_element(),
                Selected::Icon => icon.clone().into_element(),
            })),
    );

    Ok(())
}

#[derive(Display, FromStr)]
enum Selected {
    Icon,
    Calendar,
}
