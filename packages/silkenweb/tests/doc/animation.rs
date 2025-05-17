use futures_signals::signal::SignalExt;
use silkenweb::{
    animation::{finite_animation, infinite_animation},
    elements::{
        html::progress,
        svg::{attributes::Presentation, content_type::Length::Px, rect, svg},
    },
    mount,
    node::element::ParentElement,
    value::Sig,
};

pub fn doc_finite_animation() {
    const DURATION: f64 = 3000.0;
    let app = progress().max(DURATION as f32).value(Sig(
        finite_animation(DURATION).map(|time| time.unwrap_or(DURATION) as f32)
    ));
    mount("app", app);
}

pub fn doc_infinite_animation() {
    let app = svg().width(200.0).height(200.0).child(
        rect()
            .x(Px(25.0))
            .y(Px(25.0))
            .width(Px(50.0))
            .height(Px(50.0))
            .transform(Sig(
                infinite_animation().map(|time| format!("rotate({} 50 50)", time / 10.0))
            )),
    );
    mount("app", app);
}
