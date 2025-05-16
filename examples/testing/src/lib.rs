use futures_signals::signal::Mutable;
#[cfg(test)]
use silkenweb::cfg_browser;
use silkenweb::{
    elements::{html::*, ElementEvents},
    node::element::{ParentElement, TextParentElement},
    value::Sig,
};

pub fn app(count: Mutable<i32>) -> Div {
    let count_text = count.signal_ref(|i| format!("{i}"));
    let inc = move |_, _| {
        count.replace_with(|i| *i + 1);
    };

    div()
        .child(button().on_click(inc).text("+"))
        .child(p().text(Sig(count_text)))
}

#[cfg_browser(true)]
#[cfg(test)]
mod browser_tests {
    use futures_signals::signal::Mutable;
    use silkenweb::{mount, task::render_now};
    use silkenweb_test::BrowserTest;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    use crate::app;

    wasm_bindgen_test_configure!(run_in_browser);

    // Run this with `wasm-pack test --firefox --headless`
    #[wasm_bindgen_test]
    async fn browser_test() {
        let test = BrowserTest::new("app").await;
        let count = Mutable::new(0);

        let app = app(count.clone());

        mount("app", app);
        render_now().await;

        assert_eq!(r#"<div><button>+</button><p>0</p></div>"#, test.html());
        count.set(1);
        render_now().await;
        assert_eq!(r#"<div><button>+</button><p>1</p></div>"#, test.html());
    }
}

#[cfg_browser(false)]
#[cfg(test)]
mod tests {
    use futures_signals::signal::Mutable;

    use crate::app;

    // Run this with `cargo test`
    #[test]
    fn sync_test() {
        use silkenweb::task::{server, sync_scope};

        sync_scope(|| {
            let count = Mutable::new(0);

            let app = app(count.clone()).freeze();
            server::render_now_sync();
            assert_eq!(r#"<div><button>+</button><p>0</p></div>"#, app.to_string());
            count.set(1);
            server::render_now_sync();
            assert_eq!(r#"<div><button>+</button><p>1</p></div>"#, app.to_string());
        });
    }
}
