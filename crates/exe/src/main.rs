use lib::{state, tag};

fn main() {
    console_error_panic_hook::set_once();
    web_log::println!("Running");

    tag("div")
        .attribute("id", "hello-world!")
        .child(state(0, |i, set_i| {
            if i < 10 {
                set_i.set(i + 1)
            };

            tag("p").text(format!("Count = {}", i))
        }))
        .build()
        .append_to_body();
}
