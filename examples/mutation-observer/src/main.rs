use silkenweb::{dbg, elements::html::*, prelude::*};

fn main() {
    let app = details()
        .child(summary().text("Mutation Observer: expand me for more details..."))
        .child(p().text("Check the console output to see the mutation observer in action!"))
        .observe_mutations(|observe| {
            observe.open(|elem, _prev| {
                dbg!(elem.open());
            })
        });

    mount("app", app);
}
