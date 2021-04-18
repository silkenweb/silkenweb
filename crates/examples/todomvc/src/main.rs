use std::iter;

use surfinia_core::{
    hooks::{
        effect,
        state::{ReadSignal, Signal, ZipSignal},
    },
    mount,
    Builder,
    DomElement,
};
use surfinia_html::{
    button,
    div,
    element_list,
    h1,
    header,
    input,
    label,
    li,
    section,
    ul,
    Div,
    Input,
    Li,
};
use web_sys::HtmlInputElement;

#[derive(Clone)]
struct TodoItem {
    text: Signal<String>,
    completed: Signal<bool>,
    editing: Signal<bool>,
}

impl TodoItem {
    fn new(text: impl Into<String>, completed: bool) -> Self {
        Self {
            text: Signal::new(text.into()),
            completed: Signal::new(completed),
            editing: Signal::new(false),
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
                move |keyup, input| {
                    let key = keyup.key();

                    if key == "Escape" {
                        this.editing.write().set(false);
                    } else if key == "Enter" {
                        this.save_edits(&input);
                    }
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

        div()
            .class("view")
            .child(completed_checkbox)
            .child(label().text(self.text.read()).on_dblclick({
                let set_editing = self.editing.write();
                move |_, _| set_editing.set(true)
            }))
            .child(button().class("destroy"))
            .build()
    }

    fn render(&self) -> ReadSignal<Li> {
        let this = self.clone();
        let class = this.class();

        self.editing.read().map(move |&editing| {
            let item = li().class(class.clone());

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
