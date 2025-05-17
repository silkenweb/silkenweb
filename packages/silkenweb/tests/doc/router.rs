use silkenweb::{
    dom::DefaultDom,
    elements::{
        html::{a, button, div, p},
        ElementEvents,
    },
    mount,
    node::element::{ParentElement, TextParentElement},
    router::{self, anchor, link_clicked, UrlPath},
    value::Sig,
};

pub fn module_example() {
    div::<DefaultDom>()
        .child(
            button()
                .on_click(|_, _| router::set_url_path("route_1"))
                .text("Go to route 1"),
        )
        .child(
            button()
                .on_click(|_, _| router::set_url_path("route_2"))
                .text("Go to route 2"),
        )
        .child(p().text(Sig(
            router::url_path().signal_ref(|url_path| format!("URL Path is: {url_path}")),
        )));
}

pub fn url_path() {
    assert_eq!(UrlPath::new("path?query_string").path(), "path");
    assert_eq!(UrlPath::new("?query_string").path(), "");
    assert_eq!(UrlPath::new("?").path(), "");
    assert_eq!(UrlPath::new("").path(), "");
}

pub fn url_path_components() {
    let path = UrlPath::new("path1/path2/path3");
    let components: Vec<&str> = path.path_components().collect();
    assert_eq!(&components, &["path1", "path2", "path3"]);

    let path = UrlPath::new("");
    assert_eq!(path.path_components().next(), None);

    let path = UrlPath::new("path1//path2"); // Note the double `'/'`
    let components: Vec<&str> = path.path_components().collect();
    assert_eq!(&components, &["path1", "", "path2"]);
}

pub fn url_query_string() {
    assert_eq!(
        UrlPath::new("path?query_string").query_string(),
        "query_string"
    );
    assert_eq!(UrlPath::new("?query_string").query_string(), "query_string");
    assert_eq!(UrlPath::new("?").query_string(), "");
    assert_eq!(UrlPath::new("").query_string(), "");
    assert_eq!(UrlPath::new("#hash").query_string(), "");
    assert_eq!(
        UrlPath::new("?query_string#hash").query_string(),
        "query_string"
    );
}

pub fn url_query() {
    let path = UrlPath::new("path?x=1&y=2&flag");
    let kv_args: Vec<(&str, Option<&str>)> = path.query().collect();
    assert_eq!(
        &kv_args,
        &[("x", Some("1")), ("y", Some("2")), ("flag", None)]
    );
}

pub fn url_hash() {
    assert_eq!(UrlPath::new("path?query_string#hash").hash(), "hash");
    assert_eq!(UrlPath::new("#hash").hash(), "hash");
    assert_eq!(UrlPath::new("#").hash(), "");
    assert_eq!(UrlPath::new("").hash(), "");
}

pub fn anchor_example() {
    let app = anchor("/my-path").text("click me");
    mount("app", app);
}

pub fn link_clicked_example() {
    let path = "/my_path";
    let app = a().href(path).text("click me").on_click(link_clicked(path));
    mount("app", app);
}
