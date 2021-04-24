use silkenweb::{
    elements::{button, div},
    hooks::state::Signal,
    mount,
};

fn main() {
    console_error_panic_hook::set_once();

    let count = Signal::new(0);
    let inc = count.write();
    let dec = count.write();

    mount(
        "app",
        div()
            .child(
                button()
                    .on_click(move |_, _| dec.replace(|i| i - 1))
                    .text("-"),
            )
            .text(count.read().map(|i| format!("{}", i)))
            .child(
                button()
                    .on_click(move |_, _| inc.replace(|i| i + 1))
                    .text("+"),
            ),
    );
}
