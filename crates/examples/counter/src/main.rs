use surfinia::{append_to_body, button, div, StateSetter};

fn main() {
    let set_i = StateSetter::new(0);
    let inc_i = set_i.clone();
    let dec_i = set_i.clone();

    append_to_body(
        div()
            .child(button().on_click(move || inc_i.map(|i| i + 1)).text("+"))
            .child(button().on_click(move || dec_i.map(|i| i - 1)).text("-"))
            .child(set_i.with(move |i| div().text(format!("Count = {}", i)))),
    );
}
