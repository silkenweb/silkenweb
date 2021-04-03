use surfinia::{append_to_body, button, div, DivBuilder, StateSetter};

fn counter(set_i: StateSetter<u32>) -> DivBuilder {
    let inc_i = set_i.clone();
    let dec_i = set_i.clone();

    div()
        .child(button().on_click(move || inc_i.map(|i| i + 1)).text("+"))
        .child(button().on_click(move || dec_i.map(|i| i - 1)).text("-"))
        .child(set_i.state(|i| div().text(format!("Count = {}", i))))
}

fn main() {
    let set_i = StateSetter::new(0);
    
    append_to_body(counter(set_i.clone()).child(set_i.state(move |i| {
        let mut counters = div();

        for _j in 0..i {
            counters = counters.child(counter(StateSetter::new(0)));
        }

        counters
    })));
}
