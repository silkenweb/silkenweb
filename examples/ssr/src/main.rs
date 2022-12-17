use futures_signals::signal::Mutable;
use silkenweb::{
    dom::hydro::Dry, elements::html::*, node::Node, prelude::*, task::server::render_now_sync,
    value::Sig,
};

// For a more complete example, see <https://github.com/silkenweb/ssr-example>
fn main() {
    let count = Mutable::new(0);
    let count_text = count.signal_ref(|i| format!("{}", i));
    let inc = {
        clone!(count);
        move |_, _| {
            count.replace_with(|i| *i + 1);
        }
    };

    let element: Node<Dry> = Div::new()
        .child(Button::new().on_click(inc).text("+"))
        .child(P::new().text(Sig(count_text)))
        .into();

    assert_eq!(
        format!("{}", &element),
        r#"<div><button>+</button><p></p></div>"#
    );

    render_now_sync();
    assert_eq!(
        format!("{}", &element),
        r#"<div><button>+</button><p>0</p></div>"#
    );

    count.set(100);
    render_now_sync();
    assert_eq!(
        format!("{}", &element),
        r#"<div><button>+</button><p>100</p></div>"#
    );
}
