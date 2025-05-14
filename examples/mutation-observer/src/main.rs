use silkenweb::{dbg, elements::html::*, prelude::*};

fn main() {
    let app = div().child(
        details()
            .child(summary().text("Mutation Observer: expand me for more details..."))
            .child(p().text("Check the console output to see the mutation observer in action!"))
            .begin_observations()
            .open(|elem| {
                let open = elem.has_attribute("open");
                dbg!("'open' was toggled", open);
            })
            .end_observations(),
    );

    mount("app", app);
}
