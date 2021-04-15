use surfinia_core::{
    hooks::{
        list_state::ElementList,
        state::{use_state, GetState, SetState},
    },
    mount,
    Builder,
    ElementBuilder,
};
use surfinia_html::{button, div, h1, header, input, label, li, section, Li};

struct TodoItem {
    text: String,
    completed: GetState<bool>,
    set_completed: SetState<bool>,
}

impl TodoItem {
    fn new(text: impl Into<String>, completed: bool) -> Self {
        let (completed, set_completed) = use_state(completed);

        Self {
            text: text.into(),
            completed,
            set_completed,
        }
    }

    fn render(&self) -> GetState<Li> {
        self.completed.with({
            let text = self.text.clone();
            let set_completed = self.set_completed.clone();

            move |&completed| {
                let mut li = li();
                let mut completed_checkbox = input().class("toggle").type_("checkbox").on_click({
                    let set_completed = set_completed.clone();
                    move || set_completed.set(!completed)
                });

                if completed {
                    li = li.class("completed");
                    completed_checkbox = completed_checkbox.checked();
                }

                li.child(
                    div()
                        .class("view")
                        .child(completed_checkbox)
                        .child(label().text(&text))
                        .child(button().class("destroy")),
                )
                .child(input().class("edit").value(&text))
                .build()
            }
        })
    }
}

fn main() {
    console_error_panic_hook::set_once();
    let (list, _list_mut) = use_state(ElementList::new(
        ElementBuilder::new("ul").attribute("class", "todo-list"),
        TodoItem::render,
        [
            TodoItem::new("Test 1", false),
            TodoItem::new("Test 2", false),
            TodoItem::new("Test 3", true),
        ]
        .iter(),
    ));

    mount(
        "app",
        section()
            .class("todoapp")
            .child(
                header().child(h1().text("todos")).child(
                    input()
                        .class("new-todo")
                        .placeholder("What needs to be done?")
                        .autofocus(),
                ),
            )
            .child(section().class("main").child(list)),
    );
}
