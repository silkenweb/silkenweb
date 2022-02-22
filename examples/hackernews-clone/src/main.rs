use futures_signals::signal::Mutable;
use reqwasm::http::Request;
use silkenweb::{
    elements::html::{div, p, table, td, tr, Div, Tr},
    macros::{Element, ElementBuilder},
    mount,
    prelude::ParentBuilder,
    task::spawn_local,
};

#[derive(Clone)]
struct App(Mutable<Option<Content>>);

impl App {
    fn new() -> Self {
        Self(Mutable::new(None))
    }

    async fn load_frontpage(self) {
        let content = match self.try_load_frontpage().await {
            Ok(articles) => Content::FrontPage(articles),
            Err(err) => Content::Error(err.to_string()),
        };

        self.0.set(Some(content))
    }

    async fn try_load_frontpage(&self) -> Result<Vec<Article>, reqwasm::Error> {
        let top_stories: Vec<u64> =
            Request::get("https://hacker-news.firebaseio.com/v0/topstories.json")
                .send()
                .await?
                .json()
                .await?;

        Ok(top_stories
            .into_iter()
            .take(30)
            .map(|id| Article { id })
            .collect())
    }

    fn render(&self) -> Div {
        div().child_signal(self.0.signal_ref(|content| {
            if let Some(content) = content {
                content.render()
            } else {
                p().text("Loading...").build().into()
            }
        }))
    }
}

enum Content {
    FrontPage(Vec<Article>),
    Error(String),
}

impl Content {
    fn render(&self) -> Element {
        match self {
            Content::FrontPage(articles) => table()
                .children(articles.iter().map(Article::render).collect::<Vec<_>>())
                .build()
                .into(),
            Content::Error(err) => p().text(err).build().into(),
        }
    }
}

struct Article {
    id: u64,
}

impl Article {
    fn render(&self) -> Tr {
        tr().child(td().text(&format!("{}", self.id))).build()
    }
}

fn main() {
    let app = App::new();

    spawn_local(app.clone().load_frontpage());
    mount("app", app.render());
}
