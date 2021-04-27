#[macro_use]
extern crate derive_more;

use std::{cell::RefCell, iter, rc::Rc};

use silkenweb::{
    accumulators::{IncludeSum, Sum, SumTotal},
    clone,
    element_list::ElementList,
    elements::{
        a,
        button,
        div,
        footer,
        h1,
        header,
        input,
        label,
        li,
        section,
        span,
        strong,
        ul,
        Div,
        Input,
        Li,
        LiBuilder,
        Section,
        Ul,
    },
    mount,
    signal::{ReadSignal, Signal, WriteSignal, ZipSignal},
    Builder,
};
use web_sys::{HtmlDivElement, HtmlInputElement};

#[derive(Clone)]
struct TodoItem {
    id: usize,
    text: Signal<String>,
    completed: Signal<bool>,
    editing: Signal<bool>,
    parent: WriteSignal<ElementList<usize, Self>>,
    active_count: ReadSignal<IncludeSum>,
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
            .map_to(Sum::new(active_count));

        Self {
            id,
            text: Signal::new(text.into()),
            completed,
            editing: Signal::new(false),
            parent,
            active_count,
        }
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

    fn render_edit(&self) -> Input {
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
                    "Escape" => self_.editing.write().set(false),
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

    fn render_view(&self) -> Div {
        let completed_checkbox = input()
            .class("toggle")
            .type_("checkbox")
            .on_click({
                let set_completed = self.completed.write();
                move |_, _| set_completed.replace(|completed| !completed)
            })
            .checked(self.completed.read());
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

    fn render(&self) -> Li {
        li().class(self.class())
            .child(self.render_edit())
            .child(self.render_view())
            .build()
    }
}

#[derive(Display, Copy, Clone, Eq, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

#[derive(Clone)]
struct TodoApp {
    items: Signal<ElementList<usize, TodoItem>>,
    id: Rc<RefCell<usize>>, // RUSTC(cell_update): Replace with `Cell`
    filter: Signal<Filter>,
    active_count: SumTotal<usize>,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            items: Signal::new(ElementList::new(
                ul().class("todo-list"),
                TodoItem::render,
                iter::empty(),
            )),
            id: Rc::new(RefCell::new(0)),
            filter: Signal::new(Filter::All),
            active_count: SumTotal::default(),
        }
    }

    fn push(&self, text: String) {
        let self_ = self.clone();

        self.items.write().mutate(move |ts| {
            let current_id = self_.id.replace_with(|current| *current + 1);
            let parent = self_.items.write();
            ts.insert(
                current_id,
                TodoItem::new(current_id, text, false, parent, &self_.active_count),
            );
        })
    }

    fn render_filter_link(
        &self,
        filter: Filter,
        seperator: &str,
        f: impl 'static + Clone + Fn(&TodoItem) -> ReadSignal<bool>,
    ) -> LiBuilder {
        let set_filter = self.filter.write();
        let write_items = self.items.write();

        li().child(
            a().class(
                self.filter
                    .read()
                    .map(move |f| if filter == *f { "selected" } else { "" }),
            )
            .text(format!("{}", filter))
            .on_click(move |_, _| {
                clone!(f);
                set_filter.set(filter);
                write_items.mutate(|items| items.filter(f))
            }),
        )
        .text(seperator)
    }

    fn render_filters(&self) -> Ul {
        ul().class("filters")
            .child(self.render_filter_link(Filter::All, " ", |_| Signal::new(true).read()))
            .child(self.render_filter_link(Filter::Active, " ", |item| {
                item.completed.read().map(|completed| !completed)
            }))
            .child(self.render_filter_link(Filter::Completed, "", |item| item.completed.read()))
            .build()
    }

    fn render_clear_completed(&self) -> ReadSignal<Div> {
        let write_items = self.items.write();
        let items_len = self.items.read().map(ElementList::len);
        let any_completed = (self.active_count.read(), items_len)
            .map(|&active_count, &items_len| active_count != items_len)
            .only_changes();

        any_completed.map(move |&any_completed| {
            clone!(write_items);

            // TODO(empty elements): Eliminate the outer `div`.
            if any_completed {
                div().child(
                    button()
                        .class("clear-completed")
                        .text("Clear completed")
                        .on_click(move |_, _| {
                            write_items.mutate(|items| {
                                items.retain(|item| !*item.completed.read().current())
                            })
                        }),
                )
            } else {
                div()
            }
            .build()
        })
    }

    fn render_footer(&self) -> ReadSignal<Div> {
        self.items
            .read()
            .map(ElementList::is_empty)
            .only_changes()
            .map({
                let self_ = self.clone();

                move |&is_empty| {
                    // TODO(empty elements): Eliminate the outer `div`.
                    if is_empty {
                        div()
                    } else {
                        div().child(
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
                                .child(self_.render_filters())
                                .child(self_.render_clear_completed()),
                        )
                    }
                    .build()
                }
            })
    }

    fn render(&self) -> Section {
        let is_empty = self.items.read().map(ElementList::is_empty).only_changes();
        let all_complete = self
            .active_count
            .read()
            .map(|&active_count| active_count == 0)
            .only_changes();
        let write_items = self.items.write();
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
                    .child(is_empty.map(move |&is_empty| {
                        if is_empty {
                            div()
                        } else {
                            clone!(all_complete, write_items);

                            div()
                                .child(
                                    input()
                                        .id("toggle-all")
                                        .class("toggle-all")
                                        .type_("checkbox")
                                        .checked(all_complete.clone())
                                        .on_change(move |_, _| {
                                            let new_completed = !*all_complete.current();

                                            write_items.mutate(move |items| {
                                                for item in items.values() {
                                                    item.completed.write().set(new_completed);
                                                }
                                            })
                                        }),
                                )
                                .child(label().for_("toggle-all"))
                        }
                    }))
                    .child(self.items.read()),
            )
            .child(self.render_footer())
            .build()
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount("app", TodoApp::new().render());
}
