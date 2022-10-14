use silkenweb::mount;
use silkenweb_bootstrap::{badge::badge, utility::Colour};

fn main() {
    mount("app", badge("Badge", Colour::Primary));
}
