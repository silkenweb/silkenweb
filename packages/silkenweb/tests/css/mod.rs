// We put this at the top so line numbers are consistent in `stylesheet::mount`.
silkenweb::css!(content = ".red { color: red }", auto_mount);

use std::panic::Location;

use silkenweb::{
    document::Document,
    dom::DefaultDom,
    elements::html::{div, Div},
    node::element::{Const, Element},
    task::render_now,
};

isomorphic_test! {
    async fn css_auto_mount() {
        assert_eq!(DefaultDom::head_inner_html(), "");
        let elem: Div<DefaultDom, Const> = div().class(class::red()).freeze();
        render_now().await;
        assert_eq!(elem.to_string(), r#"<div class="red"></div>"#);
        let file = Location::caller().file();
        let style_html =
        format!(r#"<style data-silkenweb-head-id="silkenweb-style:{file}:2:1">.red {{ color: red }}</style>"#);
        assert_eq!(
            DefaultDom::head_inner_html(),
            style_html
        );

        // Make sure we only mount once.
        stylesheet::mount();

        assert_eq!(
            DefaultDom::head_inner_html(),
            style_html
        );
        DefaultDom::unmount_all();
    }
}
