use std::rc::Rc;

use futures_signals::{
    signal::{not, Broadcaster, Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use silkenweb::{
    clone,
    elements::{
        a, button, div, footer, h1, header, input, label, li, section, span, strong, ul, Button,
        Div, Footer, Input, Li, LiBuilder, Section, Ul,
    },
    product, signal, Builder, Effects, HtmlElement, ParentBuilder,
};
use web_sys::HtmlInputElement;

use crate::model::{Filter, TodoApp, TodoItem};

#[derive(Clone)]
pub struct TodoAppView {
    app: Rc<TodoApp>,
}

impl TodoAppView {
    pub fn new(app: Rc<TodoApp>) -> Self {
        Self { app }
    }

    pub fn render(&self, item_filter: impl 'static + Signal<Item = Filter>) -> Section {
        let app = &self.app;
        let input_elem = input()
            .class("new-todo")
            .placeholder("What needs to be done?")
            .on_keyup({
                clone!(app);

                move |keyup, input| {
                    if keyup.key() == "Enter" {
                        let text = input.value();
                        let text = text.trim().to_string();

                        if !text.is_empty() {
                            app.new_todo(text);
                            input.set_value("");
                        }
                    }
                }
            })
            .effect(|elem: &HtmlInputElement| elem.focus().unwrap())
            .build();

        let item_filter = Broadcaster::new(item_filter);

        section()
            .class("todoapp")
            .child(header().child(h1().text("todos")).child(input_elem))
            .optional_child_signal(self.render_main(item_filter.signal()))
            .optional_child_signal(self.render_footer(item_filter.signal()))
            .build()
    }

    fn render_main(
        &self,
        item_filter: impl 'static + Signal<Item = Filter>,
    ) -> impl Signal<Item = Option<Section>> {
        let app_view = self.clone();
        // TODO: Store this in a mutable, rather than using a Broadcaster? Where to run
        // the future?
        let item_filter = Broadcaster::new(item_filter);

        self.is_empty().map(move |is_empty| {
            if is_empty {
                None
            } else {
                let app = &app_view.app;

                Some(
                    section()
                        .class("main")
                        .child(
                            input()
                                .id("toggle-all")
                                .class("toggle-all")
                                .type_("checkbox")
                                .on_change({
                                    clone!(app);

                                    move |_, elem| app.set_completed_states(elem.checked())
                                })
                                .effect_signal(app_view.all_completed(), |elem, all_complete| {
                                    elem.set_checked(all_complete)
                                }),
                        )
                        .child(label().for_("toggle-all"))
                        .child(ul().class("todo-list").children_signal(
                            app_view.visible_items_signal(item_filter.signal()).map({
                                clone!(app);
                                move |item| TodoItemView::render(item, app.clone())
                            }),
                        ))
                        .build(),
                )
            }
        })
    }

    fn render_footer(
        &self,
        item_filter: impl 'static + Signal<Item = Filter>,
    ) -> impl Signal<Item = Option<Footer>> {
        let app_view = self.clone();
        let item_filter = Broadcaster::new(item_filter);

        self.is_empty().map({
            clone!(app_view);

            move |is_empty| {
                if is_empty {
                    None
                } else {
                    Some(
                        footer()
                            .class("footer")
                            .child_signal(app_view.active_count().map(move |active_count| {
                                span()
                                    .class("todo-count")
                                    .child(strong().text(&format!("{}", active_count)))
                                    .text(&format!(
                                        " item{} left",
                                        if active_count == 1 { "" } else { "s" }
                                    ))
                            }))
                            .child(app_view.render_filters(item_filter.signal()))
                            .optional_child_signal(app_view.render_clear_completed())
                            .build(),
                    )
                }
            }
        })
    }

    fn render_filter_link(
        &self,
        filter: Filter,
        item_filter: impl 'static + Signal<Item = Filter>,
        seperator: &str,
    ) -> LiBuilder {
        let filter_name = format!("{}", filter);

        li().child(
            a().class(signal(item_filter.map(move |f| {
                if filter == f { "selected" } else { "" }.to_string()
            })))
            .href(format!("/#/{}", filter_name.to_lowercase()))
            .text(&filter_name),
        )
        .text(seperator)
    }

    fn render_filters(&self, item_filter: impl 'static + Signal<Item = Filter>) -> Ul {
        let item_filter = Broadcaster::new(item_filter);
        ul().class("filters")
            .child(self.render_filter_link(Filter::All, item_filter.signal(), " "))
            .child(self.render_filter_link(Filter::Active, item_filter.signal(), " "))
            .child(self.render_filter_link(Filter::Completed, item_filter.signal(), ""))
            .build()
    }

    fn render_clear_completed(&self) -> impl Signal<Item = Option<Button>> {
        let app = self.app.clone();
        self.any_completed().map(move |any_completed| {
            clone!(app);

            if any_completed {
                Some(
                    button()
                        .class("clear-completed")
                        .text("Clear completed")
                        .on_click(move |_, _| app.clear_completed_todos())
                        .build(),
                )
            } else {
                None
            }
        })
    }

    fn visible_items_signal(
        &self,
        item_filter: impl Signal<Item = Filter>,
    ) -> impl SignalVec<Item = Rc<TodoItem>> {
        let item_filter = Broadcaster::new(item_filter);

        self.app.items_signal().filter_signal_cloned(move |item| {
            product!(item.completed(), item_filter.signal()).map(|(completed, item_filter)| {
                match item_filter {
                    Filter::All => true,
                    Filter::Active => !completed,
                    Filter::Completed => completed,
                }
            })
        })
    }

    fn is_empty(&self) -> impl Signal<Item = bool> {
        self.app.items_signal().is_empty().dedupe()
    }

    fn completed(&self) -> impl SignalVec<Item = bool> {
        self.app.items_signal().map_signal(|todo| todo.completed())
    }

    fn all_completed(&self) -> impl Signal<Item = bool> {
        self.completed()
            .filter(|completed| !completed)
            .is_empty()
            .dedupe()
    }

    fn any_completed(&self) -> impl Signal<Item = bool> {
        not(self
            .completed()
            .filter(|completed| *completed)
            .is_empty()
            .dedupe())
    }

    fn active_count(&self) -> impl Signal<Item = usize> {
        self.completed().filter(|completed| !completed).len()
    }
}

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
