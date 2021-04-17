use surfinia_core::{hooks::state::Signal, mount};
use surfinia_html::{button, div};

fn main() {
    console_error_panic_hook::set_once();

    let count = Signal::new(0);
    let inc = count.setter();
    let dec = count.setter();

    mount(
        "app",
        div()
            .child(button().on_click(move || dec.replace(|i| i - 1)).text("-"))
            .text(count.map(|i| format!("{}", i)))
            .child(button().on_click(move || inc.replace(|i| i + 1)).text("+")),
    );
}
