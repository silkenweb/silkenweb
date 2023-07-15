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
    use silkenweb_test::{mounted_html, new_mount_point};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    use crate::app;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn browser_test() {
        new_mount_point("app").await;
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

    // TODO: `silkenweb-test` package with `new_mount_point` and `mounted_html`
    mod silkenweb_test {
        use silkenweb::{document::Document, dom::DefaultDom, task::render_now};
        use silkenweb_base::document;
        use wasm_bindgen::{JsCast, UnwrapThrowExt};

        pub async fn new_mount_point(mount_point_id: &str) {
            // Clear the render queue
            render_now().await;

            if let Some(existing) = query_element(APP_CONTAINER_ID) {
                existing.remove()
            }

            DefaultDom::unmount_all();
            let app_container = document::create_element("div");
            app_container.set_id(APP_CONTAINER_ID);
            let mount_point = document::create_element("div");
            mount_point.set_id(mount_point_id);
            app_container
                .append_child(&mount_point)
                .expect_throw("Couldn't add mount point to app container");
            let body = document::body().expect_throw("Couldn't get document body");
            body.append_child(&app_container)
                .expect_throw("Couldn't add app container to body");
        }

        pub fn mounted_html() -> String {
            query_element(APP_CONTAINER_ID)
                .expect_throw("Element not found")
                .inner_html()
        }

        fn query_element(id: &str) -> Option<web_sys::HtmlElement> {
            document::query_selector(&format!("#{id}"))
                .expect_throw("Error searching for element")
                .map(|elem| {
                    elem.dyn_into()
                        .expect_throw("Element was not an `HTMLElement")
                })
        }

        const APP_CONTAINER_ID: &str = "silkenweb-test-mount-point";
    }
}
