use lib::tag;

fn main() {
    tag("div")
        .attribute("id", "hello-world!")
        .text("Hello, world!")
        .build()
        .append_to_body();
}
