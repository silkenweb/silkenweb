use surfinia::{button, div, mount, use_state};

fn main() {
    let (count, set_count) = use_state(0);
    let inc = set_count.clone();
    let dec = set_count;

    mount(
        "app",
        div()
            .child(button().on_click(move || inc.map(|i| i + 1)).text("+"))
            .child(button().on_click(move || dec.map(|i| i - 1)).text("-"))
            .child(count.with(|i| div().text(format!("Count = {}", i)))),
    );
}
