use silkenweb::{elements::html::Div, Element};

#[derive(Element)]
struct MyElement(#[element(target)] Div, #[element(target)] Div);

fn main() {}
