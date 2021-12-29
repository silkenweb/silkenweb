use std::rc::Rc;

use futures_signals::signal::{Signal, SignalExt};
use silkenweb::{
    clone,
    elements::{button, div, input, label, li, Div, Input, Li},
    product, signal, Builder, Effects, HtmlElement, ParentBuilder,
};

use crate::model::{TodoApp, TodoItem};

pub struct TodoItemView {
    todo: Rc<TodoItem>,
    app: Rc<TodoApp>,
}

impl TodoItemView {
    pub fn render(todo: Rc<TodoItem>, app: Rc<TodoApp>) -> Li {
        let view = TodoItemView { todo, app };
        li().class(signal(view.class()))
            .child(view.render_edit())
            .child(view.render_view())
            .build()
    }

    fn render_edit(&self) -> Input {
        let todo = &self.todo;
        let app = &self.app;

        input()
            .class("edit")
            .type_("text")
            .value(signal(todo.text()))
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
                    elem.focus().unwrap();
                }
            })
            .build()
    }

    fn render_view(&self) -> Div {
        let todo = &self.todo;
        let app = &self.app;
        let completed_checkbox = input()
            .class("toggle")
            .type_("checkbox")
            .on_click({
                clone!(todo, app);
                move |_, elem| app.set_completed(&todo, elem.checked())
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
                move |_, _| todo.set_editing()
            }))
            .child(button().class("destroy").on_click({
                clone!(todo, app);
                move |_, _| todo.remove(&app)
            }))
            .effect_signal(todo.is_editing(), |elem, editing| elem.set_hidden(editing))
            .build()
    }

    fn class(&self) -> impl Signal<Item = String> {
        let todo = &self.todo;
        product!(todo.completed(), todo.is_editing()).map(|(completed, editing)| {
            vec![(completed, "completed"), (editing, "editing")]
                .into_iter()
                .filter_map(|(flag, name)| if flag { Some(name) } else { None })
                .collect::<Vec<_>>()
                .join(" ")
        })
    }
}
