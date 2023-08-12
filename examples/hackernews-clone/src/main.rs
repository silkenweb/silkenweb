use std::{fmt::Display, result, time::Duration};

use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use futures::{future::join_all, Future};
use futures_signals::signal::{Mutable, SignalExt};
use reqwasm::http::Request;
use serde::{de::DeserializeOwned, Deserialize};
use silkenweb::{
    clone, css,
    elements::html::{self, a, div, h1, h2, header, li, nav, p, span, ul, Div, Li, A},
    log_panics, mount,
    node::element::{Element, GenericElement, ParentElement},
    router,
    task::spawn_local,
    value::Sig,
};
use timeago::Formatter;

css!(path = "hackernews.css", validate);

use class::*;

type Result<T> = result::Result<T, reqwasm::Error>;

#[derive(Clone)]
struct App(Mutable<Option<Content>>);

impl App {
    fn new() -> Self {
        Self(Mutable::new(None))
    }

    fn set_loading(&self) {
        self.0.set(None)
    }

    fn set_content(&self, content: Content) {
        self.0.set(Some(content))
    }

    fn render(&self) -> Div {
        div()
            .class(PAGE)
            .child(
                header().child(nav().child(h1().class(PAGE_BANNER).children([
                    router::anchor("topstories").text("Top"),
                    router::anchor("newstories").text("New"),
                    router::anchor("askstories").text("Ask"),
                    router::anchor("showstories").text("Show"),
                ]))),
            )
            .child(
                html::main()
                    .class(PAGE_CONTENT)
                    .child(Sig(self.0.signal_ref(|content| {
                        if let Some(content) = content {
                            content.render()
                        } else {
                            p().text("Loading...").into()
                        }
                    }))),
            )
    }
}

enum Content {
    FrontPage(Vec<Story>),
    Story(StoryDetail),
    User(UserDetails),
    Unknown,
    Error(String),
}

impl Content {
    async fn load_frontpage(story_type: &str) -> Self {
        async {
            let top_stories: Vec<u64> = query(story_type).await?;
            let stories = top_stories.into_iter().take(STORY_COUNT).map(query_item);

            Ok(Self::FrontPage(join_ok(stories).await))
        }
        .await
        .unwrap_or_else(Self::from_error)
    }

    async fn load_story(id: &str) -> Self {
        async {
            let story: Story = query_item(id).await?;
            let comments = CommentTree::load_vec(&story.kids, 3).await;

            Ok(Self::Story(StoryDetail { story, comments }))
        }
        .await
        .unwrap_or_else(Self::from_error)
    }

    async fn load_user(id: &str) -> Self {
        async {
            let user = query_user(id).await?;
            let submitted = join_ok(user.submitted.iter().take(STORY_COUNT).map(query_item)).await;
            Ok(Self::User(UserDetails { user, submitted }))
        }
        .await
        .unwrap_or_else(Self::from_error)
    }

    fn from_error(err: reqwasm::Error) -> Self {
        Self::Error(err.to_string())
    }

    fn render(&self) -> GenericElement {
        match self {
            Content::FrontPage(articles) => ul()
                .children(articles.iter().map(|article| li().child(article.render())))
                .into(),
            Content::Story(story) => story.render().into(),
            Content::User(user) => user.render().into(),
            Content::Unknown => p().text("Unknown").into(),
            Content::Error(err) => p().text(err).into(),
        }
    }
}

#[derive(Deserialize)]
struct Story {
    id: u64,
    title: String,
    #[serde(default)]
    score: u64,
    #[serde(default)]
    by: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    time: DateTime<Utc>,
    #[serde(default)]
    descendants: u64,
    #[serde(default)]
    text: String,
    #[serde(default)]
    kids: Vec<u64>,
    url: Option<String>,
}

impl Story {
    fn render(&self) -> Div {
        let score = self.score;
        let descendants = self.descendants;
        let time_ago = time_ago(self.time);
        let id = self.id;
        let url_path = format!("item/{id}");

        div()
            .child(
                h2().class(STORY_TITLE)
                    .child(a().href(self.url.as_ref()).text(&self.title)),
            )
            .child(
                span()
                    .class(DE_EMPHASIZE)
                    .child(span().text(format!("{score} point{} by ", plural(score))))
                    .child(user_link(&self.by))
                    .child(span().text(format!(" {time_ago} | ")))
                    .child(
                        router::anchor(url_path)
                            .text(format!("{descendants} comment{}", plural(descendants))),
                    ),
            )
    }
}

