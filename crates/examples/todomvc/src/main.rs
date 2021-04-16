use surfinia_core::{
    hooks::state::{use_state, GetState, SetState},
    mount,
    Builder,
};
use surfinia_html::{button, div, element_list, h1, header, input, label, li, section, ul, Li};

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

    fn render(&self) -> Li {
        let text = self.text.clone();

        let mut li = li();
        let completed_checkbox = input()
            .class("toggle")
            .type_("checkbox")
            .on_click({
                let set_completed = self.set_completed.clone();
                move || set_completed.map(|completed| !completed)
            })
            .checked(self.completed.with(|&completed| completed));

        li = li.class(
            self.completed
                .with(|&completed| if completed { "completed" } else { "" }.to_owned()),
        );

        // TODO: Set the checked status of the checkbox.

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
}

fn main() {
    console_error_panic_hook::set_once();
    let (list, _list_mut) = use_state(element_list(
        ul().class("todo-list"),
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
                        .autofocus(true),
                ),
            )
            .child(section().class("main").child(list)),
    );
}
