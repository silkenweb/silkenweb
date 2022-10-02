use silkenweb::{
    elements::{
        html::{button, div, p},
        ElementEvents,
    },
    mount,
    prelude::ParentBuilder,
    router,
};

fn main() {
    mount("app", {
        div()
            .child(
                button()
                    .on_click(|_, _| router::set_url_path("/basic"))
                    .text("Go to basic route"),
            )
            .child(
                button()
                    .on_click(|_, _| router::set_url_path("/with_args/arg1/arg2"))
                    .text("Go to route with args"),
            )
            .child(
                button()
                    .on_click(|_, _| router::set_url_path("/with_query?x=1&y=2"))
                    .text("Go to route with query string"),
            )
            .child(p().text_signal(router::url_path().signal_ref(|url_path| {
                let path_components = url_path.path_components_vec();

                match &path_components[..] {
                    [] => "This is the root route!".to_string(),
                    ["basic"] => "This is a basic route".to_string(),
                    ["with_args", args @ ..] => format!("This is a route with args {args:?}"),
                    ["with_query"] => {
                        format!(
                            "This is a route with a query string: {:?}",
                            url_path.query().collect::<Vec<_>>()
                        )
                    }
                    _ => "Unknown route".to_string(),
                }
            })))
    });
}