#[derive(Deserialize)]
struct Comment {
    #[serde(default)]
    by: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    time: DateTime<Utc>,
    #[serde(default)]
    text: String,
    #[serde(default)]
    kids: Vec<u64>,
}

#[derive(Deserialize)]
struct User {
    id: String,
    #[serde(default)]
    about: String,
    #[serde(default)]
    karma: u64,
    #[serde(default)]
    submitted: Vec<u64>,
}

struct UserDetails {
    user: User,
    submitted: Vec<Story>,
}

impl UserDetails {
    fn render(&self) -> Div {
        div()
            .child(h2().text(&self.user.id))
            .child(text_as_html(&self.user.about))
            .child(span().text(format!("{} karma", self.user.karma)))
            .child(
                ul().class(USER_SUBMITTED_STORIES)
                    .children(self.submitted.iter().map(Story::render)),
            )
    }
}

struct StoryDetail {
    story: Story,
    comments: Vec<CommentTree>,
}

impl StoryDetail {
    fn render(&self) -> Div {
        div()
            .child(self.story.render())
            .child(text_as_html(&self.story.text))
            .child(ul().children(self.comments.iter().map(|comment| comment.render())))
    }
}

struct CommentTree {
    comment: Comment,
    children: Vec<CommentTree>,
}

impl CommentTree {
    #[async_recursion(?Send)]
    async fn load_vec(ids: &[u64], depth: usize) -> Vec<Self> {
        join_ok(ids.iter().copied().map(|id| Self::load(id, depth))).await
    }

    #[async_recursion(?Send)]
    async fn load(id: u64, depth: usize) -> Result<Self> {
        let comment: Comment = query_item(id).await?;
        let children = if depth > 0 {
            Self::load_vec(&comment.kids, depth - 1).await
        } else {
            Vec::new()
        };

        Ok(Self { comment, children })
    }

    fn render(&self) -> Li {
        let time_ago = time_ago(self.comment.time);
        li().class(COMMENT)
            .child(
                span()
                    .class(DE_EMPHASIZE)
                    .child(user_link(&self.comment.by))
                    .text(format!(" {time_ago}")),
            )
            .child(text_as_html(&self.comment.text))
            .child(ul().children(self.children.iter().map(move |child| child.render())))
    }
}

fn text_as_html(text: &str) -> Div {
    let text = text.to_owned();
    div()
        .class(USER_CONTENT)
        .effect(move |elem| elem.set_inner_html(&text))
}

fn user_link(user: &str) -> A {
    router::anchor(format!("user/{user}")).text(user)
}

async fn join_ok<T>(items: impl IntoIterator<Item = impl Future<Output = Result<T>>>) -> Vec<T> {
    join_all(items)
        .await
        .into_iter()
        .filter_map(Result::ok)
        .collect()
}

async fn query_item<T: DeserializeOwned>(id: impl Display) -> Result<T> {
    query(&format!("item/{id}")).await
}

async fn query_user(id: impl Display) -> Result<User> {
    query(&format!("user/{id}")).await
}

async fn query<T: DeserializeOwned>(path: &str) -> Result<T> {
    let url = format!("https://hacker-news.firebaseio.com/v0/{path}.json");
    Request::get(&url).send().await?.json().await
}

fn time_ago(time: DateTime<Utc>) -> String {
    Formatter::new()
        .num_items(1)
        .convert((Utc::now() - time).to_std().unwrap_or(Duration::ZERO))
}

fn plural(count: u64) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}

fn main() {
    log_panics();

    let app = App::new();

    spawn_local(router::url_path().signal_cloned().for_each({
        clone!(app);
        move |pathname| {
            clone!(app);
            async move {
                app.set_loading();

                app.set_content(match pathname.as_str() {
                    "" => Content::load_frontpage("topstories").await,
                    "topstories" | "newstories" | "askstories" | "showstories" => {
                        Content::load_frontpage(pathname.as_str()).await
                    }
                    item => match *item.split(['/']).collect::<Vec<_>>() {
                        ["item", id] => Content::load_story(id).await,
                        ["user", id] => Content::load_user(id).await,
                        _ => Content::Unknown,
                    },
                })
            }
        }
    }));

    mount("app", app.render());
}

const STORY_COUNT: usize = 30;
