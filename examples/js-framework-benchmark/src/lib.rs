use std::{
    cell::{Cell, RefCell},
    ops::DerefMut,
    rc::Rc,
};

use futures_signals::{
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use rand::{
    prelude::{SliceRandom, SmallRng},
    Rng, SeedableRng,
};
use silkenweb::{
    clone,
    dom::{element::ElementBuilder, mount},
    elements::{
        html::{a, button, div, h1, span, table, tbody, td, tr, Div, Table, Tr},
        ElementEvents, HtmlElement, ParentBuilder,
    },
};
use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};

const ADJECTIVES: &[&str] = &[
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy",
];

const COLOURS: &[&str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];
const NOUNS: &[&str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

struct Row {
    id: usize,
    label: Mutable<String>,
}

impl Row {
    fn new(id: usize, rng: &mut impl Rng) -> Rc<Self> {
        Rc::new(Self {
            id,
            label: Mutable::new(format!(
                "{} {} {}",
                ADJECTIVES.choose(rng).unwrap_throw(),
                COLOURS.choose(rng).unwrap_throw(),
                NOUNS.choose(rng).unwrap_throw()
            )),
        })
    }

    fn render(&self, app: Rc<App>) -> Tr {
        let id = self.id;

        tr().class_signal(
            app.selected_row_id
                .signal_ref(move |selected| *selected == Some(id))
                .dedupe()
                .map(|selected| selected.then(|| "danger")),
        )
        .children([
            td().class(["col-md-1"]).text(&id.to_string()),
            td().class(["col-md-4"])
                .child(a().text_signal(self.label.signal_cloned()).on_click({
                    clone!(app);
                    move |_, _| app.select_row(id)
                })),
            td().class(["col-md-1"]).child(
                a().child(
                    span()
                        .class(["glyphicon", "glyphicon-remove"])
                        .attribute("aria-hidden", "true"),
                )
                .on_click(move |_, _| {
                    app.remove_row(id);
                }),
            ),
            td().class(["col-md-6"]),
        ])
        .build()
    }
}

struct App {
    data: MutableVec<Rc<Row>>,
    selected_row_id: Mutable<Option<usize>>,
    next_row_id: Cell<usize>,
    rng: RefCell<SmallRng>,
}

impl App {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            data: MutableVec::new(),
            selected_row_id: Mutable::new(None),
            next_row_id: Cell::new(1),
            rng: RefCell::new(SmallRng::seed_from_u64(0)),
        })
    }

    fn clear(&self) {
        self.data.lock_mut().clear();
        self.selected_row_id.set(None);
    }

    fn append(&self, count: usize) {
        let mut rows = self.data.lock_mut();

        for _ in 0..count {
            rows.push_cloned(self.new_row());
        }
    }

    fn create(&self, count: usize) {
        let new_rows = (0..count).map(|_| self.new_row()).collect();
        self.data.lock_mut().replace_cloned(new_rows);
    }

    fn update(&self) {
        for row in self.data.lock_ref().iter().step_by(10) {
            row.label.lock_mut().push_str(" !!!");
        }
    }

    fn swap(&self) {
        self.data.lock_mut().swap(1, 998);
    }

    fn select_row(&self, row_id: usize) {
        self.selected_row_id.set(Some(row_id));
    }

    fn remove_row(&self, row_id: usize) {
        self.data.lock_mut().retain(|row| row.id != row_id);
    }

    fn new_row(&self) -> Rc<Row> {
        let next_row_id = self.next_row_id.get();
        self.next_row_id.set(next_row_id + 1);
        Row::new(next_row_id, self.rng.borrow_mut().deref_mut())
    }

    fn render(self: Rc<Self>) -> Div {
        div()
            .class(["container"])
            .child(self.clone().render_jumbotron())
            .child(self.render_table())
            .child(
                span()
                    .class(["preloadicon", "glyphicon", "glyphicon-remove"])
                    .attribute("aria-hidden", "true"),
            )
            .build()
    }

    fn render_jumbotron(self: Rc<Self>) -> Div {
        div()
            .class(["jumbotron"])
            .child(
                div().class(["row"]).children([
                    div()
                        .class(["col-md-6"])
                        .child(h1().text("Silkenweb keyed")),
                    div()
                        .class(["col-md-6"])
                        .child(self.render_action_buttons()),
                ]),
            )
            .build()
    }

    fn render_action_buttons(self: &Rc<Self>) -> Div {
        div()
            .class(["row"])
            .children([
                self.render_button("run", "Create 1,000 rows", |app| app.create(1_000)),
                self.render_button("runlots", "Create 10,000 rows", |app| app.create(10_000)),
                self.render_button("add", "Append 1,000 rows", |app| app.append(1_000)),
                self.render_button("update", "Update every 10th row", |app| app.update()),
                self.render_button("clear", "Clear", |app| app.clear()),
                self.render_button("swaprows", "Swap Rows", |app| app.swap()),
            ])
            .build()
    }

    fn render_button<F>(self: &Rc<Self>, id: &str, title: &str, mut on_click: F) -> Div
    where
        F: FnMut(&Self) + 'static,
    {
        let app = self.clone();

        div()
            .class(["col-sm-6", "smallpad"])
            .child(
                button()
                    .class(["btn", "btn-primary", "btn-block"])
                    .r#type("button")
                    .id(id)
                    .text(title)
                    .on_click(move |_, _| on_click(&app)),
            )
            .build()
    }

    fn render_table(self: Rc<Self>) -> Table {
        table()
            .class(["table", "table-hover", "table-striped", "test-data"])
            .child(
                tbody().children_signal(
                    self.data
                        .signal_vec_cloned()
                        .map(move |row| row.render(self.clone())),
                ),
            )
            .build()
    }
}

#[wasm_bindgen(start)]
pub fn main_js() {
    mount("main", App::new().render());
}
