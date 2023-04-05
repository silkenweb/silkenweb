use silkenweb::{elements::html::*, prelude::*};

silkenweb::css!("color.css");

fn change_color(
    color: &Mutable<&'static str>,
    description: &str,
    new_color: &'static str,
) -> Button {
    clone!(color);
    button()
        .text(description)
        .on_click(move |_, _| color.set(new_color))
}

fn main() {
    let color = Mutable::new("green");
    let app = div()
        .style_property("--color", Sig(color.signal_cloned()))
        .child(change_color(&color, "Red", "red"))
        .child(change_color(&color, "Green", "green"))
        .child(
            div()
                .class(class::COLOR)
                .text("Click either the 'Red' or 'Green' button to set the color of this text."),
        );
    mount("app", app);
}
