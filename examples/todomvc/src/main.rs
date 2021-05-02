#[macro_use]
extern crate derive_more;

use std::{cell::Cell, iter, rc::Rc};

use silkenweb::{
    accumulators::{SumElement, SumHandle, SumTotal},
    clone,
    element_list::ElementList,
    elements::{
        a, button, div, footer, h1, header, input, label, li, section, span, strong, ul, Button,
        Div, Footer, Input, Li, LiBuilder, Section, Ul,
    },
    mount,
    router::url,
    signal::{ReadSignal, Signal, WriteSignal, ZipSignal},
    Builder,
};
use web_sys::{HtmlDivElement, HtmlInputElement};

fn main() {
    console_error_panic_hook::set_once();
    mount("app", TodoApp::new().render());
}

#[derive(Clone)]
struct TodoApp {
    items: Signal<ElementList<usize, TodoItem>>,
    id: Rc<Cell<usize>>,
    filter: ReadSignal<Filter>,
    active_count: SumTotal<usize>,
}

impl TodoApp {
    fn new() -> Self {
        let items = Signal::new(ElementList::new(
            ul().class("todo-list"),
            TodoItem::render,
            iter::empty(),
        ));

        let filter = url().map({
            let write_items = items.write();
            move |url| match url.fragment() {
                Some("/active") => {
                    Self::set_filter(&write_items, |item| {
                        item.completed.read().map(|completed| !completed)
                    });
                    Filter::Active
                }
                Some("/completed") => {
                    Self::set_filter(&write_items, |item| item.completed.read());
                    Filter::Completed
                }
                _ => {
                    Self::set_filter(&write_items, |_| Signal::new(true).read());
                    Filter::All
                }
            }
        });

        Self {
            items,
            id: Rc::new(Cell::new(0)),
            filter,
            active_count: SumTotal::default(),
        }
    }

    fn set_filter(
        write_items: &WriteSignal<ElementList<usize, TodoItem>>,
        f: impl 'static + Fn(&TodoItem) -> ReadSignal<bool>,
    ) {
        write_items.mutate(|items| items.filter(f));
    }

    fn render(&self) -> Section {
        let input_elem = input()
            .class("new-todo")
            .placeholder("What needs to be done?")
            .on_keyup({
                let self_ = self.clone();

                move |keyup, input| {
                    if keyup.key() == "Enter" {
                        let text = input.value();
                        let text = text.trim().to_string();

                        if !text.is_empty() {
                            self_.push(text);
                            input.set_value("");
                        }
                    }
                }
            })
            .effect(|elem: &HtmlInputElement| elem.focus().unwrap())
            .build();

        section()
            .class("todoapp")
            .child(header().child(h1().text("todos")).child(input_elem))
            .child(
                section()
                    .class("main")
                    .child(self.define_todo_items())
                    .child(self.items.read()),
            )
            .child(self.define_footer())
            .build()
    }

    fn define_todo_items(&self) -> ReadSignal<Div> {
        let is_empty = self.items.read().map(ElementList::is_empty).only_changes();
        let all_complete = self
            .active_count
            .read()
            .map(|&active_count| active_count == 0)
            .only_changes();
        let write_items = self.items.write();

        is_empty.map(move |&is_empty| {
            if is_empty {
                div()
            } else {
                clone!(all_complete, write_items);
                let initial_complete = *all_complete.current();

                div()
                    .child(
                        input()
                            .id("toggle-all")
                            .class("toggle-all")
                            .type_("checkbox")
                            .checked(initial_complete)
                            .on_change({
                                clone!(all_complete);
                                move |_, _| {
                                    let new_completed = !*all_complete.current();

                                    write_items.mutate(move |items| {
                                        for item in items.values() {
                                            item.completed.write().set(new_completed);
                                        }
                                    })
                                }
                            })
                            .effect(all_complete.map(|&complete| {
                                move |elem: &HtmlInputElement| elem.set_checked(complete)
                            })),
                    )
                    .child(label().for_("toggle-all"))
            }
            .build()
        })
    }

    fn define_footer(&self) -> ReadSignal<Option<Footer>> {
        self.items
            .read()
            .map(ElementList::is_empty)
            .only_changes()
            .map({
                let self_ = self.clone();

                move |&is_empty| {
                    if is_empty {
                        None
                    } else {
                        Some(
                            footer()
                                .class("footer")
                                .child(self_.active_count.read().map(move |&active_count| {
                                    span()
                                        .class("todo-count")
                                        .child(strong().text(format!("{}", active_count)))
                                        .text(format!(
                                            " item{} left",
                                            if active_count == 1 { "" } else { "s" }
                                        ))
                                }))
                                .child(self_.define_filters())
                                .child(self_.define_clear_completed())
                                .build(),
                        )
                    }
                }
            })
    }

