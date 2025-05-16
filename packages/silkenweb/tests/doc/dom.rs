use silkenweb::{
    document::Document,
    dom::{Dry, Hydro, Template, Wet},
    elements::html::{p, P},
    hydration::hydrate,
    node::element::{Const, TextParentElement},
};

pub fn dry() {
    let app: P<Dry> = p().text("Hello, world!");

    assert_eq!(app.freeze().to_string(), "<p>Hello, world!</p>");
}

pub async fn hydro() {
    let app: P<Hydro> = p().text("Hello, world!");

    hydrate("app-id", app).await;
}

pub fn wet() {
    let app: P<Wet> = p().text("Hello, world!");

    Wet::mount("app-id", app);
}

pub fn template() {
    let elem: P<Template<String, Dry>> = p().on_instantiate(|p, message| p.text(message));
    let template: P<Template<String, Dry>, Const> = elem.freeze();
    let hello = template.instantiate(&"Hello, world!".to_string());
    let goodbye = template.instantiate(&"Goodbye!".to_string());

    assert_eq!(hello.freeze().to_string(), "<p>Hello, world!</p>");
    assert_eq!(goodbye.freeze().to_string(), "<p>Goodbye!</p>");
}
