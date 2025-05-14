use silkenweb::{
    elements::{
        html::{button, div, p, Button},
        ElementEvents,
    },
    log_panics, mount,
    node::element::{ParentElement, TextParentElement},
    router::{self, UrlPath},
    value::Sig,
};

fn main() {
    log_panics();

    mount("app", {
        div()
            .child(route_button("", "Root route"))
            .child(route_button("basic", "Basic route"))
            .child(route_button("with_args/arg1/arg2", "Go to route with args"))
            .child(route_button(
                "with_query?x=1&y=2&flag",
                "Go to route with query string",
            ))
            .child(p().text(Sig(router::url_path().signal_ref(|url_path| {
                let path_components = url_path.path_components_vec();

                match &path_components[..] {
                    [] => "Root route!".to_string(),
                    ["basic"] => "Basic route".to_string(),
                    ["with_args", args @ ..] => format!("Route with args {args:?}"),
                    ["with_query"] => format!("Route with query: {:?}", url_path.query_map()),
                    _ => "Unknown route".to_string(),
                }
            }))))
    });
}

fn route_button(route: &str, description: &str) -> Button {
    let route = UrlPath::new(route);
    button()
        .on_click(move |_, _| router::set_url_path(route.clone()))
        .text(description)
}
