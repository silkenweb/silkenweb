use silkenweb::{elements::html::Div, Element};

#[derive(Element)]
struct MyElement(#[element(target, xyz)] Div);

fn main() {}
