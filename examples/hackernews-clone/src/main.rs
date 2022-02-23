use std::time::Duration;

use chrono::{DateTime, Utc};
use futures::future::join_all;
use futures_signals::signal::Mutable;
use reqwasm::http::Request;
use serde::{de::DeserializeOwned, Deserialize};
use silkenweb::{
    elements::html::{a, div, p, span, table, td, tr, Div, Tr},
    macros::{Element, ElementBuilder},
    mount,
    prelude::ParentBuilder,
    task::spawn_local,
};
use timeago::Formatter;

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

    async fn try_load_frontpage(&self) -> Result<Vec<Story>, reqwasm::Error> {
        let top_stories: Vec<u64> = query_api("topstories").await?;

        let stories = top_stories
            .into_iter()
            .take(30)
            .map(query_api_item::<Story>);

        Ok(join_all(stories)
            .await
            .into_iter()
            .filter_map(|story| story.ok())
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
    FrontPage(Vec<Story>),
    Error(String),
}

impl Content {
    fn render(&self) -> Element {
        match self {
            Content::FrontPage(articles) => table()
                .children(
                    articles
                        .iter()
                        .enumerate()
                        .flat_map(|(index, article)| article.render(index + 1).into_iter()),
                )
                .build()
                .into(),
            Content::Error(err) => p().text(err).build().into(),
        }
    }
}

#[derive(Deserialize)]
struct Story {
    // id 	The item's unique id.
    // deleted 	true if the item is deleted.
    // type 	The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    // by 	The username of the item's author.
    // time 	Creation date of the item, in Unix Time.
    // text 	The comment, story or poll text. HTML.
    // dead 	true if the item is dead.
    // parent 	The comment's parent: either another comment or the relevant story.
    // poll 	The pollopt's associated poll.
    // kids 	The ids of the item's comments, in ranked display order.
    // url 	The URL of the story.
    // score 	The story's score, or the votes for a pollopt.
    // title 	The title of the story, poll or job. HTML.
    // parts 	A list of related pollopts, in display order.
    // descendants 	In the case of stories or polls, the total
    title: String,
    #[serde(default)]
    score: u64,
    #[serde(default)]
    by: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    time: DateTime<Utc>,
    #[serde(default)]
    descendants: u64,
}

impl Story {
    fn render(&self, index: usize) -> [Tr; 2] {
        let score = self.score;
        let descendants = self.descendants;
        let time_ago = Formatter::new()
            .num_items(1)
            .convert((Utc::now() - self.time).to_std().unwrap_or(Duration::ZERO));

        [
            tr().child(td().text(&format!("{index}.")))
                .child(td().text(&self.title))
                .build(),
            tr().child(td())
                .child(
                    td().child(span().text(&format!("{score} point{}", plural(score))))
                        .text(" by ")
                        .child(a().text(&self.by))
                        .text(&format!(" {time_ago} | "))
                        .child(a().text(&format!("{descendants} comment{}", plural(descendants)))),
                )
                .build(),
        ]
    }
}

async fn query_api_item<T: DeserializeOwned>(id: u64) -> Result<T, reqwasm::Error> {
    query_api(&format!("item/{id}")).await
}

async fn query_api<T: DeserializeOwned>(path: &str) -> Result<T, reqwasm::Error> {
    Request::get(&format!(
        "https://hacker-news.firebaseio.com/v0/{path}.json"
    ))
    .send()
    .await?
    .json()
    .await
}

fn plural(count: u64) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}

fn main() {
    let app = App::new();

    spawn_local(app.clone().load_frontpage());
    mount("app", app.render());
}
