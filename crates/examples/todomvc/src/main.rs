use std::iter;

use surfinia_core::{
    hooks::state::{ReadSignal, Signal},
    mount,
    Builder,
};
use surfinia_html::{button, div, element_list, h1, header, input, label, li, section, ul, Li};

struct TodoItem {
    text: String,
    completed: Signal<bool>,
    editing: Signal<bool>,
}

impl TodoItem {
    fn new(text: impl Into<String>, completed: bool) -> Self {
        Self {
            text: text.into(),
            completed: Signal::new(completed),
            editing: Signal::new(false),
        }
    }

    fn render(&self) -> ReadSignal<Li> {
        self.editing.read().map({
            let text = self.text.clone();
            let set_editing = self.editing.write();
            let set_completed = self.completed.write();
            let get_completed = self.completed.read();

            move |&editing| {
                {
                    let item = li().class(get_completed.map(move |&completed| {
                        let mut classes = Vec::new();

                        if completed {
                            classes.push("completed");
                        }

                        if editing {
                            classes.push("editing");
                        }

                        classes.join(" ")
                    }));

                    if editing {
                        // TODO: on_blur and on_keyup(Enter) to finish editing

                        // TODO: Set focus once this is rendered.
                        item.child(input().class("edit").type_("text").value(&text))
                            .on_keyup({
                                let set_editing = set_editing.clone();
                                move |keyup, input| {
                                    if keyup.key() == "Escape" {
                                        set_editing.set(false);
                                    }
                                }
                            })
                    } else {
                        let completed_checkbox = input()
                            .class("toggle")
                            .type_("checkbox")
                            .on_click({
                                let set_completed = set_completed.clone();
                                move |_, _| set_completed.replace(|completed| !completed)
                            })
                            .checked(get_completed.map(|&completed| completed));

                        item.child(
                            div()
                                .class("view")
                                .child(completed_checkbox)
                                .child(label().text(&text).on_dblclick({
                                    let set_editing = set_editing.clone();
                                    move |_, _| set_editing.set(true)
                                }))
                                .child(button().class("destroy")),
                        )
                    }
                }
                .build()
            }
        })
    }
}

fn main() {
    console_error_panic_hook::set_once();
    let list = Signal::new(element_list(
        ul().class("todo-list"),
        TodoItem::render,
        iter::empty(),
    ));
    let list_mut = list.write();

    mount(
        "app",
        section()
            .class("todoapp")
            .child(
                header().child(h1().text("todos")).child(
                    input()
                        .class("new-todo")
                        .placeholder("What needs to be done?")
                        .autofocus(true)
                        .on_keyup(move |keyup, input| {
                            if keyup.key() == "Enter" {
                                list_mut.mutate(move |ts| {
                                    let text = input.value();
                                    let text = text.trim();

                                    if !text.is_empty() {
                                        ts.push(&TodoItem::new(text, false));
                                        input.set_value("");
                                    }
                                })
                            }
                        }),
                ),
            )
            .child(section().class("main").child(list.read())),
    );
}
