use lib::{state, tag};

fn main() {
    console_error_panic_hook::set_once();
    
    tag("div")
        .attribute("id", "hello-world!")
        .child(state(0, |i, set_i| {
            web_log::println!("gen 1");
            if i < 10 {
                set_i.set(i + 1)
            };
            web_log::println!("gen 2");

            tag("p").text(format!("Count = {}", i))
        }))
        .build()
        .append_to_body();
}
