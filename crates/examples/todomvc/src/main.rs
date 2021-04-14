use surfinia::{
    button,
    div,
    h1,
    header,
    input,
    label,
    li,
    mount,
    section,
    use_list_state,
    Builder,
    ElementBuilder,
    Li,
};

struct TodoItem {
    text: String,
}

impl TodoItem {
    fn render(&self) -> Li {
        li().child(
            div()
                .class("view")
                .child(input().class("toggle").type_("checkbox"))
                .child(label().text(&self.text))
                .child(button().class("destroy")),
        )
        .child(input().class("edit").value(&self.text))
        .build()
    }
}

fn main() {
    console_error_panic_hook::set_once();
    let (list, _list_mut) = use_list_state(
        ElementBuilder::new("ul").attribute("class", "todo-list"),
        vec![TodoItem {
            text: "test".to_string(),
        }]
        .into_iter(),
    );

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
            .child(section().class("main").child(list.with(TodoItem::render))),
    );
}
