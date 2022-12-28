use criterion::{criterion_group, criterion_main, Criterion};
use futures_signals::signal::{Mutable, SignalExt};
use silkenweb::{
    dom::{Dom, Dry},
    elements::html::{self, button, div, h1, p, span, Div},
    node::Node,
    prelude::{ElementEvents, ParentElement},
    task::server::render_now_sync,
    value::Sig,
};
use web_sys::{HtmlButtonElement, MouseEvent};

fn update(count: Mutable<isize>, inc: isize) -> impl Fn(MouseEvent, HtmlButtonElement) {
    move |_, _| {
        count.replace_with(|i| *i + inc);
    }
}

fn counter<D: Dom>(initial: isize) -> Div<D> {
    let count = Mutable::new(initial);
    let count_text = count.signal().map(|i| format!("Value: {i}!"));

    div()
        .child(button().on_click(update(count.clone(), -1)).text("-1"))
        .child(span().text(Sig(count_text)))
        .child(button().on_click(update(count, 1)).text("+1"))
}

/// A very basic SSR benchmark similar to
/// <https://github.com/gbj/leptos/blob/a68d276c90f0273999ba52fc2a34268c4453dd1c/benchmarks/src/ssr.rs>
pub fn ssr(c: &mut Criterion) {
    c.bench_function("ssr", |b| {
        b.iter(|| {
            let node: Node<Dry> = html::main()
                .child(h1().text("Welcome to our benchmark page."))
                .child(p().text("Here's some introductory text."))
                .children([1, 2, 3].into_iter().map(counter))
                .into();

            render_now_sync();
            let rendered = node.to_string();

            assert_eq!(rendered, "<main><h1>Welcome to our benchmark page.</h1><p>Here's some introductory text.</p><div><button>-1</button><span>Value: 1!</span><button>+1</button></div><div><button>-1</button><span>Value: 2!</span><button>+1</button></div><div><button>-1</button><span>Value: 3!</span><button>+1</button></div></main>");
        })
    });
}

criterion_group!(benches, ssr);
criterion_main!(benches);
