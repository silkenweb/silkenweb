use surfinia_core::{
    hooks::state::{use_state, GetState},
    mount,
    Builder,
    Element,
};
use surfinia_html::{button, div};

fn main() {
    console_error_panic_hook::set_once();

    let (count, set_count) = use_state(0);
    let inc = set_count.clone();
    let dec = set_count;

    mount(
        "app",
        div()
            .child(button().on_click(move || inc.map(|i| i + 1)).text("+"))
            .child(button().on_click(move || dec.map(|i| i - 1)).text("-"))
            .childx(count.with_derived(|i| div().text(format!("Count = {}", i)).build().into())),
    );
}
