pub mod badge;
pub mod colour;
pub mod utility;

pub mod css {
    // TODO: Should `css_classes!` be able to make a trait to apply the class to an
    // `ElementBuilder`?
    silkenweb::css_classes!(visibility: pub, path: "bootstrap-5.2.2/css/bootstrap.min.css");
}

pub mod icon {
    silkenweb::css_classes!(visibility: pub, path: "bootstrap-icons-1.9.1/bootstrap-icons.css");
}
