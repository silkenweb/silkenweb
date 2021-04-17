use std::iter;

use surfinia_core::{hooks::state::Signal, mount, Builder};
use surfinia_html::{button, div, element_list, h1, header, input, label, li, section, ul, Li};
use wasm_bindgen::JsCast;
use web_sys as dom;

struct TodoItem {
    text: String,
    completed: Signal<bool>,
}

impl TodoItem {
    fn new(text: impl Into<String>, completed: bool) -> Self {
        Self {
            text: text.into(),
            completed: Signal::new(completed),
        }
    }

    fn render(&self) -> Li {
        let text = self.text.clone();

        let completed_checkbox = input()
            .class("toggle")
            .type_("checkbox")
            .on_click({
                let set_completed = self.completed.write();
                move |_| set_completed.replace(|completed| !completed)
            })
            .checked(self.completed.read().map(|&completed| completed));

        li().class(
            self.completed
                .read()
                .map(|&completed| if completed { "completed" } else { "" }),
        )
        .child(
            div()
                .class("view")
                .child(completed_checkbox)
                .child(label().text(&text))
                .child(button().class("destroy")),
        )
        .child(input().class("edit").value(&text))
        .build()
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
                        .on_keyup(move |keyup| {
                            if keyup.key() == "Enter" {
                                list_mut.mutate(move |ts| {
                                    let target = keyup.target().unwrap();
                                    // TODO: Wrap event type and provide pre-cast target()
                                    let input: dom::HtmlInputElement = target.dyn_into().unwrap();
                                    let text = input.value();
                                    let text = text.trim();

                                    if !text.is_empty() {
                                        ts.push(&TodoItem::new(text, false));
                                    }

                                    // TODO: Clear value here or inside `if`?
                                    input.set_value("");
                                })
                            }
                        }),
                ),
            )
            .child(section().class("main").child(list.read())),
    );
}
