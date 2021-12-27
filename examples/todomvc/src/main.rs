#[macro_use]
extern crate derive_more;

use std::{cell::Cell, rc::Rc};

use futures_signals::{
    signal::{Broadcaster, Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
use serde::{Deserialize, Serialize};
use silkenweb::{
    clone,
    elements::{
        a, button, div, footer, h1, header, input, label, li, section, span, strong, ul, Button,
        Div, Footer, Input, Li, LiBuilder, Section, Ul,
    },
    local_storage, mount, product,
    router::url,
    signal, Builder, Effects, HtmlElement, ParentBuilder,
};
use web_sys::HtmlInputElement;

fn main() {
    console_error_panic_hook::set_once();

    let item_filter = url().signal_cloned().map({
        |url| match url.hash().as_str() {
            "#/active" => Filter::Active,
            "#/completed" => Filter::Completed,
            _ => Filter::All,
        }
    });

    mount("app", TodoApp::render(TodoApp::load(), item_filter));
}

#[derive(Serialize, Deserialize)]
struct TodoApp {
    todo_id: Cell<u128>,
    items: MutableVec<Rc<TodoItem>>,
}

impl TodoApp {
    fn load() -> Rc<Self> {
        Rc::new(
            if let Some(app_str) =
                local_storage().and_then(|storage| storage.get_item(STORAGE_KEY).ok().flatten())
            {
                serde_json::from_str(&app_str).unwrap()
            } else {
                Self {
                    todo_id: Cell::new(0),
                    items: MutableVec::new(),
                }
            },
        )
    }

    fn save(&self) {
        if let Some(storage) = local_storage() {
            storage
                .set_item(STORAGE_KEY, &serde_json::to_string(self).unwrap())
                .unwrap();
        }
    }

    fn render(app: Rc<Self>, item_filter: impl 'static + Signal<Item = Filter>) -> Section {
        let input_elem = input()
            .class("new-todo")
            .placeholder("What needs to be done?")
            .on_keyup({
                clone!(app);

                move |keyup, input| {
                    if keyup.key() == "Enter" {
                        let text = input.value();
                        let text = text.trim().to_string();

                        if !text.is_empty() {
                            let todo_id = app.todo_id.get();
                            app.todo_id.set(todo_id + 1);

                            app.items
                                .lock_mut()
                                .push_cloned(TodoItem::new(todo_id, text));
                            input.set_value("");
                            app.save();
                        }
                    }
                }
            })
            .effect(|elem: &HtmlInputElement| elem.focus().unwrap())
            .build();

        let completed = app
            .items
            .signal_vec_cloned()
            .map_signal(|todo| todo.completed.signal());
        let active_count = Broadcaster::new(completed.filter(|completed| !completed).len());
        let item_filter = Broadcaster::new(item_filter);
        let is_empty = Broadcaster::new(app.items.signal_vec_cloned().is_empty());

        section()
            .class("todoapp")
            .child(header().child(h1().text("todos")).child(input_elem))
            .optional_child_signal(Self::define_main(
                app.clone(),
                item_filter.signal(),
                active_count.signal(),
                is_empty.signal(),
            ))
            .optional_child_signal(Self::define_footer(
                app,
                item_filter.signal(),
                active_count.signal(),
                is_empty.signal(),
            ))
            .build()
    }

    fn define_main(
        app: Rc<Self>,
        item_filter: impl 'static + Signal<Item = Filter>,
        active_count: impl 'static + Signal<Item = usize>,
        is_empty: impl 'static + Signal<Item = bool>,
    ) -> impl Signal<Item = Option<Section>> {
        let item_filter = Broadcaster::new(item_filter);
        let all_complete = Broadcaster::new(active_count.map(|count| count == 0).dedupe());

        is_empty.map(move |is_empty| {
            if is_empty {
                None
            } else {
                Some(
                    section()
                        .class("main")
                        .child(
                            input()
                                .id("toggle-all")
                                .class("toggle-all")
                                .type_("checkbox")
                                .checked(signal(all_complete.signal()))
                                .on_change({
                                    clone!(app);

                                    move |_, elem| {
                                        let checked = elem.checked();

                                        for item in app.items.lock_ref().iter() {
                                            item.completed.set_neq(checked);
                                        }

                                        app.save();
                                    }
                                })
                                .effect_signal(all_complete.signal(), |elem, all_complete| {
                                    elem.set_checked(all_complete)
                                }),
                        )
                        .child(label().for_("toggle-all"))
                        .child(ul().class("todo-list").children_signal(
                            app.visible_items_signal(item_filter.signal()).map({
                                clone!(app);
                                move |item| TodoItem::render(item, app.clone())
                            }),
                        ))
                        .build(),
                )
            }
        })
    }

    fn define_footer(
        app: Rc<Self>,
        item_filter: impl 'static + Signal<Item = Filter>,
        active_count: impl 'static + Signal<Item = usize>,
        is_empty: impl 'static + Signal<Item = bool>,
    ) -> impl Signal<Item = Option<Footer>> {
        let active_count = Broadcaster::new(active_count);
        let item_filter = Broadcaster::new(item_filter);

        is_empty.map({
            clone!(app);

            move |is_empty| {
                if is_empty {
                    None
                } else {
                    Some(
                        footer()
                            .class("footer")
                            .child_signal(active_count.signal().map(move |active_count| {
                                span()
                                    .class("todo-count")
                                    .child(strong().text(&format!("{}", active_count)))
                                    .text(&format!(
                                        " item{} left",
                                        if active_count == 1 { "" } else { "s" }
                                    ))
                            }))
                            .child(app.define_filters(item_filter.signal()))
                            .optional_child_signal(Self::define_clear_completed(
                                app.clone(),
                                active_count.signal(),
                            ))
                            .build(),
                    )
                }
            }
        })
    }

    fn define_filter_link(
        &self,
        filter: Filter,
        item_filter: impl 'static + Signal<Item = Filter>,
        seperator: &str,
    ) -> LiBuilder {
        let filter_name = format!("{}", filter);

        li().child(
            a().class(signal(item_filter.map(move |f| {
                if filter == f { "selected" } else { "" }.to_string()
            })))
            .href(format!("/#/{}", filter_name.to_lowercase()))
            .text(&filter_name),
        )
        .text(seperator)
    }

    fn define_filters(&self, item_filter: impl 'static + Signal<Item = Filter>) -> Ul {
        let item_filter = Broadcaster::new(item_filter);
        ul().class("filters")
            .child(self.define_filter_link(Filter::All, item_filter.signal(), " "))
            .child(self.define_filter_link(Filter::Active, item_filter.signal(), " "))
            .child(self.define_filter_link(Filter::Completed, item_filter.signal(), ""))
            .build()
    }

    fn define_clear_completed(
        app: Rc<Self>,
        active_count: impl 'static + Signal<Item = usize>,
    ) -> impl Signal<Item = Option<Button>> {
        product!(app.items.signal_vec_cloned().len(), active_count).map(
            move |(item_count, active_count)| {
                let any_completed = item_count != active_count;
                clone!(app);

                if any_completed {
                    Some(
                        button()
                            .class("clear-completed")
                            .text("Clear completed")
                            .on_click(move |_, _| {
                                app.items.lock_mut().retain(|item| !item.completed.get());
                                app.save();
                            })
                            .build(),
                    )
                } else {
                    None
                }
            },
        )
    }

    fn visible_items_signal(
        &self,
        item_filter: impl Signal<Item = Filter>,
    ) -> impl SignalVec<Item = Rc<TodoItem>> {
        let item_filter = Broadcaster::new(item_filter);

        self.items_signal().filter_signal_cloned(move |item| {
            product!(item.completed.signal(), item_filter.signal()).map(
                |(completed, item_filter)| match item_filter {
                    Filter::All => true,
                    Filter::Active => !completed,
                    Filter::Completed => completed,
                },
            )
        })
    }

    fn items_signal(&self) -> impl 'static + SignalVec<Item = Rc<TodoItem>> {
        self.items.signal_vec_cloned()
    }

    fn remove_item(&self, todo_id: u128) {
        self.items.lock_mut().retain(|item| item.id != todo_id);
    }
}

#[derive(Serialize, Deserialize)]
struct TodoItem {
    id: u128,
    text: Mutable<String>,
    completed: Mutable<bool>,
    #[serde(skip)]
    editing: Mutable<bool>,
}

impl TodoItem {
    fn new(id: u128, text: String) -> Rc<Self> {
        Rc::new(Self {
            id,
            text: Mutable::new(text),
            completed: Mutable::new(false),
            editing: Mutable::new(false),
        })
    }

    fn render(todo: Rc<Self>, app: Rc<TodoApp>) -> Li {
        li().class(signal(todo.class()))
            .child(Self::define_edit(&todo, &app))
            .child(Self::define_view(&todo, app))
            .build()
    }

    fn define_edit(todo: &Rc<Self>, app: &Rc<TodoApp>) -> Input {
        input()
            .class("edit")
            .type_("text")
            .value(signal(todo.text()))
            .on_focusout({
                clone!(todo, app);
                move |_, input| todo.save_edits(&app, &input)
            })
            .on_keyup({
                clone!(todo, app);
                move |keyup, input| match keyup.key().as_str() {
                    "Escape" => {
                        input.set_value(&todo.text.get_cloned());
                        todo.editing.set(false);
                    }
                    "Enter" => {
                        todo.save_edits(&app, &input);
                        app.save();
                    }
                    _ => (),
                }
            })
            .effect_signal(todo.editing.signal(), |elem, editing| {
                elem.set_hidden(!editing);

                if editing {
                    elem.focus().unwrap();
                }
            })
            .build()
    }

    fn define_view(todo: &Rc<TodoItem>, app: Rc<TodoApp>) -> Div {
        let completed_checkbox = input()
            .class("toggle")
            .type_("checkbox")
            .on_click({
                clone!(todo, app);
                move |_, elem| {
                    todo.completed.set(elem.checked());
                    app.save();
                }
            })
            .checked(signal(todo.completed()))
            .effect_signal(todo.completed(), |elem, completed| {
                elem.set_checked(completed)
            });

        div()
            .class("view")
            .child(completed_checkbox)
            .child(label().text_signal(todo.text()).on_dblclick({
                clone!(todo);
                move |_, _| todo.editing.set(true)
            }))
            .child(button().class("destroy").on_click({
                clone!(todo);
                move |_, _| {
                    app.remove_item(todo.id);
                    app.save();
                }
            }))
            .effect_signal(todo.editing.signal(), |elem, editing| {
                elem.set_hidden(editing)
            })
            .build()
    }

    fn save_edits(&self, app: &TodoApp, input: &HtmlInputElement) {
        if !self.editing.get() {
            return;
        }

        let text = input.value();
        let text = text.trim();
        let id = self.id;

        if text.is_empty() {
            app.remove_item(id);
        } else {
            self.text.set(text.to_string());
            self.editing.set(false);
        }
    }

    fn class(&self) -> impl Signal<Item = String> {
        product!(self.completed(), self.editing.signal()).map(|(completed, editing)| {
            vec![(completed, "completed"), (editing, "editing")]
                .into_iter()
                .filter_map(|(flag, name)| if flag { Some(name) } else { None })
                .collect::<Vec<_>>()
                .join(" ")
        })
    }

    fn text(&self) -> impl Signal<Item = String> {
        self.text.signal_cloned()
    }

    fn completed(&self) -> impl Signal<Item = bool> {
        self.completed.signal()
    }
}

#[derive(Display, Copy, Clone, Eq, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

const STORAGE_KEY: &str = "silkenweb-examples-todomvc";
