use gloo_console::info;
use html::{div, img, p};
use silkenweb::{css, prelude::*};
use web_sys::DragEvent;

css!(path = "style.css");

fn main() {
    log_panics();

    let app = div()
        .child(p().text("Drag the kitten into the box and check the console"))
        .child(
            div()
                .class(class::BOXED)
                .on_drop(log_drop)
                .on_dragover(|ev, _| ev.prevent_default()),
        )
        .child(
            img()
                .src("https://placekitten.com/200/300")
                .width("200")
                .height("300")
                .draggable("true")
                .on_dragstart(set_drag_id),
        );
    mount("app", app);
}

fn log_drop(ev: DragEvent, _target: impl Sized) {
    info!(
        "Received",
        ev.data_transfer().unwrap().get_data("text").unwrap()
    )
}

fn set_drag_id(ev: DragEvent, _target: impl Sized) {
    ev.data_transfer()
        .unwrap()
        .set_data("text", "Kitty")
        .unwrap()
}