    fn define_filter_link(&self, filter: Filter, seperator: &str) -> LiBuilder {
        let filter_name = format!("{}", filter);

        li().child(
            a().class(
                self.filter
                    .map(move |f| if filter == *f { "selected" } else { "" }),
            )
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

    fn define_clear_completed(&self) -> ReadSignal<Option<Button>> {
        let write_items = self.items.write();
        let items_len = self.items.read().map(ElementList::len);
        let any_completed = (self.active_count.read(), items_len)
            .map(|&active_count, &items_len| active_count != items_len)
            .only_changes();

        any_completed.map(move |&any_completed| {
            clone!(write_items);

            if any_completed {
                Some(
                    button()
                        .class("clear-completed")
                        .text("Clear completed")
                        .on_click(move |_, _| {
                            write_items.mutate(|items| {
                                items.retain(|item| !*item.completed.read().current())
                            })
                        })
                        .build(),
                )
            } else {
                None
            }
        })
    }

    fn push(&self, text: String) {
        let self_ = self.clone();

        self.items.write().mutate(move |ts| {
            let current_id = self_.id.replace(self_.id.get() + 1);
            let parent = self_.items.write();
            ts.insert(
                current_id,
                TodoItem::new(current_id, text, false, parent, &self_.active_count),
            );
        })
    }
}

#[derive(Clone)]
struct TodoItem {
    id: usize,
    text: Signal<String>,
    completed: Signal<bool>,
    editing: Signal<bool>,
    parent: WriteSignal<ElementList<usize, Self>>,
    active_count: ReadSignal<SumHandle>,
}

impl TodoItem {
    fn new(
        id: usize,
        text: impl Into<String>,
        completed: bool,
        parent: WriteSignal<ElementList<usize, Self>>,
        active_count: &SumTotal<usize>,
    ) -> Self {
        let completed = Signal::new(completed);
        let active_count = completed
            .read()
            .map(|completed| (!completed) as usize)
            .map_to(SumElement::new(active_count));

        Self {
            id,
            text: Signal::new(text.into()),
            completed,
            editing: Signal::new(false),
            parent,
            active_count,
        }
    }

    fn render(&self) -> Li {
        li().class(self.class())
            .child(self.define_edit())
            .child(self.define_view())
            .build()
    }

    fn define_edit(&self) -> Input {
        input()
            .class("edit")
            .type_("text")
            .value(&self.text.read())
            .on_focusout({
                let self_ = self.clone();
                move |_, input| self_.save_edits(&input)
            })
            .on_keyup({
                let self_ = self.clone();
                move |keyup, input| match keyup.key().as_str() {
                    "Escape" => {
                        input.set_value(&self_.text.read().current());
                        self_.editing.write().set(false)
                    }
                    "Enter" => self_.save_edits(&input),
                    _ => (),
                }
            })
            .effect(self.editing.read().map(|&editing| {
                move |elem: &HtmlInputElement| {
                    elem.set_hidden(!editing);

                    if editing {
                        elem.focus().unwrap()
                    }
                }
            }))
            .build()
    }

    fn define_view(&self) -> Div {
        let completed = self.completed.read();
        let completed_checkbox = input()
            .class("toggle")
            .type_("checkbox")
            .on_click({
                let set_completed = self.completed.write();
                move |_, _| set_completed.replace(|completed| !completed)
            })
            .checked(*completed.current())
            .effect(
                completed
                    .map(|&complete| move |elem: &HtmlInputElement| elem.set_checked(complete)),
            );
        let parent = self.parent.clone();
        let id = self.id;

        div()
            .class("view")
            .child(completed_checkbox)
            .child(label().text(self.text.read()).on_dblclick({
                let set_editing = self.editing.write();
                move |_, _| set_editing.set(true)
            }))
            .child(
                button()
                    .class("destroy")
                    .on_click(move |_, _| parent.mutate(move |p| p.remove(&id))),
            )
            .effect(
                self.editing
                    .read()
                    .map(|&editing| move |elem: &HtmlDivElement| elem.set_hidden(editing)),
            )
            .build()
    }

    fn save_edits(&self, input: &HtmlInputElement) {
        let text = input.value();
        let text = text.trim();

        if text.is_empty() {
            let id = self.id;
            self.parent.mutate(move |p| p.remove(&id));
        } else if *self.editing.read().current() {
            self.text.write().set(text.to_string());
            self.editing.write().set(false);
        }
    }

    fn class(&self) -> ReadSignal<String> {
        (self.completed.read(), self.editing.read()).map(|&completed, &editing| {
            vec![(completed, "completed"), (editing, "editing")]
                .into_iter()
                .filter_map(|(flag, name)| if flag { Some(name) } else { None })
                .collect::<Vec<_>>()
                .join(" ")
        })
    }
}

#[derive(Display, Copy, Clone, Eq, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}
