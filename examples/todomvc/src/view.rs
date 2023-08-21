use std::rc::Rc;

use derive_more::Constructor;
use futures_signals::{
    signal::{not, ReadOnlyMutable, Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use silkenweb::{
    clone,
    elements::{
        html::{
            a, button, div, footer, h1, header, input, label, li, section, span, strong, ul,
            Button, Div, Footer, Input, Li, Section, Ul,
        },
        ElementEvents, HtmlElement, HtmlElementEvents,
    },
    node::{element::Element, Node},
    prelude::ParentElement,
    router::url_path,
    task::TaskSignal,
    value::Sig,
};
use silkenweb_signals_ext::SignalProduct;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;

use crate::model::{Filter, TodoApp, TodoItem};

#[derive(Constructor, Clone)]
pub struct TodoAppView {
    app: Rc<TodoApp>,
}

impl TodoAppView {
    pub fn render(&self) -> Section {
        let item_filter = url_path()
            .signal_ref({
                |url_path| match url_path.as_str() {
                    "#/active" => Filter::Active,
                    "#/completed" => Filter::Completed,
                    _ => Filter::All,
                }
            })
            .to_mutable();

        clone!(self.app);
        let input_elem = input()
            .class("new-todo")
            .placeholder("What needs to be done?")
            .on_keyup(move |keyup, input| {
                if keyup.key() == "Enter" {
                    let text = input.value();
                    let text = text.trim().to_string();

                    if !text.is_empty() {
                        app.new_todo(text);
                        input.set_value("");
                    }
                }
            })
            .effect(|elem: &HtmlInputElement| elem.focus().unwrap_throw());

        let app_view = self.clone();
        let body = self
            .is_empty()
            .map(move |is_empty| {
                if is_empty {
                    Vec::new()
                } else {
                    vec![
                        app_view.render_main(item_filter.clone()).into(),
                        app_view.render_footer(item_filter.clone()).into(),
                    ] as Vec<Node>
                }
            })
            .to_signal_vec();

        section()
            .class("todoapp")
            .child(header().child(h1().text("todos")).child(input_elem))
            .children_signal(body)
    }

    fn render_main(&self, item_filter: ReadOnlyMutable<Filter>) -> Section {
        clone!(self.app);
        section()
            .class("main")
            .child(
                input()
                    .id("toggle-all")
                    .class("toggle-all")
                    .r#type("checkbox")
                    .on_change({
                        clone!(app);

                        move |_, elem| app.set_completed_states(elem.checked())
                    })
                    .effect_signal(self.all_completed(), |elem, all_complete| {
                        elem.set_checked(all_complete)
                    }),
            )
            .child(label().r#for("toggle-all"))
            .child(
                ul().class("todo-list").children_signal(
                    self.visible_items_signal(item_filter)
                        .map(move |item| TodoItemView::render(item, app.clone())),
                ),
            )
    }

    fn render_footer(&self, item_filter: ReadOnlyMutable<Filter>) -> Footer {
        footer()
            .class("footer")
            .child(Sig(self.active_count().map(move |active_count| {
                span()
                    .class("todo-count")
                    .child(strong().text(format!("{active_count}")))
                    .text(format!(
                        " item{} left",
                        if active_count == 1 { "" } else { "s" }
                    ))
            })))
            .child(self.render_filters(item_filter))
            .optional_child(Sig(self.render_clear_completed()))
    }

    fn render_filter_link(
        &self,
        filter: Filter,
        item_filter: impl Signal<Item = Filter> + 'static,
        seperator: &str,
    ) -> Li {
        let filter_name = format!("{filter}");

        li().child(
            a().classes(Sig(
                item_filter.map(move |f| (filter == f).then_some("selected"))
            ))
            .href(format!("#/{}", filter_name.to_lowercase()))
            .text(&filter_name),
        )
        .text(seperator)
    }

    fn render_filters(&self, item_filter: ReadOnlyMutable<Filter>) -> Ul {
        ul().class("filters").children([
            self.render_filter_link(Filter::All, item_filter.signal(), " "),
            self.render_filter_link(Filter::Active, item_filter.signal(), " "),
            self.render_filter_link(Filter::Completed, item_filter.signal(), ""),
        ])
    }

    fn render_clear_completed(&self) -> impl Signal<Item = Option<Button>> {
        clone!(self.app);

        self.any_completed().map(move |any_completed| {
            any_completed.then(|| {
                clone!(app);

                button()
                    .class("clear-completed")
                    .text("Clear completed")
                    .on_click(move |_, _| app.clear_completed_todos())
            })
        })
    }

    fn visible_items_signal(
        &self,
        item_filter: ReadOnlyMutable<Filter>,
    ) -> impl SignalVec<Item = Rc<TodoItem>> {
        self.app.items_signal().filter_signal_cloned(move |item| {
            (item.completed(), item_filter.signal()).signal_ref(|completed, item_filter| {
                match item_filter {
                    Filter::All => true,
                    Filter::Active => !*completed,
                    Filter::Completed => *completed,
                }
            })
        })
    }

    fn is_empty(&self) -> impl Signal<Item = bool> {
        self.app.items_signal().is_empty().dedupe()
    }

    fn completed(&self) -> impl SignalVec<Item = bool> {
        self.app.items_signal().map_signal(|todo| todo.completed())
    }

    fn all_completed(&self) -> impl Signal<Item = bool> {
        self.completed()
            .filter(|completed| !completed)
            .is_empty()
            .dedupe()
    }

    fn any_completed(&self) -> impl Signal<Item = bool> {
        not(self
            .completed()
            .filter(|completed| *completed)
            .is_empty()
            .dedupe())
    }

    fn active_count(&self) -> impl Signal<Item = usize> {
        self.completed().filter(|completed| !completed).len()
    }
}

pub struct TodoItemView {
    todo: Rc<TodoItem>,
    app: Rc<TodoApp>,
}

impl TodoItemView {
    pub fn render(todo: Rc<TodoItem>, app: Rc<TodoApp>) -> Li {
        let view = TodoItemView { todo, app };
        li().classes(Sig(view.class()))
            .child(view.render_edit())
            .child(view.render_view())
    }

    fn render_edit(&self) -> Input {
        let todo = &self.todo;
        let app = &self.app;

        input()
            .class("edit")
            .r#type("text")
            .value(Sig(todo.text()))
            .on_focusout({
                clone!(todo, app);
                move |_, input| todo.save_edits(&app, input.value())
            })
            .on_keyup({
                clone!(todo, app);
                move |keyup, input| match keyup.key().as_str() {
                    "Escape" => input.set_value(&todo.revert_edits()),
                    "Enter" => todo.save_edits(&app, input.value()),
                    _ => (),
                }
            })
            .effect_signal(todo.is_editing(), |elem, editing| {
                elem.set_hidden(!editing);

                if editing {
                    elem.focus().unwrap_throw();
                }
            })
    }

    fn render_view(&self) -> Div {
        let todo = &self.todo;
        let app = &self.app;
        let completed_checkbox = input()
            .class("toggle")
            .r#type("checkbox")
            .on_click({
                clone!(todo, app);
                move |_, elem| app.set_completed(&todo, elem.checked())
            })
            .checked(Sig(todo.completed()))
            .effect_signal(todo.completed(), |elem, completed| {
                elem.set_checked(completed)
            });

        div()
            .class("view")
            .child(completed_checkbox)
            .child(label().text(Sig(todo.text())).on_dblclick({
                clone!(todo);
                move |_, _| todo.set_editing()
            }))
            .child(button().class("destroy").on_click({
                clone!(todo, app);
                move |_, _| todo.remove(&app)
            }))
            .effect_signal(todo.is_editing(), |elem, editing| elem.set_hidden(editing))
    }

    fn class(&self) -> impl Signal<Item = impl Iterator<Item = &'static str>> {
        let todo = &self.todo;
        (todo.completed(), todo.is_editing()).signal_ref(|completed, editing| {
            [completed.then(|| "completed"), editing.then(|| "editing")]
                .into_iter()
                .flatten()
        })
    }
}
