use lib::{state, tag};

fn main() {
    console_error_panic_hook::set_once();
    web_log::println!("Running");

    tag("div")
        .attribute("id", "hello-world!")
        .child(state(0, |i, set_i| {
            if i < 100 {
                set_i.set(i + 1)
            };

            // TODO: Make text a free function as well.
            let mut counters = tag("p").text(format!("Count = {}", i));

            for j in 0..i {
                counters = counters.child(state(j, |j, set_j| {
                    set_j.set(j + 1);
                    tag("p").text(format!("Count = {}", j))
                }))
            }

            counters
        }))
        .build()
        .append_to_body();
}
