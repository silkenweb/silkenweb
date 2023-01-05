use futures_signals::signal::Mutable;
use silkenweb::{
    dom::Dry, elements::html::*, prelude::*, task::server::render_now_sync, value::Sig,
};

// For a more complete example, see <https://github.com/silkenweb/ssr-example>
fn main() {
    let count = Mutable::new(0);
    let element = app(count.clone()).freeze();

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

fn app(count: Mutable<i32>) -> Div<Dry> {
    let count_text = count.signal_ref(|i| format!("{}", i));
    let inc = move |_, _| {
        count.replace_with(|i| *i + 1);
    };

    div()
        .child(button().on_click(inc).text("+"))
        .child(p().text(Sig(count_text)))
}
