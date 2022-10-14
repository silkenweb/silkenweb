use silkenweb::mount;
use silkenweb_bootstrap::{badge::Badge, colour::Colour};

fn main() {
    let app = Badge::new("Hello, world!", Colour::Primary).rounded_pill();

    mount("app", app);
}
