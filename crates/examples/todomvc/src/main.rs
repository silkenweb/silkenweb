use surfinia::{h1, header, input, mount, section};

fn main() {
    console_error_panic_hook::set_once();

    mount(
        "app",
        section().class("todoapp").child(
            header().child(h1().text("todos")).child(
                input()
                    .class("new-todo")
                    .placeholder("What needs to be done?")
                    .autofocus(),
            ),
        ),
    );
}
