use gloo_console::info;
use html::{div, img, p};
use silkenweb::{css, prelude::*};

css!(
    auto_mount,
    content = "
        .boxed {
            width: 350px;
            height: 70px;
            padding: 10px;
            border: 1px solid #aaaaaa;
        }
"
);

fn main() {
    log_panics();

    let app = div()
        .child(p().text("Drag the kitten into the box and check the console"))
        .child(
            div()
                .class(class::boxed())
                .on_drop(|ev, _| {
                    info!(
                        "Received ",
                        ev.data_transfer().unwrap().get_data("text").unwrap()
                    )
                })
                .on_dragover(|ev, _| ev.prevent_default()),
        )
        .child(
            img()
                .src("https://placekitten.com/200/300")
                .width("200")
                .height("300")
                .draggable("true")
                .on_dragstart(|ev, _| {
                    ev.data_transfer()
                        .unwrap()
                        .set_data("text", "Kitty")
                        .unwrap()
                }),
        );
    mount("app", app);
}
