use futures_signals::signal::Mutable;
use silkenweb::{
    css, document::Document, dom::DefaultDom, elements::html::*, prelude::*,
    task::server::render_now_sync, value::Sig,
};

css!(content = ".red { color: red }", auto_mount);

// For a more complete example, see <https://github.com/silkenweb/ssr-example>
fn main() {
    let count = Mutable::new(0);
    let element = app(count.clone()).freeze();

    println!("Style: {}", DefaultDom::head_inner_html());
    println!("App: {}", &element);

    render_now_sync();
    println!("App (count = 0): {}", &element);

    count.set(100);
    render_now_sync();
    println!("App (count = 100): {}", &element);
}

fn app(count: Mutable<i32>) -> Div {
    let count_text = count.signal_ref(|i| format!("{i}"));
    let inc = move |_, _| {
        count.replace_with(|i| *i + 1);
    };

    div()
        .class(class::red())
        .child(button().on_click(inc).text("+"))
        .child(p().text(Sig(count_text)))
}
