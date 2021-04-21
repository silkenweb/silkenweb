#[macro_use]
extern crate derive_more;

use std::{cell::RefCell, iter, rc::Rc};

use surfinia_core::{
    hooks::{
        effect,
        list_state::ElementList,
        state::{ReadSignal, Signal, WriteSignal, ZipSignal},
    },
    mount,
    Builder,
    DomElement,
};
use surfinia_html::{
    a,
    button,
    div,
    element_list,
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
};
use web_sys::HtmlInputElement;

#[derive(Clone)]
struct TodoItem {
    id: usize,
    text: Signal<String>,
    completed: Signal<bool>,
    editing: Signal<bool>,
    parent: WriteSignal<ElementList<usize, Self>>,
}

impl TodoItem {
    fn new(
        id: usize,
        text: impl Into<String>,
        completed: bool,
        parent: WriteSignal<ElementList<usize, Self>>,
    ) -> Self {
        Self {
            id,
            text: Signal::new(text.into()),
            completed: Signal::new(completed),
            editing: Signal::new(false),
            parent,
        }
    }

    fn save_edits(&self, input: &HtmlInputElement) {
        let text = input.value();
        let text = text.trim();

        if !text.is_empty() {
            self.text.write().set(text.to_string());
            self.editing.write().set(false);
        }
    }

    fn class(&self) -> ReadSignal<String> {
        (self.completed.read(), self.editing.read()).map(move |&completed, &editing| {
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
                let this = self.clone();
                move |_, input| this.save_edits(&input)
            })
            .on_keyup({
                let this = self.clone();
                move |keyup, input| match keyup.key().as_str() {
                    "Escape" => this.editing.write().set(false),
                    "Enter" => this.save_edits(&input),
                    _ => (),
                }
            })
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
            .checked(self.completed.read().map(|&completed| completed));
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
            .build()
    }

    fn render(&self) -> ReadSignal<Li> {
        let this = self.clone();
        
        self.editing.read().map(move |&editing| {
            let item = li().class(this.class());

            if editing {
                let input = this.render_edit();
                let dom_elem = input.dom_element();

                effect(move || dom_elem.focus().unwrap());
                item.child(input)
            } else {
                item.child(this.render_view())
            }
            .build()
        })
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
    id: Rc<RefCell<usize>>, // TODO: Cell
    filter: Signal<Filter>,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            items: Signal::new(element_list(
                ul().class("todo-list"),
                TodoItem::render,
                iter::empty(),
            )),
            id: Rc::new(RefCell::new(0)),
            filter: Signal::new(Filter::All),
        }
    }

    fn push(&self, text: String) {
        let this = self.clone();

        self.items.write().mutate(move |ts| {
            let current_id = this.id.replace_with(|current| *current + 1);
            let parent = this.items.write();
            ts.insert(current_id, TodoItem::new(current_id, text, false, parent));
        })
    }

    fn render_filter_link(
        current_filter: &Signal<Filter>,
        write_items: WriteSignal<ElementList<usize, TodoItem>>,
        filter: Filter,
        seperator: &str,
        f: impl 'static + Clone + Fn(&TodoItem) -> ReadSignal<bool>,
    ) -> LiBuilder {
        let set_filter = current_filter.write();
        li().child(
            a().class(
                current_filter
                    .read()
                    .map(move |f| if filter == *f { "selected" } else { "" }),
            )
            .text(format!("{}", filter))
            .on_click(move |_, _| {
                let write_items = write_items.clone();
                let f = f.clone();
                set_filter.set(filter);
                write_items.mutate(move |items| items.filter(f))
            }),
        )
        .text(seperator)
    }

    fn render_filters(
        current: &Signal<Filter>,
        write_items: WriteSignal<ElementList<usize, TodoItem>>,
    ) -> Ul {
        ul().class("filters")
            .child(Self::render_filter_link(
                &current,
                write_items.clone(),
                Filter::All,
                " ",
                |_| Signal::new(true).read(),
            ))
            .child(Self::render_filter_link(
                &current,
                write_items.clone(),
                Filter::Active,
                " ",
                |item| item.completed.read().map(|completed| !completed),
            ))
            .child(Self::render_filter_link(
                &current,
                write_items,
                Filter::Completed,
                "",
                |item| item.completed.read(),
            ))
            .build()
    }

    fn render_footer(&self) -> ReadSignal<Div> {
        let write_items = self.items.write();

        self.items.read().map({
            let current_filter = self.filter.clone();

            move |l| {
                // TODO: We could do with the concept of an empty element, rather than using div
                // here.
                let mut footer_div = div();

                if !l.is_empty() {
                    let len = l.len(); // TODO: Exclude completed
                    let write_items = write_items.clone();

                    footer_div = footer_div.child(
                        footer()
                            .class("footer")
                            .child(span().class("todo-count").child(strong().text(format!(
                                "{} item{} left",
                                len,
                                if len == 1 { "" } else { "s" }
                            ))))
                            .child(Self::render_filters(&current_filter, write_items)),
                    )
                }

                footer_div.build()
            }
        })
    }

    fn render(&self) -> Section {
        section()
            .class("todoapp")
            .child(
                header().child(h1().text("todos")).child(
                    input()
                        .class("new-todo")
                        .placeholder("What needs to be done?")
                        .autofocus(true)
                        .on_keyup({
                            let this = self.clone();

                            move |keyup, input| {
                                if keyup.key() == "Enter" {
                                    let this = this.clone();
                                    let text = input.value();
                                    let text = text.trim().to_string();

                                    if !text.is_empty() {
                                        this.push(text);
                                        input.set_value("");
                                    }
                                }
                            }
                        }),
                ),
            )
            .child(section().class("main").child(self.items.read()))
            .child(self.render_footer())
            .build()
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount("app", TodoApp::new().render());
}
