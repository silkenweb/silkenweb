#[macro_use]
extern crate derive_more;

use std::{cell::Cell, cmp::max, iter, rc::Rc};

use futures_signals::{
    map_ref,
    signal::{not, Broadcaster, Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
use silkenweb::{
    clone,
    elements::{
        a, button, div, footer, h1, header, input, label, li, section, span, strong, ul, Button,
        Div, Footer, Input, Li, LiBuilder, Section, Ul,
    },
    mount,
    router::url,
    signal, Builder, Effects, ParentBuilder,
};
use web_sys::HtmlInputElement;

fn main() {
    console_error_panic_hook::set_once();
    mount("app", TodoApp::render(TodoApp::new()));
}

struct TodoApp {
    items: MutableVec<Rc<TodoItem>>,
    filter: Mutable<Filter>,
}

impl TodoApp {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            items: MutableVec::new(),
            filter: Mutable::new(Filter::All),
        })
    }

    fn render(app: Rc<Self>) -> Section {
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
                            app.items.lock_mut().push_cloned(TodoItem::new(text));
                            input.set_value("");
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

        section()
            .class("todoapp")
            .child(header().child(h1().text("todos")).child(input_elem))
            .child(
                section()
                    .class("main")
                    .child_signal(Self::define_todo_items(app.clone(), active_count.signal()))
                    .child(
                        ul().class("todo-list").children_signal(
                            app.items
                                .signal_vec_cloned()
                                .map(|item| TodoItem::render(item)),
                        ),
                    ),
            )
            .optional_child_signal(Self::define_footer(app, active_count.signal()))
            .build()
    }

    fn items_signal(&self) -> impl SignalVec<Item = Rc<TodoItem>> {
        self.items.signal_vec_cloned()
    }

    fn define_todo_items(
        app: Rc<Self>,
        active_count: impl 'static + Signal<Item = usize>,
    ) -> impl Signal<Item = Div> {
        let is_empty = app.items_signal().is_empty();
        let all_complete = Broadcaster::new(active_count.map(|count| count == 0).dedupe());

        is_empty.map(move |is_empty| {
            if is_empty {
                div()
            } else {
                div()
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
                                }
                            })
                            .effect_signal(all_complete.signal(), |elem, all_complete| {
                                elem.set_checked(all_complete)
                            }),
                    )
                    .child(label().for_("toggle-all"))
            }
            .build()
        })
    }

    fn define_footer(
        app: Rc<Self>,
        active_count: impl 'static + Signal<Item = usize>,
    ) -> impl Signal<Item = Option<Footer>> {
        let active_count = Broadcaster::new(active_count);

        app.items_signal().is_empty().map({
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
                                    .child(strong().text(format!("{}", active_count)))
                                    .text(format!(
                                        " item{} left",
                                        if active_count == 1 { "" } else { "s" }
                                    ))
                            }))
                            .child(app.define_filters())
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

    fn define_filter_link(&self, filter: Filter, seperator: &str) -> LiBuilder {
        let filter_name = format!("{}", filter);

        li().child(
            a().class(signal(self.filter.signal().map(move |f| {
                if filter == f { "selected" } else { "" }.to_string()
            })))
            .href(format!("/#/{}", filter_name.to_lowercase()))
            .text(&filter_name),
        )
        .text(seperator)
    }

    fn define_filters(&self) -> Ul {
        ul().class("filters")
            .child(self.define_filter_link(Filter::All, " "))
            .child(self.define_filter_link(Filter::Active, " "))
            .child(self.define_filter_link(Filter::Completed, ""))
            .build()
    }

    fn define_clear_completed(
        app: Rc<Self>,
        active_count: impl 'static + Signal<Item = usize>,
    ) -> impl Signal<Item = Option<Button>> {
        let item_count = app.items.signal_vec_cloned().len();

        // TODO: Combine signals to tuples (signal_product! macro)
        map_ref!(item_count, active_count => (*item_count, *active_count)).map(
            move |(item_count, active_count)| {
                let any_completed = item_count != active_count;
                clone!(app);

                if any_completed {
                    Some(
                        button()
                            .class("clear-completed")
                            .text("Clear completed")
                            .on_click(move |_, _| {
                                app.items.lock_mut().retain(|item| !item.completed.get())
                            })
                            .build(),
                    )
                } else {
                    None
                }
            },
        )
    }
}

struct TodoItem {
    text: Mutable<String>,
    completed: Mutable<bool>,
    editing: Mutable<bool>,
}

impl TodoItem {
    fn new(text: String) -> Rc<Self> {
        Rc::new(Self {
            text: Mutable::new(text),
            completed: Mutable::new(false),
            editing: Mutable::new(false),
        })
    }

    fn render(todo: Rc<Self>) -> Li {
        li().class(signal(todo.class()))
            .child(Self::define_edit(&todo))
            .child(Self::define_view(&todo))
            .build()
    }

    fn define_edit(todo: &Rc<Self>) -> Input {
        input()
            .class("edit")
            .type_("text")
            .value(signal(todo.text()))
            .on_focusout({
                clone!(todo);
                move |_, input| todo.save_edits(&input)
            })
            .on_keyup({
                clone!(todo);
                move |keyup, input| match keyup.key().as_str() {
                    "Escape" => {
                        input.set_value(&todo.text.get_cloned());
                        todo.editing.set(false);
                    }
                    "Enter" => todo.save_edits(&input),
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

    fn define_view(todo: &Rc<TodoItem>) -> Div {
        let completed_checkbox = input()
            .class("toggle")
            .type_("checkbox")
            .on_click({
                clone!(todo);
                move |_, elem| todo.completed.set(elem.checked())
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
            .child(button().class("destroy").on_click(move |_, _| todo!()))
            .effect_signal(todo.editing.signal(), |elem, editing| {
                elem.set_hidden(editing)
            })
            .build()
    }

    fn save_edits(&self, input: &HtmlInputElement) {
        todo!()
    }

    fn class(&self) -> impl Signal<Item = String> {
        let completed = self.completed();
        let editing = self.editing.signal();

        map_ref!(completed, editing => (*completed, *editing)).map(|(completed, editing)| {
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
