use futures_signals::signal::Mutable;
use silkenweb::{elements::html::*, prelude::*, value::Sig};

pub fn app(count: Mutable<i32>) -> Div {
    let count_text = count.signal_ref(|i| format!("{i}"));
    let inc = move |_, _| {
        count.replace_with(|i| *i + 1);
    };

    div()
        .child(button().on_click(inc).text("+"))
        .child(p().text(Sig(count_text)))
}

#[cfg(test)]
mod tests {
    use futures_signals::signal::Mutable;
    use silkenweb::{mount, task::render_now};
    use silkenweb_test::{mounted_html, setup_test};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    use crate::app;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn browser_test() {
        setup_test("app").await;
        let count = Mutable::new(0);

        let app = app(count.clone());

        mount("app", app);
        render_now().await;

        assert_eq!(r#"<div><button>+</button><p>0</p></div>"#, mounted_html());
        count.set(1);
        render_now().await;
        assert_eq!(r#"<div><button>+</button><p>1</p></div>"#, mounted_html());
    }

    #[test]
    fn sync_test() {
        use silkenweb::task::server;

        server::sync_scope(|| {
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
