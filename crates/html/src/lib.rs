use surfinia_core::{tag, Element, ElementBuilder};

pub fn div() -> DivBuilder {
    DivBuilder(HtmlElementBuilder::new("div"))
}

pub struct DivBuilder(HtmlElementBuilder);

impl DivBuilder {
    pub fn id(self, value: impl AsRef<str>) -> Self {
        Self(self.0.id(value))
    }

    pub fn text(self, child: impl AsRef<str>) -> Self {
        Self(self.0.text(child))
    }

    pub fn child<Child: Parent<Div>>(self, c: Child) -> Self {
        Self(self.0.child(c.into()))
    }

    pub fn build(self) -> Div {
        Div(self.0.build())
    }
}

impl From<DivBuilder> for Element {
    fn from(div: DivBuilder) -> Self {
        div.build().into()
    }
}

pub struct Div(Element);

impl From<Div> for Element {
    fn from(div: Div) -> Self {
        div.0
    }
}

impl From<Element> for Div {
    fn from(elem: Element) -> Self {
        Div(elem)
    }
}

impl<Child: content_category::Flow> ParentCategory<Div> for Child {}

impl content_category::Flow for Div {}
impl content_category::Palpable for Div {}

// TODO: Blanket impls so we only define content categories for Div, not
// DivBuilder
impl content_category::Flow for DivBuilder {}
impl content_category::Palpable for DivBuilder {}

pub fn button() -> ButtonBuilder {
    ButtonBuilder(HtmlElementBuilder::new("button"))
}

pub struct ButtonBuilder(HtmlElementBuilder);

impl ButtonBuilder {
    pub fn id(self, value: impl AsRef<str>) -> Self {
        Self(self.0.id(value))
    }

    pub fn text(self, child: impl AsRef<str>) -> Self {
        Self(self.0.text(child))
    }

    pub fn child<Child: Parent<Button>>(self, c: Child) -> Self {
        Self(self.0.child(c.into()))
    }

    pub fn on_click(self, f: impl 'static + FnMut()) -> Self {
        Self(self.0.on_click(f))
    }

    pub fn build(self) -> Button {
        Button(self.0.build())
    }
}

impl From<ButtonBuilder> for Element {
    fn from(div: ButtonBuilder) -> Self {
        div.build().into()
    }
}

pub struct Button(Element);

impl From<Button> for Element {
    fn from(button: Button) -> Self {
        button.0
    }
}

impl From<Element> for Button {
    fn from(elem: Element) -> Self {
        Button(elem)
    }
}

impl<Child: content_category::Flow> ParentCategory<Button> for Child {}

impl content_category::Flow for Button {}
impl content_category::Palpable for Button {}
impl content_category::Flow for ButtonBuilder {}
impl content_category::Palpable for ButtonBuilder {}

struct HtmlElementBuilder(ElementBuilder);

impl HtmlElementBuilder {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self(tag(name))
    }

    pub fn id(self, value: impl AsRef<str>) -> Self {
        Self(self.0.attribute("id", value))
    }

    pub fn on_click(self, mut f: impl 'static + FnMut()) -> Self {
        Self(self.0.on("click", move |_| f()))
    }

    pub fn text(self, child: impl AsRef<str>) -> Self {
        Self(self.0.text(child))
    }

    pub fn child(self, c: impl Into<Element>) -> Self {
        Self(self.0.child(c.into()))
    }

    pub fn build(self) -> Element {
        self.0.build()
    }
}

pub trait Parent<T>: Into<Element> {}

impl<Child, T: ParentCategory<Child> + Into<Element>> Parent<Child> for T {}

pub trait ParentCategory<T> {}

pub mod content_category {
    macro_rules! content_categories {
        ($($name:ident),* $(,)?) => { $(pub trait $name {})* }
    }

    content_categories!(
        Flow,
        Phrasing,
        Interactive,
        Listed,
        Labelable,
        Submittable,
        Palpable,
    );
}
