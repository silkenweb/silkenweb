use surfinia::{append_to_body, button, div, state};

fn main() {
    append_to_body(state(0, |i, set_i| {
        let inc_i = set_i.clone();
        let dec_i = set_i;

        div()
            .child(button().on_click(move || inc_i.set(i + 1)).text("+"))
            .child(button().on_click(move || dec_i.set(i - 1)).text("-"))
            .text(format!("Count = {}", i))
            // TODO: Can we find a way to remove this .build call?
            .build()
    }));
}
